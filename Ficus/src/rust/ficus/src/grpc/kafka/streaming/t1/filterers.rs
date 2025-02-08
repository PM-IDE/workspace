use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::grpc::kafka::streaming::t1::configs::{EventsTimeoutConfiguration, TracesQueueConfiguration, TracesTimeoutConfiguration};
use chrono::Utc;
use std::ops::Sub;

#[derive(Clone)]
pub enum T1LogFilterer {
    None,
    EventsTimeoutFilterer(EventsTimeoutFiltererImpl),
    TracesTimeoutFilterer(TracesTimeoutFiltererImpl),
    TracesQueueFilterer(TracesQueueFiltererImpl)
}

impl T1LogFilterer {
    pub fn filter(&self, log: &mut XesEventLogImpl) {
        match self {
            T1LogFilterer::None => {}
            T1LogFilterer::EventsTimeoutFilterer(filterer) => filterer.filter(log),
            T1LogFilterer::TracesTimeoutFilterer(filterer) => filterer.filter(log),
            T1LogFilterer::TracesQueueFilterer(filterer) => filterer.filter(log)
        }
    }
}

#[derive(Clone)]
pub struct EventsTimeoutFiltererImpl {
    config: EventsTimeoutConfiguration,
}

impl EventsTimeoutFiltererImpl {
    pub fn new(config: EventsTimeoutConfiguration) -> Self {
        Self { config }
    }

    pub fn filter(&self, log: &mut XesEventLogImpl) {
        let current_stamp = Utc::now();
        let timeout = self.config.timeout_ms() as i64;
        log.filter_events_by(|e| current_stamp.sub(e.timestamp()).num_milliseconds() > timeout);
    }
}

#[derive(Clone)]
pub struct TracesTimeoutFiltererImpl {
    config: TracesTimeoutConfiguration,
}

impl TracesTimeoutFiltererImpl {
    pub fn new(config: TracesTimeoutConfiguration) -> Self {
        Self { config }
    }

    pub fn filter(&self, log: &mut XesEventLogImpl) {
        let current_stamp = Utc::now();
        let timeout = self.config.timeout_ms() as i64;
        log.filter_traces(&|t, _| {
            let last_event = t.events().last().unwrap().borrow();
            current_stamp.sub(last_event.timestamp()).num_milliseconds() > timeout
        });
    }
}

#[derive(Clone)]
pub struct TracesQueueFiltererImpl {
    config: TracesQueueConfiguration
}

impl TracesQueueFiltererImpl {
    pub fn new(config: TracesQueueConfiguration) -> Self {
        Self {
            config
        }
    }

    pub fn filter(&self, log: &mut XesEventLogImpl) {
        let traces_count = log.traces().len() as u64;
        if traces_count <= self.config.queue_capacity() {
            return;
        }

        let traces_to_remove = traces_count - self.config.queue_capacity();
        log.filter_traces(&|_, index| (*index as u64) < traces_to_remove);
    }
}