use crate::utils::user_data::user_data::UserDataImpl;
use std::{cell::RefCell, collections::HashSet, rc::Rc};

use crate::event_log::core::event::{event::Event, event_hasher::EventHasher};
use crate::utils::user_data::user_data::UserData;

use super::trace::Trace;

#[derive(Debug)]
pub struct EventLogBase<TTrace>
where
    TTrace: Trace,
{
    traces: Vec<Rc<RefCell<TTrace>>>,
    user_data: UserDataImpl,
}

impl<TTrace> Clone for EventLogBase<TTrace>
where
    TTrace: Trace,
{
    fn clone(&self) -> Self {
        Self {
            traces: (&self.traces)
                .into_iter()
                .map(|ptr| Rc::new(RefCell::new(ptr.borrow().clone())))
                .collect(),
            user_data: self.user_data.clone(),
        }
    }
}

impl<TTrace> EventLogBase<TTrace>
where
    TTrace: Trace,
{
    pub fn empty() -> Self {
        Self {
            traces: vec![],
            user_data: UserDataImpl::new(),
        }
    }

    pub fn new(traces: Vec<Rc<RefCell<TTrace>>>) -> Self {
        Self {
            traces,
            user_data: UserDataImpl::new(),
        }
    }

    pub fn get_traces(&self) -> &Vec<Rc<RefCell<TTrace>>> {
        &self.traces
    }

    pub fn push(&mut self, trace: Rc<RefCell<TTrace>>) {
        self.traces.push(trace);
    }

    pub fn filter_events_by<TPred>(&mut self, predicate: TPred)
    where
        TPred: Fn(&TTrace::TEvent) -> bool,
    {
        let traces = &mut self.traces;
        for index in (0..traces.len()).rev() {
            traces[index].borrow_mut().remove_events_by(&predicate);
            if traces[index].borrow().events().is_empty() {
                traces.remove(index);
            }
        }
    }

    pub fn mutate_events<TMutator>(&mut self, mutator: TMutator)
    where
        TMutator: Fn(&mut TTrace::TEvent),
    {
        for trace in &self.traces {
            trace.borrow_mut().mutate_events(&mutator);
        }
    }

    pub fn to_hashes_vectors<THasher>(&self, hasher: &THasher) -> Vec<Vec<u64>>
    where
        THasher: EventHasher<TTrace::TEvent>,
    {
        let mut hashes = Vec::new();
        for trace in &self.traces {
            let mut trace_hashes = Vec::new();
            for event in trace.borrow().events() {
                trace_hashes.push(hasher.hash(&event.borrow()));
            }

            hashes.push(trace_hashes);
        }

        hashes
    }

    pub fn filter_traces(&mut self, predicate: &impl Fn(&TTrace, &usize) -> bool) {
        let mut to_remove = HashSet::new();
        for index in 0..self.traces.len() {
            if predicate(&self.traces.get(index).unwrap().borrow(), &index) {
                to_remove.insert(index);
            }
        }

        for index in (0..self.traces.len()).rev() {
            if to_remove.contains(&index) {
                self.traces.remove(index);
            }
        }
    }

    pub fn to_raw_vector(&self) -> Vec<Vec<String>> {
        let mut raw_log = Vec::new();
        for trace in self.get_traces() {
            let mut events = Vec::new();
            for event in trace.borrow().events() {
                events.push(event.borrow().name().to_owned());
            }

            raw_log.push(events);
        }

        raw_log
    }

    pub fn user_data(&self) -> &UserDataImpl {
        &self.user_data
    }

    pub fn user_data_mut(&mut self) -> &mut UserDataImpl {
        &mut self.user_data
    }
}
