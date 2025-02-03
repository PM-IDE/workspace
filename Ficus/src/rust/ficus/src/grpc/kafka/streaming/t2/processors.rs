use crate::features::streaming::counters::lossy_count::LossyCount;
use crate::features::streaming::counters::sliding_window::SlidingWindow;
use crate::grpc::kafka::models::XesFromBxesKafkaTraceCreatingError;
use crate::grpc::kafka::streaming::processors::{CaseMetadata, ProcessMetadata};
use crate::pipelines::context::PipelineContext;
use bxes_kafka::consumer::bxes_kafka_consumer::BxesKafkaTrace;
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

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

#[derive(Clone)]
pub struct T2LossyCountStreamingProcessor {
    error: f64,
    support: f64,
    processes_dfg: HashMap<String, LossyCount<(String, String), ()>>,
    traces_last_event_class: LossyCount<Uuid, String>,
}

impl T2LossyCountStreamingProcessor {
    pub fn new(error: f64, support: f64) -> Self {
        Self {
            error,
            support,
            processes_dfg: HashMap::new(),
            traces_last_event_class: LossyCount::new(error),
        }
    }

    pub fn observe(&self, trace: BxesKafkaTrace, context: &mut PipelineContext) -> Result<(), XesFromBxesKafkaTraceCreatingError> {
        Ok(())
    }
}

#[derive(Clone)]
pub struct T2SlidingWindowProcessor {
    element_lifespan: Duration,
    processes_dfg: HashMap<String, SlidingWindow<(String, String), u64>>,
    traces_last_event_classes: SlidingWindow<String, String>,
}

impl T2SlidingWindowProcessor {
    pub fn new(element_lifespan: Duration) -> Self {
        Self {
            element_lifespan,
            processes_dfg: HashMap::new(),
            traces_last_event_classes: SlidingWindow::new_time(element_lifespan),
        }
    }

    pub fn observe(&self, trace: BxesKafkaTrace, context: &mut PipelineContext) -> Result<(), XesFromBxesKafkaTraceCreatingError> {
        let process_metadata = ProcessMetadata::create_from(trace.metadata())?;
        let case_metadata = CaseMetadata::create_from(trace.metadata())?;

        Ok(())
    }
}
