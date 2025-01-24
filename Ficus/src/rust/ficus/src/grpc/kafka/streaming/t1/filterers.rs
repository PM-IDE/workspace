use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::grpc::kafka::streaming::t1::configs::{EventsTimeoutConfiguration, TracesTimeoutConfiguration};
use chrono::Utc;
use std::ops::Sub;

#[derive(Clone)]
pub(in crate::grpc) enum T1LogFilterer {
    None,
    EventsTimeoutFilterer(EventsTimeoutFiltererImpl),
    TracesTimeoutFilterer(TracesTimeoutFiltererImpl),
}

impl T1LogFilterer {
    pub fn filter(&self, log: &mut XesEventLogImpl) {
        match self {
            T1LogFilterer::None => {}
            T1LogFilterer::EventsTimeoutFilterer(filterer) => filterer.filter(log),
            T1LogFilterer::TracesTimeoutFilterer(filterer) => filterer.filter(log),
        }
    }
}

#[derive(Clone)]
pub(in crate::grpc) struct EventsTimeoutFiltererImpl {
    config: EventsTimeoutConfiguration,
}

impl EventsTimeoutFiltererImpl {
    pub fn new(config: EventsTimeoutConfiguration) -> Self {
        Self { config }
    }

    pub fn filter(&self, log: &mut XesEventLogImpl) {
        let current_stamp = Utc::now();
        let timeout = self.config.timeout_ms() as i64;
        log.filter_events_by(|e| e.timestamp().sub(current_stamp).num_milliseconds() > timeout);
    }
}

#[derive(Clone)]
pub(in crate::grpc) struct TracesTimeoutFiltererImpl {
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
            last_event.timestamp().sub(current_stamp).num_milliseconds() > timeout
        });
    }
}
