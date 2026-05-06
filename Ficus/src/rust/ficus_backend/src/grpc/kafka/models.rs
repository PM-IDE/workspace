use crate::grpc::{
  events::events_handler::PipelineEventsHandler, kafka::kafka_service::KafkaSubscription, logs_handler::ConsoleLogMessageHandler,
};
use bxes::models::domain::bxes_value::BxesValue;
use ficus::features::cases::CaseName;
use ficus::pipelines::context::PipelineContext;
use ficus::pipelines::keys::context_keys::{CASE_NAME_KEY, PROCESS_NAME_KEY, UNSTRUCTURED_METADATA_KEY};
use ficus::utils::user_data::user_data::UserData;
use ficus::{
  event_log::bxes::bxes_to_xes_converter::BxesToXesReadError,
  pipelines::{errors::pipeline_errors::PipelinePartExecutionError, pipeline_parts::PipelineParts},
};
use std::rc::Rc;
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

pub(super) struct ExtractedTraceMetadata {
  pub process: ProcessMetadata,
  pub case: CaseMetadata,
  pub unstructured_metadata: Vec<(Rc<str>, Rc<str>)>,
}

unsafe impl Send for ExtractedTraceMetadata {}
unsafe impl Sync for ExtractedTraceMetadata {}

impl ExtractedTraceMetadata {
  pub fn create_from(metadata: &HashMap<Rc<str>, Rc<BxesValue>>) -> Result<Self, XesFromBxesKafkaTraceCreatingError> {
    Ok(ExtractedTraceMetadata {
      process: ProcessMetadata::create_from(metadata)?,
      case: CaseMetadata::create_from(metadata)?,
      unstructured_metadata: metadata_to_string_string_pairs(metadata),
    })
  }

  pub fn write_to_context(&self, context: &mut PipelineContext) {
    context.put_concrete(PROCESS_NAME_KEY.key(), self.process.process_name.clone());
    context.put_concrete(UNSTRUCTURED_METADATA_KEY.key(), self.unstructured_metadata.clone());
    context.put_concrete(
      CASE_NAME_KEY.key(),
      CaseName {
        display_name: self.case.case_display_name.clone(),
        name_parts: self.case.case_name_parts.clone(),
      },
    );
  }
}

pub(super) struct ProcessMetadata {
  pub process_name: Rc<str>,
}

impl ProcessMetadata {
  pub fn create_from(metadata: &HashMap<Rc<str>, Rc<BxesValue>>) -> Result<Self, XesFromBxesKafkaTraceCreatingError> {
    let process_name = string_value_or_err(metadata, KAFKA_PROCESS_NAME)?;

    Ok(Self { process_name })
  }
}

pub(super) struct CaseMetadata {
  pub case_id: Uuid,
  pub case_display_name: Rc<str>,
  pub case_name_parts: Vec<Rc<str>>,
  pub case_name_parts_joined: Rc<str>,
}

impl CaseMetadata {
  pub fn create_from(metadata: &HashMap<Rc<str>, Rc<BxesValue>>) -> Result<Self, XesFromBxesKafkaTraceCreatingError> {
    let case_id = uuid_or_err(metadata, KAFKA_CASE_ID)?;
    let case_name_parts_joined = string_value_or_err(metadata, KAFKA_CASE_NAME_PARTS)?;
    let case_display_name = string_value_or_err(metadata, KAFKA_CASE_DISPLAY_NAME)?;
    let case_name_parts: Vec<Rc<str>> = case_name_parts_joined
      .split(KAFKA_CASE_NAME_PARTS_SEPARATOR)
      .map(|s| s.to_string().into())
      .collect();

    Ok(Self {
      case_id,
      case_display_name,
      case_name_parts,
      case_name_parts_joined,
    })
  }
}

pub(super) fn string_value_or_err(
  metadata: &HashMap<Rc<str>, Rc<BxesValue>>,
  key_name: &str,
) -> Result<Rc<str>, XesFromBxesKafkaTraceCreatingError> {
  let value = value_or_err(metadata, key_name)?;

  if let BxesValue::String(process_name) = value.as_ref() {
    Ok(process_name.clone())
  } else {
    Err(XesFromBxesKafkaTraceCreatingError::MetadataValueIsNotAString(key_name.to_string()))
  }
}

fn value_or_err(metadata: &HashMap<Rc<str>, Rc<BxesValue>>, key: &str) -> Result<Rc<BxesValue>, XesFromBxesKafkaTraceCreatingError> {
  if let Some(value) = metadata.get(key) {
    Ok(value.clone())
  } else {
    Err(XesFromBxesKafkaTraceCreatingError::MetadataValueNotFound(key.to_string()))
  }
}

pub(super) fn uuid_or_err(metadata: &HashMap<Rc<str>, Rc<BxesValue>>, key: &str) -> Result<Uuid, XesFromBxesKafkaTraceCreatingError> {
  let value = value_or_err(metadata, key)?;
  if let BxesValue::Guid(id) = value.as_ref() {
    Ok(*id)
  } else {
    Err(XesFromBxesKafkaTraceCreatingError::TraceIdIsNotUuid)
  }
}

fn metadata_to_string_string_pairs(metadata: &HashMap<Rc<str>, Rc<BxesValue>>) -> Vec<(Rc<str>, Rc<str>)> {
  metadata
    .iter()
    .filter_map(|pair| {
      if pair.0.as_ref() == KAFKA_CASE_NAME_PARTS || pair.0.as_ref() == KAFKA_CASE_DISPLAY_NAME || pair.0.as_ref() == KAFKA_PROCESS_NAME {
        None
      } else if let BxesValue::String(value) = pair.1.as_ref() {
        Some((pair.0.to_owned(), value.clone()))
      } else {
        None
      }
    })
    .collect()
}
