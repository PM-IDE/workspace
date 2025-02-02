use std::time::Duration;
use crate::ficus_proto::grpc_t2_streaming_configuration::Configuration;
use crate::ficus_proto::GrpcT2StreamingConfiguration;
use crate::grpc::kafka::streaming::t2::processors::{T2LossyCountStreamingProcessor, T2SlidingWindowProcessor, T2StreamingProcessor};

pub enum T2StreamingConfiguration {
    LossyCount(LossyCountConfiguration),
    SlidingWindow(TimedSlidingWindowConfiguration)
}

impl T2StreamingConfiguration {
    pub fn new(grpc_config: &GrpcT2StreamingConfiguration) -> Option<Self> {
        match grpc_config.configuration.as_ref() {
            None => None,
            Some(c) => Some(match c {
                Configuration::LossyCount(lc) => T2StreamingConfiguration::LossyCount(LossyCountConfiguration {
                    error: lc.error,
                    support: lc.support,
                }),
                Configuration::TimedSlidingWindow(sc) => T2StreamingConfiguration::SlidingWindow(TimedSlidingWindowConfiguration {
                    element_lifetime: Duration::from_millis(sc.lifespan_ms as u64)
                })
            }),
        }
    }
    
    pub fn create_processor(&self) -> T2StreamingProcessor {
        match self {
            T2StreamingConfiguration::LossyCount(lc) => T2StreamingProcessor::LossyCount(lc.create_processor()),
            T2StreamingConfiguration::SlidingWindow(sw) => T2StreamingProcessor::SlidingWindow(sw.create_processor())
        }
    }
}

pub struct LossyCountConfiguration {
    error: f64,
    support: f64,
}

impl LossyCountConfiguration {
    pub fn create_processor(&self) -> T2LossyCountStreamingProcessor {
        T2LossyCountStreamingProcessor::new(self.error, self.support)
    }
}

pub struct TimedSlidingWindowConfiguration {
    element_lifetime: Duration
}

impl TimedSlidingWindowConfiguration {
    pub fn create_processor(&self) -> T2SlidingWindowProcessor {
        T2SlidingWindowProcessor::new(self.element_lifetime)
    }
}