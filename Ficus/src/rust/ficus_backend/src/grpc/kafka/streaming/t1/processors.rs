use crate::grpc::kafka::models::{ExtractedTraceMetadata, string_value_or_err, uuid_or_err};
use crate::grpc::kafka::streaming::processors::ProcessorState;
use crate::grpc::{
  kafka::models::{KAFKA_CASE_ID, KAFKA_CASE_NAME_PARTS, KAFKA_TRACE_ID, KafkaTraceProcessingError, XesFromBxesKafkaTraceCreatingError},
  logs_handler::ConsoleLogMessageHandler,
};
use bxes_kafka::consumer::bxes_kafka_consumer::BxesKafkaTrace;
use ficus::pipelines::keys::context_keys::EVENT_LOG_KEY;
use ficus::utils::user_data::user_data::UserData;
use ficus::{
  event_log::{
    bxes::bxes_to_xes_converter::read_bxes_events,
    core::{event::event::EventPayloadValue, event_log::EventLog, trace::trace::Trace},
    xes::xes_event_log::XesEventLogImpl,
  },
  features::streaming::t1::filterers::T1LogFilterer,
  pipelines::context::{LogMessageHandler, PipelineContext},
};
use log::{info, warn};
use std::{
  cell::RefCell,
  collections::HashMap,
  rc::Rc,
  sync::{Arc, Mutex},
};
use uuid::Uuid;

#[derive(Clone)]
pub struct T1StreamingProcessor {
  logger: ConsoleLogMessageHandler,
  names_to_processed_data: Arc<Mutex<HashMap<String, ProcessorState<XesEventLogImpl>>>>,
  filterer: T1LogFilterer,
}

impl T1StreamingProcessor {
  pub fn new(filterer: T1LogFilterer) -> Self {
    Self {
      logger: ConsoleLogMessageHandler::new(),
      names_to_processed_data: Arc::new(Mutex::new(HashMap::new())),
      filterer,
    }
  }

  pub fn observe(&self, trace: &BxesKafkaTrace) -> Result<(), KafkaTraceProcessingError> {
    match self.update_log(trace) {
      Ok(_) => Ok(()),
      Err(err) => {
        let message = format!("Failed to get update result, err: {}", err);
        self.logger.handle(message.as_str()).expect("Must log message");
        Err(KafkaTraceProcessingError::XesFromBxesTraceCreationError(err))
      }
    }
  }

  pub fn fill_pipeline_context(&self, context: &mut PipelineContext, case_name: &str) {
    let names_to_processed_data = self.names_to_processed_data.lock().unwrap();

    if let Some(data) = names_to_processed_data.get(case_name).cloned() {
      context.put_concrete(EVENT_LOG_KEY.key(), data.data);
      data.metadata.write_to_context(context);

      info!("Filled event log for case {case_name:?}");
    } else {
      warn!("Failed to find log for case {case_name:?}");
    }
  }
}

impl T1StreamingProcessor {
  fn update_log(&self, trace: &BxesKafkaTrace) -> Result<(), XesFromBxesKafkaTraceCreatingError> {
    let case_id = uuid_or_err(trace.metadata(), KAFKA_CASE_ID)?;
    let case_name_parts_joined = string_value_or_err(trace.metadata(), KAFKA_CASE_NAME_PARTS)?;

    self.get_or_create_event_log(trace, case_id, case_name_parts_joined.as_ref())
  }

  fn get_or_create_event_log(
    &self,
    trace: &BxesKafkaTrace,
    trace_id: Uuid,
    case_key: &str,
  ) -> Result<(), XesFromBxesKafkaTraceCreatingError> {
    let mut names_to_processed_data = self.names_to_processed_data.lock();
    let names_to_processed_data = match names_to_processed_data.as_mut() {
      Ok(names_to_logs) => names_to_logs,
      Err(_) => panic!("Failed to acquire a names_to_logs map from mutex"),
    };

    if !names_to_processed_data.contains_key(case_key) {
      let new_log = XesEventLogImpl::default();
      names_to_processed_data.insert(
        case_key.to_owned(),
        ProcessorState {
          data: new_log,
          metadata: Arc::new(ExtractedTraceMetadata::create_from(trace.metadata())?),
        },
      );
    }

    let existing_data = names_to_processed_data.get_mut(case_key).expect("Log should be present");

    self.filterer.filter(&mut existing_data.data);

    let mut read_xes_trace = match read_bxes_events(trace.events()) {
      Ok(xes_trace) => xes_trace,
      Err(err) => return Err(XesFromBxesKafkaTraceCreatingError::BxesToXexConversionError(err)),
    };

    for existing_xes_trace in existing_data.data.traces() {
      let mut existing_xes_trace = existing_xes_trace.borrow_mut();
      if let Some(EventPayloadValue::Guid(id)) = existing_xes_trace.metadata().get(KAFKA_TRACE_ID).cloned() {
        if id == trace_id {
          info!("Found an existing trace for trace id {}, appending it", id);

          for event in read_xes_trace.events() {
            existing_xes_trace.push(event.clone());
          }

          drop(existing_xes_trace);
          return Ok(());
        }
      }
    }

    read_xes_trace
      .metadata_mut()
      .insert(KAFKA_TRACE_ID.to_owned(), EventPayloadValue::Guid(trace_id));

    let read_xes_trace = Rc::new(RefCell::new(read_xes_trace));
    existing_data.data.push(read_xes_trace);

    info!("Created new trace for trace id {}", trace_id);

    Ok(())
  }
}
