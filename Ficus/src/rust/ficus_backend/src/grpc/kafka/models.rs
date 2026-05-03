use crate::grpc::{
  events::events_handler::PipelineEventsHandler, kafka::kafka_service::KafkaSubscription, logs_handler::ConsoleLogMessageHandler,
};
use ficus::{
  event_log::bxes::bxes_to_xes_converter::BxesToXesReadError,
  pipelines::{errors::pipeline_errors::PipelinePartExecutionError, pipeline_parts::PipelineParts},
};
use std::{
  collections::HashMap,
  fmt::{Debug, Display, Formatter},
  sync::{Arc, Mutex},
};
use uuid::Uuid;

pub(super) const KAFKA_CASE_DISPLAY_NAME: &str = "case_display_name";
pub(super) const KAFKA_CASE_NAME_PARTS: &str = "case_name_parts";
pub(super) const KAFKA_CASE_ID: &str = "case_id";
pub(super) const KAFKA_CASE_NAME_PARTS_SEPARATOR: &str = ";";
pub(super) const KAFKA_PROCESS_NAME: &str = "process_name";
pub(super) const KAFKA_TRACE_ID: &str = "trace_id";

#[derive(Debug)]
pub enum KafkaTraceProcessingError {
  XesFromBxesTraceCreationError(XesFromBxesKafkaTraceCreatingError),
  FailedToPreprocessTrace(PipelinePartExecutionError),
}

impl Display for KafkaTraceProcessingError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      KafkaTraceProcessingError::XesFromBxesTraceCreationError(e) => Display::fmt(e, f),
      KafkaTraceProcessingError::FailedToPreprocessTrace(e) => Display::fmt(e, f),
    }
  }
}

#[derive(Debug)]
pub enum XesFromBxesKafkaTraceCreatingError {
  MetadataValueIsNotAString(String),
  MetadataValueNotFound(String),
  BxesToXexConversionError(BxesToXesReadError),
  TraceIdIsNotUuid,
}

impl Display for XesFromBxesKafkaTraceCreatingError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let str = match self {
      XesFromBxesKafkaTraceCreatingError::BxesToXexConversionError(err) => err.to_string(),
      XesFromBxesKafkaTraceCreatingError::MetadataValueIsNotAString(key_name) => {
        format!("Value for key {} is not a String", key_name.to_owned())
      }
      XesFromBxesKafkaTraceCreatingError::MetadataValueNotFound(key_name) => {
        format!("The key {} is not found", key_name)
      }
      XesFromBxesKafkaTraceCreatingError::TraceIdIsNotUuid => "Trace id was not of type uuid ".to_string(),
    };

    write!(f, "{}", str)
  }
}

#[derive(Clone)]
pub struct PipelineExecutionDto {
  pub(super) pipeline_parts: Arc<PipelineParts>,
  pub(super) events_handler: Arc<dyn PipelineEventsHandler>,
}

impl PipelineExecutionDto {
  pub fn new(pipeline_parts: Arc<PipelineParts>, events_handler: Arc<dyn PipelineEventsHandler>) -> Self {
    Self {
      pipeline_parts,
      events_handler,
    }
  }
}

#[derive(Clone)]
pub struct KafkaConsumerCreationDto {
  pub uuid: Uuid,
  pub name: Arc<str>,
  pub subscriptions_to_execution_requests: Arc<Mutex<HashMap<Uuid, KafkaSubscription>>>,
  pub logger: ConsoleLogMessageHandler,
}

impl KafkaConsumerCreationDto {
  pub fn new(name: Arc<str>, subscriptions_to_execution_requests: Arc<Mutex<HashMap<Uuid, KafkaSubscription>>>) -> Self {
    Self {
      uuid: Uuid::new_v4(),
      name,
      subscriptions_to_execution_requests,
      logger: ConsoleLogMessageHandler::new(),
    }
  }
}
