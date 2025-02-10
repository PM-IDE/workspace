use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::event_log::core::event::event::{Event, EventPayloadValue};

pub trait Trace: Clone {
    type TEvent: Event;
    type TTraceInfo: TraceInfo;
    type TTracePositions: TraceEventsPositions;

    fn empty() -> Self;

    fn events(&self) -> &Vec<Rc<RefCell<Self::TEvent>>>;
    fn events_mut(&mut self) -> &mut Vec<Rc<RefCell<Self::TEvent>>>;

    fn push(&mut self, event: Rc<RefCell<Self::TEvent>>);

    fn to_names_vec(&self) -> Vec<String>;
    fn get_or_create_trace_info(&mut self) -> &Self::TTraceInfo;
    fn get_or_create_events_positions(&mut self) -> &Self::TTracePositions;

    fn remove_events_by<TPred>(&mut self, predicate: TPred)
    where
        TPred: Fn(&Self::TEvent) -> bool;

    fn mutate_events<TMutator>(&mut self, mutator: TMutator)
    where
        TMutator: Fn(&mut Self::TEvent);

    fn metadata(&self) -> &HashMap<String, EventPayloadValue>;
    fn metadata_mut(&mut self) -> &mut HashMap<String, EventPayloadValue>;
}

pub trait TraceInfo {
    fn events_counts(&self) -> &HashMap<String, usize>;
    fn events_count(&self) -> usize;
}

pub trait TraceEventsPositions {
    fn event_positions(&self, event_class: &String) -> Option<&Vec<usize>>;
}
