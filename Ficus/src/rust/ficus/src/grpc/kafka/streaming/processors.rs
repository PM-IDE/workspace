use crate::grpc::kafka::models::XesFromBxesKafkaTraceCreatingError;
use crate::grpc::kafka::streaming::t1::processors::T1StreamingProcessor;
use crate::pipelines::context::PipelineContext;
use bxes_kafka::consumer::bxes_kafka_consumer::BxesKafkaTrace;

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
