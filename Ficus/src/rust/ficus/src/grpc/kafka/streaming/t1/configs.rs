use crate::ficus_proto::grpc_t1_streaming_configuration::Configuration;
use crate::ficus_proto::{GrpcT1EventsTimeBasedCaching, GrpcT1StreamingConfiguration, GrpcT1TraceTimeBasedCaching};
use crate::grpc::kafka::streaming::t1::filterers::{
  EventsTimeoutFiltererImpl, T1LogFilterer, TracesQueueFiltererImpl, TracesTimeoutFiltererImpl,
};
use crate::grpc::kafka::streaming::t1::processors::T1StreamingProcessor;

pub enum T1StreamingConfiguration {
  EventsTimeout(EventsTimeoutConfiguration),
  TracesTimeout(TracesTimeoutConfiguration),
  TracesQueue(TracesQueueConfiguration),
}

impl T1StreamingConfiguration {
  pub fn new(grpc_config: &GrpcT1StreamingConfiguration) -> Option<Self> {
    match grpc_config.configuration.as_ref() {
      None => None,
      Some(c) => Some(match c {
        Configuration::EventsTimeout(et) => {
          T1StreamingConfiguration::EventsTimeout(EventsTimeoutConfiguration::new(et.events_timeout_ms as u64))
        }
        Configuration::TracesTimeout(tt) => {
          T1StreamingConfiguration::TracesTimeout(TracesTimeoutConfiguration::new(tt.traces_timeout_ms as u64))
        }
        Configuration::TracesQueueConfiguration(tq) => {
          T1StreamingConfiguration::TracesQueue(TracesQueueConfiguration::new(tq.queue_capacity as u64))
        }
      }),
    }
  }

  pub fn create_processor(&self) -> T1StreamingProcessor {
    T1StreamingProcessor::new(match self {
      T1StreamingConfiguration::EventsTimeout(c) => T1LogFilterer::EventsTimeoutFilterer(EventsTimeoutFiltererImpl::new(c.clone())),
      T1StreamingConfiguration::TracesTimeout(c) => T1LogFilterer::TracesTimeoutFilterer(TracesTimeoutFiltererImpl::new(c.clone())),
      T1StreamingConfiguration::TracesQueue(c) => T1LogFilterer::TracesQueueFilterer(TracesQueueFiltererImpl::new(c.clone())),
    })
  }
}

#[derive(Clone)]
pub struct EventsTimeoutConfiguration {
  timeout_ms: u64,
}

impl EventsTimeoutConfiguration {
  pub fn new(timeout_ms: u64) -> Self {
    Self { timeout_ms }
  }

  pub fn timeout_ms(&self) -> u64 {
    self.timeout_ms.clone()
  }
}

#[derive(Clone)]
pub struct TracesTimeoutConfiguration {
  timeout_ms: u64,
}

impl TracesTimeoutConfiguration {
  pub fn new(timeout_ms: u64) -> Self {
    Self { timeout_ms }
  }

  pub fn timeout_ms(&self) -> u64 {
    self.timeout_ms.clone()
  }
}

#[derive(Clone)]
pub struct TracesQueueConfiguration {
  queue_capacity: u64,
}

impl TracesQueueConfiguration {
  pub fn new(queue_capacity: u64) -> Self {
    Self { queue_capacity }
  }

  pub fn queue_capacity(&self) -> u64 {
    self.queue_capacity.clone()
  }
}
