use lazycell::LazyCell;

use crate::{
    event_log::core::trace::trace::{TraceEventsPositions, TraceInfo},
    utils::hash_map_utils::{add_to_list_in_map, increase_in_map},
};

use super::event::Event;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use log::debug;

#[derive(Debug)]
pub struct EventsHolder<TEvent>
where
    TEvent: Event,
{
    events: Vec<Rc<RefCell<TEvent>>>,
    events_sequence_info: LazyCell<EventSequenceInfo>,
    events_positions: LazyCell<EventsPositions>,
}

impl<TEvent> Clone for EventsHolder<TEvent>
where
    TEvent: Event,
{
    fn clone(&self) -> Self {
        Self {
            events: (&self.events)
                .into_iter()
                .map(|ptr| Rc::new(RefCell::new(ptr.borrow().clone())))
                .collect(),

            events_sequence_info: LazyCell::new(),
            events_positions: LazyCell::new(),
        }
    }
}

impl<TEvent> EventsHolder<TEvent>
where
    TEvent: Event,
{
    pub fn empty() -> Self {
        Self {
            events: vec![],
            events_sequence_info: LazyCell::new(),
            events_positions: LazyCell::new(),
        }
    }

    pub fn new(events: Vec<Rc<RefCell<TEvent>>>) -> Self {
        Self {
            events,
            events_sequence_info: LazyCell::new(),
            events_positions: LazyCell::new(),
        }
    }

    pub fn events(&self) -> &Vec<Rc<RefCell<TEvent>>> {
        &self.events
    }

    pub fn push(&mut self, event: Rc<RefCell<TEvent>>) {
        self.events.push(event);
    }

    pub fn remove_events_by<TPred>(&mut self, predicate: TPred)
    where
        TPred: Fn(&TEvent) -> bool,
    {
        let mut new_events = vec![];
        let events = &self.events;
        for index in 0..events.len() {
            if !predicate(&events[index].borrow()) {
                new_events.push(events[index].clone())
            } else {
                debug!("Removing event at index {}: {:?}", index, events[index].borrow())
            }
        }

        self.events = new_events;
    }

    pub fn to_names_vec(&self) -> Vec<String> {
        let mut names = Vec::new();
        for event in &self.events {
            names.push(event.borrow().name().to_owned());
        }

        names
    }

    pub fn mutate_events<TMutator>(&mut self, mutator: TMutator)
    where
        TMutator: Fn(&mut TEvent),
    {
        for event in &self.events {
            mutator(&mut event.borrow_mut());
        }
    }

    pub fn get_or_create_event_sequence_info(&mut self) -> &EventSequenceInfo {
        if !self.events_sequence_info.filled() {
            self.events_sequence_info.fill(EventSequenceInfo::new(self)).ok();
        }

        self.events_sequence_info.borrow().unwrap()
    }

    pub fn get_or_create_events_positions(&mut self) -> &EventsPositions {
        if !self.events_positions.filled() {
            self.events_positions.fill(EventsPositions::new(self)).ok();
        }

        self.events_positions.borrow().unwrap()
    }

    pub fn events_mut(&mut self) -> &mut Vec<Rc<RefCell<TEvent>>> {
        &mut self.events
    }
}

#[derive(Debug)]
pub struct EventSequenceInfo {
    events_counts: HashMap<String, usize>,
    events_count: usize,
}

impl TraceInfo for EventSequenceInfo {
    fn events_counts(&self) -> &HashMap<String, usize> {
        &self.events_counts
    }

    fn events_count(&self) -> usize {
        self.events_count
    }
}

impl EventSequenceInfo {
    fn new<TEvent>(events_holder: &EventsHolder<TEvent>) -> EventSequenceInfo
    where
        TEvent: Event,
    {
        let mut events_counts = HashMap::new();
        for event in events_holder.events() {
            increase_in_map(&mut events_counts, event.borrow().name());
        }

        EventSequenceInfo {
            events_counts,
            events_count: events_holder.events().len(),
        }
    }
}

#[derive(Debug)]
pub struct EventsPositions {
    positions: HashMap<String, Vec<usize>>,
}

impl EventsPositions {
    pub fn new<TEvent>(events: &EventsHolder<TEvent>) -> EventsPositions
    where
        TEvent: Event,
    {
        let mut positions = HashMap::new();
        let mut index = 0;

        for event in events.events() {
            add_to_list_in_map(&mut positions, event.borrow().name(), index);
            index += 1;
        }

        EventsPositions { positions }
    }
}

impl TraceEventsPositions for EventsPositions {
    fn event_positions(&self, event_class: &String) -> Option<&Vec<usize>> {
        self.positions.get(event_class)
    }
}
