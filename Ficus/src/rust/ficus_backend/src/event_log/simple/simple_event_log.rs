use std::{cell::RefCell, collections::HashMap, rc::Rc};

use chrono::{DateTime, Duration, Utc};

use crate::utils::user_data::user_data::UserDataOwner;
use crate::{
    event_log::core::{
        event::{
            event::{Event, EventPayloadValue},
            event_base::EventBase,
            event_hasher::EventHasher,
            events_holder::{EventSequenceInfo, EventsHolder, EventsPositions},
        },
        event_log::EventLog,
        trace::{trace::Trace, traces_holder::EventLogBase},
    },
    utils::user_data::user_data::UserDataImpl,
};

#[derive(Debug)]
pub struct SimpleEventLog {
    base: EventLogBase<SimpleTrace>,
}

impl SimpleEventLog {
    pub fn new(raw_event_log: &Vec<Vec<&str>>) -> SimpleEventLog {
        let mut traces = Vec::new();
        for raw_trace in raw_event_log {
            traces.push(Rc::new(RefCell::new(SimpleTrace::new(raw_trace))));
        }

        SimpleEventLog {
            base: EventLogBase::new(traces),
        }
    }

    pub fn push(&mut self, trace: Rc<RefCell<<SimpleEventLog as EventLog>::TTrace>>) {
        self.base.push(trace);
    }
}

impl Clone for SimpleEventLog {
    fn clone(&self) -> Self {
        Self { base: self.base.clone() }
    }
}

impl UserDataOwner for SimpleEventLog {
    fn user_data(&self) -> &UserDataImpl {
        self.base.user_data()
    }

    fn user_data_mut(&mut self) -> &mut UserDataImpl {
        self.base.user_data_mut()
    }
}

impl EventLog for SimpleEventLog {
    type TEvent = SimpleEvent;
    type TTraceInfo = EventSequenceInfo;
    type TTrace = SimpleTrace;

    fn empty() -> Self {
        Self {
            base: EventLogBase::empty(),
        }
    }

    fn traces(&self) -> &Vec<Rc<RefCell<Self::TTrace>>> {
        &self.base.get_traces()
    }

    fn push(&mut self, trace: Rc<RefCell<Self::TTrace>>) {
        self.base.push(trace);
    }

    fn to_raw_vector(&self) -> Vec<Vec<String>> {
        self.base.to_raw_vector()
    }

    fn to_hashes_event_log<THasher>(&self, hasher: &THasher) -> Vec<Vec<u64>>
    where
        THasher: EventHasher<Self::TEvent>,
    {
        self.base.to_hashes_vectors(hasher)
    }

    fn filter_events_by<TPred>(&mut self, predicate: TPred)
    where
        TPred: Fn(&Self::TEvent) -> bool,
    {
        self.base.filter_events_by(predicate);
    }

    fn mutate_events<TMutator>(&mut self, mutator: TMutator)
    where
        TMutator: Fn(&mut Self::TEvent),
    {
        self.base.mutate_events(mutator);
    }

    fn filter_traces(&mut self, predicate: &impl Fn(&Self::TTrace, &usize) -> bool) {
        self.base.filter_traces(predicate);
    }
}

#[derive(Debug)]
pub struct SimpleTrace {
    events_holder: EventsHolder<SimpleEvent>,
}

impl Clone for SimpleTrace {
    fn clone(&self) -> Self {
        Self {
            events_holder: self.events_holder.clone(),
        }
    }
}

impl Trace for SimpleTrace {
    type TEvent = SimpleEvent;
    type TTraceInfo = EventSequenceInfo;
    type TTracePositions = EventsPositions;

    fn empty() -> Self {
        Self {
            events_holder: EventsHolder::empty(),
        }
    }

    fn events(&self) -> &Vec<Rc<RefCell<Self::TEvent>>> {
        &self.events_holder.events()
    }

    fn events_mut(&mut self) -> &mut Vec<Rc<RefCell<Self::TEvent>>> {
        self.events_holder.events_mut()
    }

    fn push(&mut self, event: Rc<RefCell<Self::TEvent>>) {
        self.events_holder.push(event);
    }

    fn to_names_vec(&self) -> Vec<String> {
        self.events_holder.to_names_vec()
    }

    fn get_or_create_trace_info(&mut self) -> &Self::TTraceInfo {
        self.events_holder.get_or_create_event_sequence_info()
    }

    fn get_or_create_events_positions(&mut self) -> &Self::TTracePositions {
        self.events_holder.get_or_create_events_positions()
    }

    fn remove_events_by<TPred>(&mut self, predicate: TPred)
    where
        TPred: Fn(&Self::TEvent) -> bool,
    {
        self.events_holder.remove_events_by(predicate);
    }

    fn mutate_events<TMutator>(&mut self, mutator: TMutator)
    where
        TMutator: Fn(&mut Self::TEvent),
    {
        self.events_holder.mutate_events(mutator);
    }
}

const TRACE_EVENT_START_DATE: DateTime<Utc> = DateTime::<Utc>::MIN_UTC;

impl SimpleTrace {
    pub fn empty() -> Self {
        Self {
            events_holder: EventsHolder::empty(),
        }
    }

    pub fn new(raw_trace: &Vec<&str>) -> Self {
        let mut events = Vec::new();
        let mut current_date = TRACE_EVENT_START_DATE;
        for raw_event in raw_trace {
            events.push(Rc::new(RefCell::new(SimpleEvent::new(raw_event.to_string(), current_date))));
            current_date = current_date + Duration::seconds(1);
        }

        Self {
            events_holder: EventsHolder::new(events),
        }
    }

    pub fn push(&mut self, event: Rc<RefCell<<SimpleTrace as Trace>::TEvent>>) {
        self.events_holder.push(event);
    }
}

#[derive(Debug)]
pub struct SimpleEvent {
    event_base: EventBase,
}

impl SimpleEvent {}

impl Event for SimpleEvent {
    fn name(&self) -> &String {
        &self.event_base.name
    }

    fn timestamp(&self) -> &DateTime<Utc> {
        &self.event_base.timestamp
    }

    fn payload_map(&self) -> Option<&HashMap<String, EventPayloadValue>> {
        panic!("Not supported")
    }

    fn ordered_payload(&self) -> Vec<(&String, &EventPayloadValue)> {
        panic!("Not supported")
    }

    fn user_data(&mut self) -> &mut UserDataImpl {
        self.event_base.user_data_holder.get_mut()
    }

    fn set_name(&mut self, new_name: String) {
        self.event_base.name = Rc::new(Box::new(new_name));
    }

    fn set_timestamp(&mut self, new_timestamp: DateTime<Utc>) {
        self.event_base.timestamp = new_timestamp;
    }

    fn add_or_update_payload(&mut self, _: String, _: EventPayloadValue) {
        panic!("Not supported")
    }

    fn new(name: String, timestamp: DateTime<Utc>) -> Self {
        Self {
            event_base: EventBase::new(Rc::new(Box::new(name)), timestamp),
        }
    }

    fn new_with_min_date(name: String) -> Self {
        Self::new(name, DateTime::<Utc>::MIN_UTC)
    }

    fn new_with_max_date(name: String) -> Self {
        Self::new(name, DateTime::<Utc>::MAX_UTC)
    }

    fn name_pointer(&self) -> &Rc<Box<String>> {
        &self.event_base.name
    }
}

impl Clone for SimpleEvent {
    fn clone(&self) -> Self {
        Self {
            event_base: self.event_base.clone(),
        }
    }
}
