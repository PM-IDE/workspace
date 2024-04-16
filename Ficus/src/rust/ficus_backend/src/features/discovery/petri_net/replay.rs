use std::collections::{HashMap, VecDeque};

use crate::event_log::core::{event::event::Event, event_log::EventLog, trace::trace::Trace};

use super::{marking::Marking, petri_net::DefaultPetriNet, transition::Transition};

#[derive(Debug)]
pub struct ReplayState {
    markings: HashMap<u64, usize>,
    fired_transitions: Vec<u64>,
}

impl ReplayState {
    pub fn fired_transitions(&self) -> &Vec<u64> {
        &self.fired_transitions
    }
}

impl ReplayState {
    pub fn new_raw(markings: HashMap<u64, usize>, fired_transitions: Vec<u64>) -> Self {
        Self {
            markings,
            fired_transitions,
        }
    }

    pub fn new(initial_marking: Marking) -> Self {
        Self {
            fired_transitions: vec![],
            markings: initial_marking
                .active_places()
                .iter()
                .map(|c| (c.place_id(), c.tokens_count()))
                .collect(),
        }
    }

    pub fn handle_transition(state: &ReplayState, net: &DefaultPetriNet, transition: &str) -> Option<Vec<(bool, ReplayState)>> {
        let candidates = Self::find_transitions_to_fire(net, transition);

        let mut new_states = vec![];
        for candidate_transition in candidates {
            let mut can_fire = true;
            for arc in candidate_transition.incoming_arcs() {
                if !state.markings.contains_key(&arc.place_id()) {
                    can_fire = false;
                    break;
                }
            }

            if can_fire {
                let mut new_markings = state.markings.clone();
                for arc in candidate_transition.incoming_arcs() {
                    let place_id = &arc.place_id();
                    let count = new_markings[place_id];
                    let new_count = count - arc.tokens_count();
                    if new_count <= 0 {
                        new_markings.remove(place_id);
                    } else {
                        *new_markings.get_mut(place_id).unwrap() = new_count;
                    }
                }

                for arc in candidate_transition.outgoing_arcs() {
                    let place_id = &arc.place_id();
                    if let Some(count) = new_markings.get(place_id) {
                        *new_markings.get_mut(place_id).unwrap() = count + arc.tokens_count();
                    } else {
                        new_markings.insert(*place_id, *arc.tokens_count());
                    }
                }

                let mut new_fired_transitions = state.fired_transitions.clone();
                new_fired_transitions.push(candidate_transition.id());

                let new_state = ReplayState::new_raw(new_markings, new_fired_transitions);
                new_states.push((!candidate_transition.is_silent(), new_state));
            }
        }

        Some(new_states)
    }

    fn find_transitions_to_fire<'a>(net: &'a DefaultPetriNet, symbol: &'a str) -> Vec<&'a Transition<String, ()>> {
        let mut result = vec![];
        for transition in net.all_transitions() {
            if transition.name() == symbol {
                result.push(transition);
            }
        }

        for transition in net.all_transitions() {
            if *transition.is_silent() {
                result.push(transition);
            }
        }

        result
    }
}

pub fn replay_petri_net(log: &impl EventLog, net: &DefaultPetriNet) -> Option<Vec<Option<ReplayState>>> {
    let mut result = vec![];
    for trace in log.traces() {
        let marking = match net.initial_marking() {
            Some(marking) => marking.clone(),
            None => return None,
        };

        let trace = trace.borrow();
        let mut stack = VecDeque::new();
        stack.push_back((0usize, ReplayState::new(marking)));

        loop {
            if stack.len() == 0 {
                result.push(None);
                break;
            }

            let current_state = stack.pop_back().unwrap();
            let events = trace.events();
            if current_state.0 >= events.len() {
                result.push(Some(current_state.1));
                break;
            }

            let transition = trace.events().get(current_state.0).unwrap();
            let new_states = ReplayState::handle_transition(&current_state.1, net, transition.borrow().name());

            if let Some(new_states) = new_states {
                for new_state in new_states {
                    //if we fired silent transition dont consume symbol from trace
                    let new_index = current_state.0 + if new_state.0 { 1 } else { 0 };
                    stack.push_back((new_index, new_state.1));
                }
            }
        }
    }

    Some(result)
}
