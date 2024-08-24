use std::collections::HashSet;

use crate::event_log::core::{event::event::Event, event_log::EventLog, trace::trace::Trace};

use super::{petri_net::DefaultPetriNet, place::Place};

#[derive(Debug, Clone)]
pub struct Marking {
    active_places: Vec<SingleMarking>,
}

impl Marking {
    pub fn new(single_markings: Vec<SingleMarking>) -> Self {
        Self {
            active_places: single_markings,
        }
    }

    pub fn active_places(&self) -> &Vec<SingleMarking> {
        &self.active_places
    }
}

#[derive(Debug, Clone)]
pub struct SingleMarking {
    place_id: u64,
    tokens_count: usize,
}

impl SingleMarking {
    pub fn new(place_id: u64, tokens_count: usize) -> Self {
        Self { place_id, tokens_count }
    }

    pub fn place_id(&self) -> u64 {
        self.place_id
    }
    pub fn tokens_count(&self) -> usize {
        self.tokens_count
    }
}

pub fn ensure_initial_marking(log: &impl EventLog, petri_net: &mut DefaultPetriNet) {
    let mut start_transitions = HashSet::new();
    let mut end_transitions = HashSet::new();

    for trace in log.traces() {
        let trace = trace.borrow();
        let events = trace.events();
        let first_event = events.first().unwrap().borrow();
        let start_transition = first_event.name();

        let second_event = events.last().unwrap().borrow();
        let end_transition = second_event.name();

        if let Some(start_transition) = petri_net.find_transition_by_name(start_transition) {
            start_transitions.insert(start_transition.id());
        }

        if let Some(end_transition) = petri_net.find_transition_by_name(end_transition) {
            end_transitions.insert(end_transition.id());
        }
    }

    let start_place_id = petri_net.add_place(Place::with_name("Start".to_owned()));
    let end_place_id = petri_net.add_place(Place::with_name("End".to_owned()));

    for transition_id in start_transitions {
        petri_net.connect_place_to_transition(&start_place_id, &transition_id, None);
    }

    for transition_id in end_transitions {
        petri_net.connect_transition_to_place(&transition_id, &end_place_id, None);
    }

    petri_net.set_initial_marking(Marking::new(vec![SingleMarking::new(start_place_id, 1)]));
}
