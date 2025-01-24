use crate::ficus_proto::grpc_t1_streaming_configuration::Configuration;
use crate::ficus_proto::{GrpcT1EventsTimeBasedCaching, GrpcT1StreamingConfiguration, GrpcT1TraceTimeBasedCaching};
use crate::grpc::kafka::streaming::t1::filterers::{EventsTimeoutFiltererImpl, T1LogFilterer, TracesTimeoutFiltererImpl};
use crate::grpc::kafka::streaming::t1::processors::T1StreamingProcessor;

pub(in crate::grpc) enum T1StreamingConfiguration {
    EventsTimeout(EventsTimeoutConfiguration),
    TracesTimeout(TracesTimeoutConfiguration),
}

impl T1StreamingConfiguration {
    pub fn new(grpc_config: &GrpcT1StreamingConfiguration) -> Option<Self> {
        match grpc_config.configuration.as_ref() {
            None => None,
            Some(c) => Some(match c {
                Configuration::EventsTimeout(et) => T1StreamingConfiguration::EventsTimeout(EventsTimeoutConfiguration::new(&et)),
                Configuration::TracesTimeout(tt) => T1StreamingConfiguration::TracesTimeout(TracesTimeoutConfiguration::new(&tt)),
            }),
        }
    }

    pub fn create_processor(&self) -> T1StreamingProcessor {
        T1StreamingProcessor::new(match self {
            T1StreamingConfiguration::EventsTimeout(c) => T1LogFilterer::EventsTimeoutFilterer(EventsTimeoutFiltererImpl::new(c.clone())),
            T1StreamingConfiguration::TracesTimeout(c) => T1LogFilterer::TracesTimeoutFilterer(TracesTimeoutFiltererImpl::new(c.clone())),
        })
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
