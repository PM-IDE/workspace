use crate::grpc::kafka::{
  models::{
    KafkaTraceProcessingError, PipelineExecutionDto, XesFromBxesKafkaTraceCreatingError, KAFKA_CASE_DISPLAY_NAME, KAFKA_CASE_ID,
    KAFKA_CASE_NAME_PARTS, KAFKA_CASE_NAME_PARTS_SEPARATOR, KAFKA_PROCESS_NAME,
  },
  streaming::{t1::processors::T1StreamingProcessor, t2::processors::T2StreamingProcessor},
};
use bxes::models::domain::bxes_value::BxesValue;
use bxes_kafka::consumer::bxes_kafka_consumer::BxesKafkaTrace;
use ficus::{
  features::cases::CaseName,
  pipelines::{
    context::PipelineContext,
    keys::context_keys::{CASE_NAME_KEY, PROCESS_NAME_KEY, UNSTRUCTURED_METADATA_KEY},
  },
  utils::user_data::user_data::UserData,
};
use std::{collections::HashMap, rc::Rc};
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
  pub process_name: Rc<str>,
}

impl ProcessMetadata {
  pub fn create_from(metadata: &HashMap<Rc<str>, Rc<BxesValue>>) -> Result<Self, XesFromBxesKafkaTraceCreatingError> {
    let process_name = string_value_or_err(metadata, KAFKA_PROCESS_NAME)?;

    Ok(Self { process_name })
  }
}

pub(in crate::grpc::kafka::streaming) struct CaseMetadata {
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

pub(in crate::grpc::kafka::streaming) struct ExtractedTraceMetadata {
  pub process: ProcessMetadata,
  pub case: CaseMetadata,
  pub unstructured_metadata: Vec<(Rc<str>, Rc<str>)>,
}

impl ExtractedTraceMetadata {
  pub fn create_from(metadata: &HashMap<Rc<str>, Rc<BxesValue>>) -> Result<Self, XesFromBxesKafkaTraceCreatingError> {
    Ok(ExtractedTraceMetadata {
      process: ProcessMetadata::create_from(metadata)?,
      case: CaseMetadata::create_from(metadata)?,
      unstructured_metadata: metadata_to_string_string_pairs(metadata),
    })
  }
}

fn add_system_metadata(
  metadata: &HashMap<Rc<str>, Rc<BxesValue>>,
  context: &mut PipelineContext,
) -> Result<(), XesFromBxesKafkaTraceCreatingError> {
  let metadata = ExtractedTraceMetadata::create_from(metadata)?;

  context.put_concrete(PROCESS_NAME_KEY.key(), metadata.process.process_name);
  context.put_concrete(UNSTRUCTURED_METADATA_KEY.key(), metadata.unstructured_metadata);
  context.put_concrete(
    CASE_NAME_KEY.key(),
    CaseName {
      display_name: metadata.case.case_display_name,
      name_parts: metadata.case.case_name_parts,
    },
  );

  Ok(())
}

pub(in crate::grpc::kafka::streaming) fn string_value_or_err(
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

pub(in crate::grpc::kafka::streaming) fn value_or_err(
  metadata: &HashMap<Rc<str>, Rc<BxesValue>>,
  key: &str,
) -> Result<Rc<BxesValue>, XesFromBxesKafkaTraceCreatingError> {
  if let Some(value) = metadata.get(key) {
    Ok(value.clone())
  } else {
    Err(XesFromBxesKafkaTraceCreatingError::MetadataValueNotFound(key.to_string()))
  }
}

pub(in crate::grpc::kafka::streaming) fn uuid_or_err(
  metadata: &HashMap<Rc<str>, Rc<BxesValue>>,
  key: &str,
) -> Result<Uuid, XesFromBxesKafkaTraceCreatingError> {
  let value = value_or_err(metadata, key)?;
  if let BxesValue::Guid(id) = value.as_ref() {
    Ok(*id)
  } else {
    Err(XesFromBxesKafkaTraceCreatingError::TraceIdIsNotUuid)
  }
}

pub(in crate::grpc::kafka::streaming) fn metadata_to_string_string_pairs(
  metadata: &HashMap<Rc<str>, Rc<BxesValue>>,
) -> Vec<(Rc<str>, Rc<str>)> {
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
