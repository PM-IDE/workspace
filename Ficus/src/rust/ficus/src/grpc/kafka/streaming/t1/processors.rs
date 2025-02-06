use crate::event_log::bxes::bxes_to_xes_converter::read_bxes_events;
use crate::event_log::core::event::event::EventPayloadValue;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::event_log::xes::xes_trace::XesTraceImpl;
use crate::grpc::events::events_handler::CaseName;
use crate::grpc::kafka::models::{
    LogUpdateResult, XesFromBxesKafkaTraceCreatingError, KAFKA_CASE_DISPLAY_NAME, KAFKA_CASE_NAME_PARTS, KAFKA_PROCESS_NAME, KAFKA_TRACE_ID,
};
use crate::grpc::kafka::streaming::processors::ExtractedTraceMetadata;
use crate::grpc::kafka::streaming::t1::filterers::T1LogFilterer;
use crate::grpc::logs_handler::ConsoleLogMessageHandler;
use crate::pipelines::context::{LogMessageHandler, PipelineContext};
use crate::pipelines::keys::context_keys::{CASE_NAME, EVENT_LOG_KEY, PROCESS_NAME, UNSTRUCTURED_METADATA};
use crate::utils::user_data::user_data::UserData;
use bxes::models::domain::bxes_value::BxesValue;
use bxes_kafka::consumer::bxes_kafka_consumer::BxesKafkaTrace;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone)]
pub struct T1StreamingProcessor {
    logger: ConsoleLogMessageHandler,
    names_to_logs: Arc<Mutex<HashMap<String, XesEventLogImpl>>>,
    filterer: T1LogFilterer,
}

impl T1StreamingProcessor {
    pub fn new(filterer: T1LogFilterer) -> Self {
        Self {
            logger: ConsoleLogMessageHandler::new(),
            names_to_logs: Arc::new(Mutex::new(HashMap::new())),
            filterer,
        }
    }

    pub fn observe(&self, trace: BxesKafkaTrace, context: &mut PipelineContext) -> Result<(), XesFromBxesKafkaTraceCreatingError> {
        match self.update_log(trace) {
            Ok(update_result) => {
                context.put_concrete(EVENT_LOG_KEY.key(), update_result.new_log);
                context.put_concrete(PROCESS_NAME.key(), update_result.process_name);
                context.put_concrete(CASE_NAME.key(), update_result.case_name);
                context.put_concrete(UNSTRUCTURED_METADATA.key(), update_result.unstructured_metadata);

                Ok(())
            }
            Err(err) => {
                let message = format!("Failed to get update result, err: {}", err.to_string());
                self.logger.handle(message.as_str()).expect("Must log message");
                Err(err)
            }
        }
    }
}

impl T1StreamingProcessor {
    fn update_log(&self, trace: BxesKafkaTrace) -> Result<LogUpdateResult, XesFromBxesKafkaTraceCreatingError> {
        let metadata = ExtractedTraceMetadata::create_from(trace.metadata())?;

        let result = LogUpdateResult {
            process_name: metadata.process.process_name,
            case_name: CaseName {
                display_name: metadata.case.case_display_name,
                name_parts: metadata.case.case_name_parts,
            },
            new_log: self.get_or_create_event_log(&trace, metadata.case.case_id, metadata.case.case_name_parts_joined.as_str())?,
            unstructured_metadata: metadata.unstructured_metadata
        };

        Ok(result)
    }

    fn get_or_create_event_log(
        &self,
        trace: &BxesKafkaTrace,
        trace_id: Uuid,
        case_key: &str,
    ) -> Result<XesEventLogImpl, XesFromBxesKafkaTraceCreatingError> {
        let mut names_to_logs = self.names_to_logs.lock();
        let names_to_logs = match names_to_logs.as_mut() {
            Ok(names_to_logs) => names_to_logs,
            Err(_) => panic!("Failed to acquire a names_to_logs map from mutex"),
        };

        if !names_to_logs.contains_key(case_key) {
            let new_log = XesEventLogImpl::empty();
            names_to_logs.insert(case_key.to_owned(), new_log);
        }

        let existing_log = names_to_logs.get_mut(case_key).expect("Log should be present");

        self.filterer.filter(existing_log);

        let mut read_xes_trace = match read_bxes_events(trace.events()) {
            Ok(xes_trace) => xes_trace,
            Err(err) => return Err(XesFromBxesKafkaTraceCreatingError::BxesToXexConversionError(err)),
        };

        for existing_xes_trace in existing_log.traces() {
            let mut existing_xes_trace = existing_xes_trace.borrow_mut();
            if let Some(current_trace_id) = Self::try_get_trace_id(&existing_xes_trace).clone() {
                if current_trace_id == trace_id {
                    for event in read_xes_trace.events() {
                        existing_xes_trace.push(event.clone());
                    }

                    drop(existing_xes_trace);
                    return Ok(existing_log.clone());
                }
            }
        }

        read_xes_trace
            .metadata_mut()
            .insert(KAFKA_TRACE_ID.to_owned(), EventPayloadValue::Guid(trace_id));

        let read_xes_trace = Rc::new(RefCell::new(read_xes_trace));
        existing_log.push(read_xes_trace);

        Ok(existing_log.clone())
    }

    fn try_get_trace_id(trace: &XesTraceImpl) -> Option<Uuid> {
        if let Some(EventPayloadValue::Guid(id)) = trace.metadata().get(KAFKA_TRACE_ID) {
            Some(id.clone())
        } else {
            None
        }
    }
}
