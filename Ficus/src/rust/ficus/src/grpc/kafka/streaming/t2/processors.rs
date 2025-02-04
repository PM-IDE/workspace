use crate::grpc::kafka::models::XesFromBxesKafkaTraceCreatingError;
use crate::grpc::kafka::streaming::t2::dfg_data_structures::{
    DfgDataStructures, LossyCountDfgDataStructures, SlidingWindowDfgDataStructures,
};
use crate::pipelines::context::PipelineContext;
use bxes_kafka::consumer::bxes_kafka_consumer::BxesKafkaTrace;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Clone)]
pub struct T2StreamingProcessor {
    data_structure: Arc<Mutex<DfgDataStructures>>,
}

impl T2StreamingProcessor {
    pub fn new_sliding_window(element_lifetime: Duration) -> Self {
        Self {
            data_structure: Arc::new(Mutex::new(DfgDataStructures::SlidingWindow(SlidingWindowDfgDataStructures::new(
                element_lifetime,
            )))),
        }
    }

    pub fn new_lossy_count(error: f64) -> Self {
        Self {
            data_structure: Arc::new(Mutex::new(DfgDataStructures::LossyCount(LossyCountDfgDataStructures::new(error)))),
        }
    }

    pub fn observe(&self, trace: BxesKafkaTrace, context: &mut PipelineContext) -> Result<(), XesFromBxesKafkaTraceCreatingError> {
        let mut data_structure = self.data_structure.lock().expect("Must acquire lock");
        data_structure.process_bxes_trace(trace, context)
    }
}
