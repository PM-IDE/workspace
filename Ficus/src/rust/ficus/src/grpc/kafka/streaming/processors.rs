use crate::grpc::events::events_handler::CaseName;
use crate::grpc::kafka::models::{
  KafkaTraceProcessingError, PipelineExecutionDto, XesFromBxesKafkaTraceCreatingError, KAFKA_CASE_DISPLAY_NAME, KAFKA_CASE_ID,
  KAFKA_CASE_NAME_PARTS, KAFKA_CASE_NAME_PARTS_SEPARATOR, KAFKA_PROCESS_NAME, KAFKA_TRACE_ID,
};
use crate::grpc::kafka::streaming::t1::processors::T1StreamingProcessor;
use crate::grpc::kafka::streaming::t2::processors::T2StreamingProcessor;
use crate::pipelines::context::PipelineContext;
use crate::pipelines::keys::context_keys::{CASE_NAME, PROCESS_NAME, UNSTRUCTURED_METADATA};
use crate::utils::user_data::user_data::UserData;
use bxes::models::domain::bxes_value::BxesValue;
use bxes_kafka::consumer::bxes_kafka_consumer::BxesKafkaTrace;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

#[derive(Clone)]
pub enum TracesProcessor {
  T1(T1StreamingProcessor),
  T2(T2StreamingProcessor),
}

pub struct KafkaTraceProcessingContext<'a, 'b> {
  pub trace: BxesKafkaTrace,
  pub context: &'a mut PipelineContext<'b>,
  pub execution_dto: PipelineExecutionDto,
}

impl TracesProcessor {
  pub fn observe(&self, mut context: KafkaTraceProcessingContext) -> Result<(), KafkaTraceProcessingError> {
    match self {
      TracesProcessor::T1(processor) => processor.observe(&context.trace, context.context),
      TracesProcessor::T2(processor) => processor.observe(&mut context),
    }?;

    match add_system_metadata(context.trace.metadata(), context.context) {
      Ok(_) => Ok(()),
      Err(err) => Err(KafkaTraceProcessingError::XesFromBxesTraceCreationError(err)),
    }
  }
}

pub(in crate::grpc::kafka::streaming) struct ProcessMetadata {
  pub process_name: String,
}

impl ProcessMetadata {
  pub fn create_from(metadata: &HashMap<String, Rc<Box<BxesValue>>>) -> Result<Self, XesFromBxesKafkaTraceCreatingError> {
    let process_name = string_value_or_err(metadata, KAFKA_PROCESS_NAME)?;

    Ok(Self { process_name })
  }
}

pub(in crate::grpc::kafka::streaming) struct CaseMetadata {
  pub case_id: Uuid,
  pub case_display_name: String,
  pub case_name_parts: Vec<String>,
  pub case_name_parts_joined: String,
}

impl CaseMetadata {
  pub fn create_from(metadata: &HashMap<String, Rc<Box<BxesValue>>>) -> Result<Self, XesFromBxesKafkaTraceCreatingError> {
    let case_id = uuid_or_err(metadata, KAFKA_CASE_ID)?;
    let case_name_parts_joined = string_value_or_err(metadata, KAFKA_CASE_NAME_PARTS)?;
    let case_display_name = string_value_or_err(metadata, KAFKA_CASE_DISPLAY_NAME)?;
    let case_name_parts: Vec<String> = case_name_parts_joined
      .split(KAFKA_CASE_NAME_PARTS_SEPARATOR)
      .map(|s| s.to_string())
      .collect();

    Ok(Self {
      case_id,
      case_display_name,
      case_name_parts,
      case_name_parts_joined,
    })
  }
}

pub(in crate::grpc::kafka::streaming) struct ExtractedTraceMetadata {
  pub process: ProcessMetadata,
  pub case: CaseMetadata,
  pub unstructured_metadata: Vec<(String, String)>,
}

impl ExtractedTraceMetadata {
  pub fn create_from(metadata: &HashMap<String, Rc<Box<BxesValue>>>) -> Result<Self, XesFromBxesKafkaTraceCreatingError> {
    Ok(ExtractedTraceMetadata {
      process: ProcessMetadata::create_from(metadata)?,
      case: CaseMetadata::create_from(metadata)?,
      unstructured_metadata: metadata_to_string_string_pairs(metadata),
    })
  }
}

fn add_system_metadata(
  metadata: &HashMap<String, Rc<Box<BxesValue>>>,
  context: &mut PipelineContext,
) -> Result<(), XesFromBxesKafkaTraceCreatingError> {
  let metadata = ExtractedTraceMetadata::create_from(metadata)?;

  context.put_concrete(PROCESS_NAME.key(), metadata.process.process_name);
  context.put_concrete(UNSTRUCTURED_METADATA.key(), metadata.unstructured_metadata);
  context.put_concrete(
    CASE_NAME.key(),
    CaseName {
      display_name: metadata.case.case_display_name,
      name_parts: metadata.case.case_name_parts,
    },
  );

  Ok(())
}

pub(in crate::grpc::kafka::streaming) fn string_value_or_err(
  metadata: &HashMap<String, Rc<Box<BxesValue>>>,
  key_name: &str,
) -> Result<String, XesFromBxesKafkaTraceCreatingError> {
  let value = value_or_err(metadata, key_name)?;

  if let BxesValue::String(process_name) = value.as_ref().as_ref() {
    Ok(process_name.as_ref().as_ref().to_owned())
  } else {
    Err(XesFromBxesKafkaTraceCreatingError::MetadataValueIsNotAString(key_name.to_string()))
  }
}

pub(in crate::grpc::kafka::streaming) fn value_or_err(
  metadata: &HashMap<String, Rc<Box<BxesValue>>>,
  key: &str,
) -> Result<Rc<Box<BxesValue>>, XesFromBxesKafkaTraceCreatingError> {
  if let Some(value) = metadata.get(key) {
    Ok(value.clone())
  } else {
    Err(XesFromBxesKafkaTraceCreatingError::MetadataValueNotFound(key.to_string()))
  }
}

pub(in crate::grpc::kafka::streaming) fn uuid_or_err(
  metadata: &HashMap<String, Rc<Box<BxesValue>>>,
  key: &str,
) -> Result<Uuid, XesFromBxesKafkaTraceCreatingError> {
  let value = value_or_err(metadata, key)?;
  if let BxesValue::Guid(id) = value.as_ref().as_ref() {
    Ok(id.clone())
  } else {
    Err(XesFromBxesKafkaTraceCreatingError::TraceIdIsNotUuid)
  }
}

pub(in crate::grpc::kafka::streaming) fn metadata_to_string_string_pairs(
  metadata: &HashMap<String, Rc<Box<BxesValue>>>,
) -> Vec<(String, String)> {
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
