use crate::grpc::kafka::models::XesFromBxesKafkaTraceCreatingError;
use crate::grpc::kafka::streaming::t2::dfg_data_structures::DfgDataStructures;
use crate::pipelines::context::PipelineContext;
use bxes_kafka::consumer::bxes_kafka_consumer::BxesKafkaTrace;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Clone)]
pub struct T2StreamingProcessor {
    dfg_data_structure: Arc<Mutex<DfgDataStructures>>,
}

impl T2StreamingProcessor {
    pub fn new_sliding_window(element_lifetime: Duration) -> Self {
        Self {
            dfg_data_structure: Arc::new(Mutex::new(DfgDataStructures::new_sliding_window(element_lifetime))),
        }
    }

    pub fn new_lossy_count(error: f64) -> Self {
        Self {
            dfg_data_structure: Arc::new(Mutex::new(DfgDataStructures::new_lossy_count(error))),
        }
    }

    pub fn observe(&self, trace: &BxesKafkaTrace, context: &mut PipelineContext) -> Result<(), XesFromBxesKafkaTraceCreatingError> {
        let mut dfg_data_structure = self.dfg_data_structure.lock().expect("Must acquire lock");

        dfg_data_structure.invalidate();
        dfg_data_structure.process_bxes_trace(trace, context)
    }
}
