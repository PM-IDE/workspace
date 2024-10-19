use crate::features::discovery::petri_net::marking::Marking;
use crate::features::discovery::petri_net::place::Place;
use crate::features::discovery::petri_net::transition::Transition;
use std::collections::HashMap;

use super::arc::Arc;

pub type DefaultPetriNet = PetriNet<String, ()>;

#[derive(Debug)]
struct PlaceTransitions {
    incoming_transitions: Vec<u64>,
    outgoing_transitions: Vec<u64>,
}

impl PlaceTransitions {
    pub fn empty() -> Self {
        Self {
            incoming_transitions: Vec::new(),
            outgoing_transitions: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct PetriNet<TTransitionData, TArcData>
where
    TTransitionData: ToString,
{
    places: HashMap<u64, Place>,
    transitions: HashMap<u64, Transition<TTransitionData, TArcData>>,
    places_to_transitions: HashMap<u64, PlaceTransitions>,
    initial_marking: Option<Marking>,
    final_marking: Option<Marking>,
}

impl<TTransitionData, TArcData> PetriNet<TTransitionData, TArcData>
where
    TTransitionData: ToString,
{
    pub fn empty() -> Self {
        Self {
            places: HashMap::new(),
            transitions: HashMap::new(),
            places_to_transitions: HashMap::new(),
            initial_marking: None,
            final_marking: None,
        }
    }

    pub fn add_place(&mut self, place: Place) -> u64 {
        let id = place.id();
        self.places.insert(place.id(), place);
        id
    }

    pub fn all_places(&self) -> Vec<&Place> {
        self.places.values().into_iter().collect()
    }

    pub fn all_transitions(&self) -> Vec<&Transition<TTransitionData, TArcData>> {
        self.transitions.values().into_iter().collect()
    }

    pub fn delete_transition(&mut self, id: &u64) -> Option<Transition<TTransitionData, TArcData>> {
        self.transitions.remove(id)
    }

    pub fn add_transition(&mut self, transition: Transition<TTransitionData, TArcData>) -> u64 {
        let id = transition.id();
        self.transitions.insert(transition.id(), transition);
        id
    }

    pub fn connect_place_to_transition(&mut self, from_place_id: &u64, to_transition_index: &u64, arc_data: Option<TArcData>) {
        self.transitions
            .get_mut(&to_transition_index)
            .unwrap()
            .add_incoming_arc(from_place_id, arc_data);

        self.init_places_transitions(from_place_id);
        self.places_to_transitions
            .get_mut(from_place_id)
            .unwrap()
            .outgoing_transitions
            .push(*to_transition_index);
    }

    fn init_places_transitions(&mut self, place_id: &u64) {
        if !self.places_to_transitions.contains_key(place_id) {
            self.places_to_transitions.insert(*place_id, PlaceTransitions::empty());
        }
    }

    pub fn connect_transition_to_place(&mut self, from_transition_id: &u64, to_place_id: &u64, arc_data: Option<TArcData>) {
        self.transitions
            .get_mut(&from_transition_id)
            .unwrap()
            .add_outgoing_arc(to_place_id, arc_data);

        self.init_places_transitions(to_place_id);
        self.places_to_transitions
            .get_mut(to_place_id)
            .unwrap()
            .incoming_transitions
            .push(*from_transition_id);
    }

    pub fn place(&self, id: &u64) -> &Place {
        self.places.get(id).as_ref().unwrap()
    }

    pub fn transition(&self, id: &u64) -> &Transition<TTransitionData, TArcData> {
        self.transitions.get(id).as_ref().unwrap()
    }

    pub fn set_initial_marking(&mut self, marking: Marking) {
        self.initial_marking = Some(marking)
    }

    pub fn set_final_marking(&mut self, marking: Marking) {
        self.final_marking = Some(marking)
    }

    pub fn initial_marking(&self) -> Option<&Marking> {
        self.initial_marking.as_ref()
    }

    pub fn final_marking(&self) -> Option<&Marking> {
        self.final_marking.as_ref()
    }

    pub fn find_place_id_by_name(&self, name: &str) -> Option<u64> {
        for place in self.places.values() {
            if place.name() == name {
                return Some(place.id());
            }
        }

        None
    }

    pub fn find_transition_by_name(&self, name: &str) -> Option<&Transition<TTransitionData, TArcData>> {
        for transition in self.transitions.values() {
            if transition.name() == name {
                return Some(transition);
            }
        }

        None
    }

    pub fn find_all_transitions_by_name(&self, name: &str) -> Option<Vec<&Transition<TTransitionData, TArcData>>> {
        let mut result = vec![];
        for transition in self.transitions.values() {
            if transition.name() == name {
                result.push(transition)
            }
        }

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    pub fn arc(&self, id: &u64) -> Option<(&Arc<TArcData>, &Transition<TTransitionData, TArcData>)> {
        for transition in self.transitions.values() {
            for arc in transition.outgoing_arcs() {
                if arc.id() == *id {
                    return Some((arc, transition));
                }
            }

            for arc in transition.incoming_arcs() {
                if arc.id() == *id {
                    return Some((arc, transition));
                }
            }
        }

        None
    }

    pub fn get_incoming_transitions(&self, place_id: &u64) -> Vec<&Transition<TTransitionData, TArcData>> {
        self.map_transitions(&self.get_place_transitions(place_id).incoming_transitions)
    }

    fn get_place_transitions(&self, place_id: &u64) -> &PlaceTransitions {
        self.places_to_transitions.get(place_id).unwrap()
    }

    fn map_transitions(&self, ids: &Vec<u64>) -> Vec<&Transition<TTransitionData, TArcData>> {
        ids.iter().map(|id| self.transitions.get(id).unwrap()).collect()
    }

    pub fn get_outgoing_transitions(&self, place_id: &u64) -> Vec<&Transition<TTransitionData, TArcData>> {
        self.map_transitions(&self.get_place_transitions(place_id).outgoing_transitions)
    }
}
