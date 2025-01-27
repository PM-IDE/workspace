use crate::grpc::kafka::models::{XesFromBxesKafkaTraceCreatingError, KAFKA_CASE_DISPLAY_NAME, KAFKA_CASE_ID, KAFKA_CASE_NAME_PARTS, KAFKA_CASE_NAME_PARTS_SEPARATOR, KAFKA_PROCESS_ID, KAFKA_PROCESS_NAME, KAFKA_TRACE_ID};
use crate::grpc::kafka::streaming::t1::processors::T1StreamingProcessor;
use crate::pipelines::context::PipelineContext;
use bxes::models::domain::bxes_value::BxesValue;
use bxes_kafka::consumer::bxes_kafka_consumer::BxesKafkaTrace;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

#[derive(Clone)]
pub enum TracesProcessor {
    T1(T1StreamingProcessor),
}

impl TracesProcessor {
    pub fn observe(&self, trace: BxesKafkaTrace, context: &mut PipelineContext) -> Result<(), XesFromBxesKafkaTraceCreatingError> {
        match self {
            TracesProcessor::T1(t1_processor) => t1_processor.observe(trace, context),
        }
    }
}

pub (in crate::grpc::kafka::streaming) struct ProcessMetadata {
    pub process_name: String,
    pub process_id: Uuid,
}

impl ProcessMetadata {
    pub fn create_from(metadata: &HashMap<String, Rc<Box<BxesValue>>>) -> Result<Self, XesFromBxesKafkaTraceCreatingError> {
        let process_name = string_value_or_err(metadata, KAFKA_PROCESS_NAME)?;
        let process_id = uuid_or_err(metadata, KAFKA_PROCESS_ID)?;

        Ok(Self {
            process_name,
            process_id
        })
    }
}

pub (in crate::grpc::kafka::streaming) struct CaseMetadata {
    pub case_id: Uuid,
    pub case_display_name: String,
    pub case_name_parts: Vec<String>,
    pub case_name_parts_joined: String
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

pub (in crate::grpc::kafka::streaming) struct ExtractedTraceMetadata {
    pub trace_id: Uuid,
    pub process: ProcessMetadata,
    pub case: CaseMetadata,
}

impl ExtractedTraceMetadata {
    pub fn create_from(metadata: &HashMap<String, Rc<Box<BxesValue>>>) -> Result<Self, XesFromBxesKafkaTraceCreatingError> {
        let trace_id = uuid_or_err(metadata, KAFKA_TRACE_ID)?;

        Ok(ExtractedTraceMetadata {
            trace_id,
            process: ProcessMetadata::create_from(metadata)?,
            case: CaseMetadata::create_from(metadata)?
        })
    }
}

fn string_value_or_err(
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

fn value_or_err(
    metadata: &HashMap<String, Rc<Box<BxesValue>>>,
    key: &str,
) -> Result<Rc<Box<BxesValue>>, XesFromBxesKafkaTraceCreatingError> {
    if let Some(value) = metadata.get(key) {
        Ok(value.clone())
    } else {
        Err(XesFromBxesKafkaTraceCreatingError::MetadataValueNotFound(key.to_string()))
    }
}

fn uuid_or_err(metadata: &HashMap<String, Rc<Box<BxesValue>>>, key: &str) -> Result<Uuid, XesFromBxesKafkaTraceCreatingError> {
    let value = value_or_err(metadata, key)?;
    if let BxesValue::Guid(id) = value.as_ref().as_ref() {
        Ok(id.clone())
    } else {
        Err(XesFromBxesKafkaTraceCreatingError::TraceIdIsNotUuid)
    }
}
