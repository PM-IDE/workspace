use std::collections::HashMap;
use bxes_kafka::consumer::bxes_kafka_consumer::BxesKafkaTrace;
use uuid::Uuid;
use crate::features::streaming::counters::lossy_count::LossyCount;
use crate::grpc::kafka::models::XesFromBxesKafkaTraceCreatingError;
use crate::pipelines::context::PipelineContext;

pub struct T2LossyCountStreamingProcessor {
    processes_dfg: HashMap<String, LossyCount<(String, String), ()>>,
    traces_last_event_class: LossyCount<Uuid, String>
}

impl T2LossyCountStreamingProcessor {
    pub fn observe(&mut self, trace: BxesKafkaTrace, context: &mut PipelineContext) -> Result<(), XesFromBxesKafkaTraceCreatingError> {
        Ok(())
    }
}