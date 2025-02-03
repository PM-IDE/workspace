use crate::grpc::kafka::models::XesFromBxesKafkaTraceCreatingError;
use crate::pipelines::context::PipelineContext;
use bxes_kafka::consumer::bxes_kafka_consumer::BxesKafkaTrace;
use crate::grpc::kafka::streaming::t2::lossy_count_processor::T2LossyCountStreamingProcessor;
use crate::grpc::kafka::streaming::t2::sliding_window_processor::T2SlidingWindowProcessor;

#[derive(Clone)]
pub enum T2StreamingProcessor {
    LossyCount(T2LossyCountStreamingProcessor),
    SlidingWindow(T2SlidingWindowProcessor),
}

impl T2StreamingProcessor {
    pub fn observe(&self, trace: BxesKafkaTrace, context: &mut PipelineContext) -> Result<(), XesFromBxesKafkaTraceCreatingError> {
        match self {
            T2StreamingProcessor::LossyCount(processor) => processor.observe(trace, context),
            T2StreamingProcessor::SlidingWindow(processor) => processor.observe(trace, context),
        }
    }
}