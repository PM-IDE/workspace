use crate::ficus_proto::grpc_t2_streaming_configuration::Configuration;
use crate::ficus_proto::{GrpcPipeline, GrpcPipelineExecutionRequest, GrpcT2StreamingConfiguration};
use crate::grpc::kafka::streaming::t2::processors::T2StreamingProcessor;
use std::time::Duration;

pub enum T2StreamingConfiguration {
    LossyCount(LossyCountConfiguration),
    SlidingWindow(TimedSlidingWindowConfiguration),
}

impl T2StreamingConfiguration {
    pub fn new(grpc_config: &GrpcT2StreamingConfiguration) -> Option<Self> {
        match grpc_config.configuration.as_ref() {
            None => None,
            Some(c) => Some(match c {
                Configuration::LossyCount(lc) => T2StreamingConfiguration::LossyCount(LossyCountConfiguration {
                    error: lc.error,
                    support: lc.support,
                    trace_preprocessing_pipeline: grpc_config.incoming_traces_filtering_pipeline.clone()
                }),
                Configuration::TimedSlidingWindow(sc) => T2StreamingConfiguration::SlidingWindow(TimedSlidingWindowConfiguration {
                    element_lifetime: Duration::from_millis(sc.lifespan_ms as u64),
                    trace_preprocessing_pipeline: grpc_config.incoming_traces_filtering_pipeline.clone()
                }),
            }),
        }
    }

    pub fn create_processor(&self) -> T2StreamingProcessor {
        match self {
            T2StreamingConfiguration::LossyCount(lc) => lc.create_processor(),
            T2StreamingConfiguration::SlidingWindow(sw) => sw.create_processor(),
        }
    }
}

pub struct LossyCountConfiguration {
    error: f64,
    support: f64,
    trace_preprocessing_pipeline: Option<GrpcPipeline>
}

impl LossyCountConfiguration {
    pub fn create_processor(&self) -> T2StreamingProcessor {
        T2StreamingProcessor::new_lossy_count(self.error, self.trace_preprocessing_pipeline.clone())
    }
}

pub struct TimedSlidingWindowConfiguration {
    element_lifetime: Duration,
    trace_preprocessing_pipeline: Option<GrpcPipeline>
}

impl TimedSlidingWindowConfiguration {
    pub fn create_processor(&self) -> T2StreamingProcessor {
        T2StreamingProcessor::new_sliding_window(self.element_lifetime, self.trace_preprocessing_pipeline.clone())
    }
}
