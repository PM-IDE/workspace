use super::arc::PetriNetArc;
use crate::features::discovery::petri_net::{
  marking::{Marking, SingleMarking},
  place::Place,
  transition::{Transition, silent_transition_name},
};
use std::{collections::HashMap, sync::Arc};

pub type DefaultPetriNet = PetriNet<Arc<str>, ()>;

#[derive(Debug, Default)]
struct PlaceTransitions {
  pub(super) incoming_transitions: Vec<u64>,
  pub(super) outgoing_transitions: Vec<u64>,
}

pub const EMPTY_PLACE_TRANSITIONS: &PlaceTransitions = &PlaceTransitions {
  incoming_transitions: vec![],
  outgoing_transitions: vec![],
};

#[derive(Debug, Default)]
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
  pub fn add_place(&mut self, place: Place) -> u64 {
    let id = place.id();
    self.places.insert(place.id(), place);
    id
  }

  pub fn all_places(&self) -> Vec<&Place> {
    self.places.values().collect()
  }

  pub fn all_places_sorted(&self) -> Vec<&Place> {
    let mut places = self.all_places();
    places.sort_by_key(|p| p.id());

    places
  }

  pub fn all_transitions(&self) -> Vec<&Transition<TTransitionData, TArcData>> {
    self.transitions.values().collect()
  }

  pub fn all_transitions_sorted(&self) -> Vec<&Transition<TTransitionData, TArcData>> {
    let mut transitions = self.all_transitions();
    transitions.sort_by_key(|t| t.id());

    transitions
  }

  pub fn delete_transition(&mut self, id: &u64) -> Option<Transition<TTransitionData, TArcData>> {
    self.transitions.remove(id)
  }

  pub fn add_transition(&mut self, transition: Transition<TTransitionData, TArcData>) -> u64 {
    let id = transition.id();
    self.transitions.insert(transition.id(), transition);
    id
  }

  pub fn connect_place_to_transition(&mut self, from_place_id: &u64, to_transition_id: &u64, arc_data: Option<TArcData>) {
    self
      .transitions
      .get_mut(to_transition_id)
      .unwrap()
      .add_incoming_arc(from_place_id, arc_data);

    self.init_places_transitions(from_place_id);
    self
      .places_to_transitions
      .get_mut(from_place_id)
      .unwrap()
      .outgoing_transitions
      .push(*to_transition_id);
  }

  fn init_places_transitions(&mut self, place_id: &u64) {
    if !self.places_to_transitions.contains_key(place_id) {
      self.places_to_transitions.insert(*place_id, PlaceTransitions::default());
    }
  }

  pub fn connect_transition_to_place(&mut self, from_transition_id: &u64, to_place_id: &u64, arc_data: Option<TArcData>) {
    self
      .transitions
      .get_mut(from_transition_id)
      .unwrap()
      .add_outgoing_arc(to_place_id, arc_data);

    self.init_places_transitions(to_place_id);
    self
      .places_to_transitions
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
    self.transitions.values().find(|&transition| transition.name() == name)
  }

  pub fn find_all_transitions_by_name(&self, name: &str) -> Option<Vec<&Transition<TTransitionData, TArcData>>> {
    let mut result = vec![];
    for transition in self.transitions.values() {
      if transition.name() == name {
        result.push(transition)
      }
    }

    if result.is_empty() { None } else { Some(result) }
  }

  pub fn arc(&self, id: &u64) -> Option<(&PetriNetArc<TArcData>, &Transition<TTransitionData, TArcData>)> {
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
    self.places_to_transitions.get(place_id).unwrap_or(EMPTY_PLACE_TRANSITIONS)
  }

  fn map_transitions(&self, ids: &[u64]) -> Vec<&Transition<TTransitionData, TArcData>> {
    ids.iter().map(|id| self.transitions.get(id).unwrap()).collect()
  }

  pub fn get_outgoing_transitions(&self, place_id: &u64) -> Vec<&Transition<TTransitionData, TArcData>> {
    self.map_transitions(&self.get_place_transitions(place_id).outgoing_transitions)
  }
}

impl<Tt: Clone + ToString, Ta: Clone> PetriNet<Tt, Ta> {
  pub fn create_remapped_net(&self) -> Self {
    #[derive(Default)]
    struct Remapping {
      remapping: HashMap<u64, u64>,
    }

    impl Remapping {
      pub fn get_id(&mut self, id: u64) -> u64 {
        let len = self.remapping.len();
        *self.remapping.entry(id).or_insert(len as u64)
      }
    }

    let mut remapping = Remapping::default();

    let remap_marking = |marking: &Marking, r: &mut Remapping| Marking {
      active_places: marking
        .active_places
        .iter()
        .map(|p| SingleMarking {
          place_id: r.get_id(p.place_id),
          tokens_count: p.tokens_count,
        })
        .collect(),
    };

    let remap_arc = |a: &PetriNetArc<Ta>, r: &mut Remapping| PetriNetArc {
      id: r.get_id(a.id),
      data: a.data.clone(),
      tokens_count: a.tokens_count,
      place_id: r.get_id(a.place_id),
    };

    let remap_places = |places: &[&Place], r: &mut Remapping| {
      places
        .into_iter()
        .map(|p| {
          let id = r.get_id(p.id);
          let p = Place {
            id,
            name: p.name.clone(),
            user_data: p.user_data.clone(),
          };

          (id, p)
        })
        .collect()
    };

    let remap_transitions = |transitions: &[&Transition<Tt, Ta>], r: &mut Remapping| {
      transitions
        .into_iter()
        .map(|t| {
          let id = r.get_id(t.id);
          let t = Transition {
            id,
            name: match t.is_silent() {
              false => t.name.clone(),
              true => silent_transition_name(id),
            },
            data: t.data.clone(),
            silent_transition: t.silent_transition,
            incoming_arcs: t.incoming_arcs.iter().map(|a| remap_arc(a, r)).collect(),
            outgoing_arcs: t.outgoing_arcs.iter().map(|a| remap_arc(a, r)).collect(),
          };

          (id, t)
        })
        .collect()
    };

    let remap_places_to_transitions = |pt: &HashMap<u64, PlaceTransitions>, r: &mut Remapping| {
      let mut vec = pt.into_iter().collect::<Vec<_>>();
      vec.sort_by_key(|(id, _)| *id);

      vec
        .into_iter()
        .map(|(id, t)| {
          let pt = PlaceTransitions {
            outgoing_transitions: t.outgoing_transitions.iter().map(|&id| r.get_id(id)).collect(),
            incoming_transitions: t.incoming_transitions.iter().map(|&id| r.get_id(id)).collect(),
          };

          (r.get_id(*id), pt)
        })
        .collect()
    };

    Self {
      places: remap_places(&self.all_places_sorted(), &mut remapping),
      transitions: remap_transitions(&self.all_transitions_sorted(), &mut remapping),
      final_marking: self.final_marking.as_ref().map(|m| remap_marking(m, &mut remapping)),
      initial_marking: self.initial_marking.as_ref().map(|m| remap_marking(m, &mut remapping)),
      places_to_transitions: remap_places_to_transitions(&self.places_to_transitions, &mut remapping),
    }
  }
}

pub struct PetriNetInfo {
  pub places_count: usize,
  pub transition_count: usize,
  pub arcs_count: usize,
}

impl PetriNetInfo {
  pub fn create<TTransitionData: ToString, TArcData>(net: &PetriNet<TTransitionData, TArcData>) -> PetriNetInfo {
    PetriNetInfo {
      places_count: net.places.len(),
      transition_count: net.transitions.len(),
      arcs_count: net
        .places
        .iter()
        .map(|(p_id, _)| net.get_incoming_transitions(p_id).len() + net.get_outgoing_transitions(p_id).len())
        .sum(),
    }
  }
}
