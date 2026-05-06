use crate::grpc::kafka::{
  models::{ExtractedTraceMetadata, KafkaTraceProcessingError, PipelineExecutionDto},
  streaming::{t1::processors::T1StreamingProcessor, t2::processors::T2StreamingProcessor},
};
use bxes_kafka::consumer::bxes_kafka_consumer::BxesKafkaTrace;
use ficus::pipelines::context::PipelineContext;
use std::sync::Arc;

#[derive(Clone)]
pub enum TracesProcessor {
  T1(T1StreamingProcessor),
  T2(T2StreamingProcessor),
}

#[derive(Clone)]
pub(super) struct ProcessorState<TData> {
  pub data: TData,
  pub metadata: Arc<ExtractedTraceMetadata>,
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

  pub fn fill_pipeline_context(&self, context: &mut PipelineContext, case_name: &str) {
    match self {
      TracesProcessor::T1(processor) => processor.fill_pipeline_context(context, case_name),
      TracesProcessor::T2(processor) => processor.fill_pipeline_context(context, case_name),
    }
  }
}
