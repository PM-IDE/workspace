use crate::grpc::kafka::models::ExtractedTraceMetadata;
use crate::grpc::kafka::{
  models::{KafkaTraceProcessingError, PipelineExecutionDto, XesFromBxesKafkaTraceCreatingError},
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

#[derive(Clone)]
pub enum TracesProcessor {
  T1(T1StreamingProcessor),
  T2(T2StreamingProcessor),
}

pub struct KafkaTraceProcessingContext {
  pub trace: BxesKafkaTrace,
  pub execution_dto: PipelineExecutionDto,
}

impl TracesProcessor {
  pub fn observe(&self, mut context: KafkaTraceProcessingContext) -> Result<(), KafkaTraceProcessingError> {
    match self {
      TracesProcessor::T1(processor) => processor.observe(&context.trace),
      TracesProcessor::T2(processor) => processor.observe(&mut context),
    }
  }

  pub fn fill_pipeline_context(&self, context: &mut PipelineContext, process_name: &str) {
    match self {
      TracesProcessor::T1(processor) => processor.fill_pipeline_context(context, process_name),
      TracesProcessor::T2(processor) => processor.fill_pipeline_context(context, process_name),
    }
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
