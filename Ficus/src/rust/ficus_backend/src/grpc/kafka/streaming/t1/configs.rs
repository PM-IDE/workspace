use crate::{
  ficus_proto::{GrpcT1StreamingConfiguration, grpc_t1_streaming_configuration::Configuration},
  grpc::kafka::streaming::t1::processors::T1StreamingProcessor,
};
use ficus::features::streaming::t1::{
  configs::{EventsTimeoutConfiguration, TracesQueueConfiguration, TracesTimeoutConfiguration},
  filterers::{EventsTimeoutFiltererImpl, T1LogFilterer, TracesQueueFiltererImpl, TracesTimeoutFiltererImpl},
};

pub enum T1StreamingConfiguration {
  EventsTimeout(EventsTimeoutConfiguration),
  TracesTimeout(TracesTimeoutConfiguration),
  TracesQueue(TracesQueueConfiguration),
}

impl T1StreamingConfiguration {
  pub fn new(grpc_config: &GrpcT1StreamingConfiguration) -> Option<Self> {
    grpc_config.configuration.as_ref().map(|c| match c {
      Configuration::EventsTimeout(et) => {
        T1StreamingConfiguration::EventsTimeout(EventsTimeoutConfiguration::new(et.events_timeout_ms as u64))
      }
      Configuration::TracesTimeout(tt) => {
        T1StreamingConfiguration::TracesTimeout(TracesTimeoutConfiguration::new(tt.traces_timeout_ms as u64))
      }
      Configuration::TracesQueueConfiguration(tq) => {
        T1StreamingConfiguration::TracesQueue(TracesQueueConfiguration::new(tq.queue_capacity as u64))
      }
    })
  }

  pub fn create_processor(&self) -> T1StreamingProcessor {
    T1StreamingProcessor::new(match self {
      T1StreamingConfiguration::EventsTimeout(c) => T1LogFilterer::EventsTimeoutFilterer(EventsTimeoutFiltererImpl::new(c.clone())),
      T1StreamingConfiguration::TracesTimeout(c) => T1LogFilterer::TracesTimeoutFilterer(TracesTimeoutFiltererImpl::new(c.clone())),
      T1StreamingConfiguration::TracesQueue(c) => T1LogFilterer::TracesQueueFilterer(TracesQueueFiltererImpl::new(c.clone())),
    })
  }
}
