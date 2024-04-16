use std::{cell::RefCell, rc::Rc};

use super::{
    event::event_hasher::EventHasher,
    trace::trace::{Trace, TraceInfo},
};
use crate::event_log::core::event::event::Event;

pub trait EventLog: Clone {
    type TEvent: Event + 'static;
    type TTraceInfo: TraceInfo + 'static;
    type TTrace: Trace<TEvent = Self::TEvent, TTraceInfo = Self::TTraceInfo> + 'static;

    fn empty() -> Self;

    fn traces(&self) -> &Vec<Rc<RefCell<Self::TTrace>>>;
    fn push(&mut self, trace: Rc<RefCell<Self::TTrace>>);

    fn to_raw_vector(&self) -> Vec<Vec<String>>;

    fn to_hashes_event_log<THasher>(&self, hasher: &THasher) -> Vec<Vec<u64>>
    where
        THasher: EventHasher<Self::TEvent>;

    fn filter_events_by<TPred>(&mut self, predicate: TPred)
    where
        TPred: Fn(&Self::TEvent) -> bool;

    fn mutate_events<TMutator>(&mut self, mutator: TMutator)
    where
        TMutator: Fn(&mut Self::TEvent);

    fn filter_traces(&mut self, predicate: &impl Fn(&Self::TTrace, &usize) -> bool);
}
