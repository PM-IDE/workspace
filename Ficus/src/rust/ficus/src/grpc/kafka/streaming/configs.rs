use crate::ficus_proto::{
    GrpcPipelineStreamingConfiguration, GrpcT1EventsTimeBasedCaching, GrpcT1StreamingConfiguration, GrpcT1TraceTimeBasedCaching,
    GrpcT2LossyCountConfiguration, GrpcT2StreamingConfiguration,
};
use crate::grpc::kafka::streaming::processors::{
    EventsTimeoutFiltererImpl, T1LogFilterer, T1StreamingProcessor, TracesProcessor, TracesTimeoutFiltererImpl,
};

type StreamingConfigurationEnum = crate::ficus_proto::grpc_pipeline_streaming_configuration::Configuration;
type T1ConfigurationEnum = crate::ficus_proto::grpc_t1_streaming_configuration::Configuration;
type T2ConfigurationEnum = crate::ficus_proto::grpc_t2_streaming_configuration::Configuration;

pub(in crate::grpc) enum StreamingConfiguration {
    NotSpecified,
    T1(T1StreamingConfiguration),
    T2(T2StreamingConfiguration),
}

impl StreamingConfiguration {
    pub fn new(grpc_config: &GrpcPipelineStreamingConfiguration) -> Option<Self> {
        match grpc_config.configuration.as_ref() {
            None => None,
            Some(c) => match c {
                StreamingConfigurationEnum::NotSpecified(_) => Some(StreamingConfiguration::NotSpecified),
                StreamingConfigurationEnum::T1Configuration(t1) => match T1StreamingConfiguration::new(t1) {
                    None => None,
                    Some(t1) => Some(StreamingConfiguration::T1(t1)),
                },
                StreamingConfigurationEnum::T2Configuration(t2) => match T2StreamingConfiguration::new(t2) {
                    None => None,
                    Some(t2) => Some(StreamingConfiguration::T2(t2)),
                },
            },
        }
    }

    pub fn create_processor(&self) -> TracesProcessor {
        match self {
            StreamingConfiguration::NotSpecified => TracesProcessor::T1(T1StreamingProcessor::new(T1LogFilterer::None)),
            StreamingConfiguration::T1(c) => TracesProcessor::T1(T1StreamingProcessor::new(match c {
                T1StreamingConfiguration::EventsTimeout(c) => {
                    T1LogFilterer::EventsTimeoutFilterer(EventsTimeoutFiltererImpl::new(c.clone()))
                }
                T1StreamingConfiguration::TracesTimeout(c) => {
                    T1LogFilterer::TracesTimeoutFilterer(TracesTimeoutFiltererImpl::new(c.clone()))
                }
            })),
            StreamingConfiguration::T2(_) => todo!(),
        }
    }
}

pub(in crate::grpc) enum T1StreamingConfiguration {
    EventsTimeout(EventsTimeoutConfiguration),
    TracesTimeout(TracesTimeoutConfiguration),
}

impl T1StreamingConfiguration {
    pub fn new(grpc_config: &GrpcT1StreamingConfiguration) -> Option<Self> {
        match grpc_config.configuration.as_ref() {
            None => None,
            Some(c) => Some(match c {
                T1ConfigurationEnum::EventsTimeout(et) => T1StreamingConfiguration::EventsTimeout(EventsTimeoutConfiguration::new(&et)),
                T1ConfigurationEnum::TracesTimeout(tt) => T1StreamingConfiguration::TracesTimeout(TracesTimeoutConfiguration::new(&tt)),
            }),
        }
    }
}

#[derive(Clone)]
pub(in crate::grpc) struct EventsTimeoutConfiguration {
    timeout_ms: u64,
}

impl EventsTimeoutConfiguration {
    pub fn new(grpc_events_timeout: &GrpcT1EventsTimeBasedCaching) -> Self {
        Self {
            timeout_ms: grpc_events_timeout.events_timeout_ms as u64,
        }
    }

    pub fn timeout_ms(&self) -> u64 {
        self.timeout_ms.clone()
    }
}

#[derive(Clone)]
pub(in crate::grpc) struct TracesTimeoutConfiguration {
    timeout_ms: u64,
}

impl TracesTimeoutConfiguration {
    pub fn new(grpc_traces_timeout: &GrpcT1TraceTimeBasedCaching) -> Self {
        Self {
            timeout_ms: grpc_traces_timeout.traces_timeout_ms as u64,
        }
    }

    pub fn timeout_ms(&self) -> u64 {
        self.timeout_ms.clone()
    }
}

pub(in crate::grpc) enum T2StreamingConfiguration {
    LossyCount(LossyCountConfiguration),
}

impl T2StreamingConfiguration {
    pub fn new(grpc_config: &GrpcT2StreamingConfiguration) -> Option<Self> {
        match grpc_config.configuration.as_ref() {
            None => None,
            Some(c) => Some(match c {
                T2ConfigurationEnum::LossyCount(lc) => T2StreamingConfiguration::LossyCount(LossyCountConfiguration {
                    error: lc.error,
                    support: lc.support,
                }),
            }),
        }
    }
}

pub(in crate::grpc) struct LossyCountConfiguration {
    error: f64,
    support: f64,
}
