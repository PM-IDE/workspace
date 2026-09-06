use crate::{
  features::{
    discovery::petri_net::{
      marking::{Marking, SingleMarking},
      petri_net::DefaultPetriNet,
      place::Place,
      transition::Transition,
    },
    mutations::mutations::{ARTIFICIAL_END_EVENT_NAME, ARTIFICIAL_START_EVENT_NAME},
  },
  utils::graph::graph::DefaultGraph,
};
use std::{
  collections::HashMap,
  fmt::{Display, Formatter},
};

pub enum GraphToPetriNetConversionError {
  NodeDataIsEmpty(u64),
}

impl Display for GraphToPetriNetConversionError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      GraphToPetriNetConversionError::NodeDataIsEmpty(id) => f.write_str(&format!("node {id} does not have data")),
    }
  }
}

pub fn convert_ecfg_to_petri_net(graph: &DefaultGraph) -> Result<DefaultPetriNet, GraphToPetriNetConversionError> {
  if graph.all_nodes().is_empty() {
    return Ok(Default::default());
  }

  let mut petri_net = DefaultPetriNet::default();

  let mut nodes_data = HashMap::new();

  for node in graph.all_nodes() {
    let name = node
      .data
      .clone()
      .ok_or_else(|| GraphToPetriNetConversionError::NodeDataIsEmpty(node.id))?;
    let t_id = petri_net.add_transition(Transition::empty(name.clone(), Some(name.clone())));

    let in_place = petri_net.add_place(Place::with_name(format!("IN_{}", name)));
    let out_place = petri_net.add_place(Place::with_name(format!("OUT_{}", name)));

    petri_net.connect_place_to_transition(&in_place, &t_id, None);
    petri_net.connect_transition_to_place(&t_id, &out_place, None);

    let name = name.as_ref();
    if name == ARTIFICIAL_START_EVENT_NAME {
      petri_net.set_initial_marking(Marking::new(vec![SingleMarking::new(in_place, 1)]));
    } else if name == ARTIFICIAL_END_EVENT_NAME {
      petri_net.set_final_marking(Marking::new(vec![SingleMarking::new(out_place, 1)]));
    }

    nodes_data.insert(node.id, (in_place, out_place));
  }

  for edge in graph.all_edges() {
    let from_place = nodes_data[edge.from_node()].1;
    let to_place = nodes_data[edge.to_node()].0;

    let s_t = petri_net.add_transition(Transition::silent());

    petri_net.connect_place_to_transition(&from_place, &s_t, None);
    petri_net.connect_transition_to_place(&s_t, &to_place, None);
  }

  Ok(petri_net)
}
