use crate::event_log::bxes::bxes_to_xes_converter::read_bxes_events;
use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::grpc::events::events_handler::CaseName;
use crate::grpc::kafka::models::{
    LogUpdateResult, XesFromBxesKafkaTraceCreatingError, KAFKA_CASE_DISPLAY_NAME, KAFKA_CASE_NAME_PARTS, KAFKA_CASE_NAME_PARTS_SEPARATOR,
    KAFKA_PROCESS_NAME,
};
use crate::grpc::kafka::streaming::configs::{EventsTimeoutConfiguration, TracesTimeoutConfiguration};
use crate::grpc::logs_handler::ConsoleLogMessageHandler;
use crate::pipelines::context::{LogMessageHandler, PipelineContext};
use crate::pipelines::keys::context_keys::{CASE_NAME, EVENT_LOG_KEY, PROCESS_NAME, UNSTRUCTURED_METADATA};
use crate::utils::user_data::user_data::UserData;
use bxes::models::domain::bxes_value::BxesValue;
use bxes_kafka::consumer::bxes_kafka_consumer::BxesKafkaTrace;
use chrono::Utc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Sub;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub(in crate::grpc) enum TracesProcessor {
    T1(T1StreamingProcessor),
}

impl TracesProcessor {
    pub fn observe(&self, trace: BxesKafkaTrace, context: &mut PipelineContext) -> Result<(), XesFromBxesKafkaTraceCreatingError> {
        match self {
            TracesProcessor::T1(default) => default.observe(trace, context),
        }
    }
}

#[derive(Clone)]
pub(in crate::grpc) enum T1LogFilterer {
    None,
    EventsTimeoutFilterer(EventsTimeoutFiltererImpl),
    TracesTimeoutFilterer(TracesTimeoutFiltererImpl),
}

impl T1LogFilterer {
    pub fn filter(&self, log: &mut XesEventLogImpl) {
        match self {
            T1LogFilterer::None => {}
            T1LogFilterer::EventsTimeoutFilterer(filterer) => filterer.filter(log),
            T1LogFilterer::TracesTimeoutFilterer(filterer) => filterer.filter(log),
        }
    }
}

#[derive(Clone)]
pub(in crate::grpc) struct EventsTimeoutFiltererImpl {
    config: EventsTimeoutConfiguration,
}

impl EventsTimeoutFiltererImpl {
    pub fn new(config: EventsTimeoutConfiguration) -> Self {
        Self { config }
    }

    pub fn filter(&self, log: &mut XesEventLogImpl) {
        let current_stamp = Utc::now();
        let timeout = self.config.timeout_ms() as i64;
        log.filter_events_by(|e| e.timestamp().sub(current_stamp).num_milliseconds() > timeout);
    }
}

#[derive(Clone)]
pub(in crate::grpc) struct TracesTimeoutFiltererImpl {
    config: TracesTimeoutConfiguration,
}

impl TracesTimeoutFiltererImpl {
    pub fn new(config: TracesTimeoutConfiguration) -> Self {
        Self { config }
    }

    pub fn filter(&self, log: &mut XesEventLogImpl) {
        let current_stamp = Utc::now();
        let timeout = self.config.timeout_ms() as i64;
        log.filter_traces(&|t, _| {
            t.events()
                .last()
                .unwrap()
                .borrow()
                .timestamp()
                .sub(current_stamp)
                .num_milliseconds()
                > timeout
        });
    }
}

#[derive(Clone)]
pub(in crate::grpc) struct T1StreamingProcessor {
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

    fn observe(&self, trace: BxesKafkaTrace, context: &mut PipelineContext) -> Result<(), XesFromBxesKafkaTraceCreatingError> {
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
        let metadata = trace.metadata();
        let mut names_to_logs = self.names_to_logs.lock();
        let names_to_logs = match names_to_logs.as_mut() {
            Ok(names_to_logs) => names_to_logs,
            Err(_) => panic!("Failed to acquire a names_to_logs map from mutex"),
        };

        let process_name = Self::string_value_or_err(metadata, KAFKA_PROCESS_NAME)?;
        let case_display_name = Self::string_value_or_err(metadata, KAFKA_CASE_DISPLAY_NAME)?;
        let case_name_parts_joined = Self::string_value_or_err(metadata, KAFKA_CASE_NAME_PARTS)?;
        let case_name_parts: Vec<String> = case_name_parts_joined
            .split(KAFKA_CASE_NAME_PARTS_SEPARATOR)
            .map(|s| s.to_string())
            .collect();

        if !names_to_logs.contains_key(case_name_parts_joined.as_str()) {
            let new_log = XesEventLogImpl::empty();
            names_to_logs.insert(case_name_parts_joined.to_owned(), new_log);
        }

        let existing_log = names_to_logs
            .get_mut(case_name_parts_joined.as_str())
            .expect("Log should be present");

        self.filterer.filter(existing_log);

        let xes_trace = match read_bxes_events(trace.events()) {
            Ok(xes_trace) => xes_trace,
            Err(err) => return Err(XesFromBxesKafkaTraceCreatingError::BxesToXexConversionError(err)),
        };

        let xes_trace = Rc::new(RefCell::new(xes_trace));
        existing_log.push(xes_trace);

        let result = LogUpdateResult {
            process_name,
            case_name: CaseName {
                display_name: case_display_name,
                name_parts: case_name_parts,
            },
            new_log: existing_log.clone(),
            unstructured_metadata: Self::metadata_to_string_string_pairs(metadata),
        };

        Ok(result)
    }

    fn string_value_or_err(
        metadata: &HashMap<String, Rc<Box<BxesValue>>>,
        key_name: &str,
    ) -> Result<String, XesFromBxesKafkaTraceCreatingError> {
        if let Some(value) = metadata.get(key_name) {
            if let BxesValue::String(process_name) = value.as_ref().as_ref() {
                Ok(process_name.as_ref().as_ref().to_owned())
            } else {
                Err(XesFromBxesKafkaTraceCreatingError::MetadataValueIsNotAString(key_name.to_string()))
            }
        } else {
            Err(XesFromBxesKafkaTraceCreatingError::MetadataValueNotFound(key_name.to_string()))
        }
    }

    fn metadata_to_string_string_pairs(metadata: &HashMap<String, Rc<Box<BxesValue>>>) -> Vec<(String, String)> {
        metadata
            .iter()
            .map(|pair| {
                if pair.0 == KAFKA_CASE_NAME_PARTS || pair.0 == KAFKA_CASE_DISPLAY_NAME || pair.0 == KAFKA_PROCESS_NAME {
                    None
                } else {
                    if let BxesValue::String(value) = pair.1.as_ref().as_ref() {
                        Some((pair.0.to_owned(), value.as_ref().as_ref().to_owned()))
                    } else {
                        None
                    }
                }
            })
            .filter(|kv| kv.is_some())
            .map(|kv| kv.unwrap())
            .collect()
    }
}
