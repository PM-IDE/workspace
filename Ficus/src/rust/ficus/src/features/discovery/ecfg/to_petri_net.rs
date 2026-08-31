use crate::{
  features::{
    discovery::petri_net::{petri_net::DefaultPetriNet, place::Place, transition::Transition},
    mutations::mutations::ARTIFICIAL_START_EVENT_NAME,
  },
  utils::graph::graph::DefaultGraph,
};
use std::{
  collections::{HashMap, VecDeque},
  sync::Arc,
};
use std::fmt::format;

pub fn convert_to_petri_net(graph: &DefaultGraph) -> Result<DefaultPetriNet, ()> {
  if graph.all_nodes().is_empty() {
    return Ok(Default::default());
  }

  let start_node = graph
    .nodes
    .iter()
    .find(|n| graph.incoming_edges(n.0).is_empty())
    .map(|n| n.0)
    .copied()
    .expect("must contain start node");

  let mut petri_net = DefaultPetriNet::default();

  let mut next_place_id = 0;
  let mut nodes_data = HashMap::new();

  for node in graph.all_nodes() {
    let name = Arc::clone(node.data.as_ref().expect("must have name for all transitions"));
    let t_id = petri_net.add_transition(Transition::empty(name.clone(), false, None));

    let in_place = petri_net.add_place(Place::with_name(format!("{next_place_id}")));
    let out_place = petri_net.add_place(Place::with_name(format!("{}", next_place_id + 1)));

    petri_net.connect_place_to_transition(&in_place, &t_id, None);
    petri_net.connect_transition_to_place(&t_id, &out_place, None);

    nodes_data.insert(node.id, (in_place, out_place));
    next_place_id += 2;
  }

  let mut s_t_idx = 0;
  for edge in graph.all_edges() {
    let from_place = nodes_data[edge.from_node()].1;
    let to_place = nodes_data[edge.to_node()].0;

    let name = Arc::from(format!("{s_t_idx}"));
    let s_t = petri_net.add_transition(Transition::empty(name, true, None));

    petri_net.connect_place_to_transition(&from_place, &s_t, None);
    petri_net.connect_transition_to_place(&s_t, &to_place, None);

    s_t_idx += 1;
  }

  Ok(petri_net)
}
