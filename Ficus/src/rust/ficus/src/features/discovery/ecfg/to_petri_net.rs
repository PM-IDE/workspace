use crate::{
  features::discovery::petri_net::{petri_net::DefaultPetriNet, place::Place, transition::Transition},
  utils::graph::graph::DefaultGraph,
};
use std::{collections::HashMap, sync::Arc};

pub fn convert_to_petri_net(graph: &DefaultGraph) -> Result<DefaultPetriNet, ()> {
  if graph.all_nodes().is_empty() {
    return Ok(Default::default());
  }

  let mut petri_net = DefaultPetriNet::default();

  let mut next_place_id = 0;
  let mut nodes_data = HashMap::new();

  for node in graph.all_nodes() {
    let name = Arc::clone(node.data.as_ref().expect("must have name for all transitions"));
    let t_id = petri_net.add_transition(Transition::empty(name.clone(), Some(name.clone())));

    let in_place = petri_net.add_place(Place::with_name(format!("{next_place_id}")));
    let out_place = petri_net.add_place(Place::with_name(format!("{}", next_place_id + 1)));

    petri_net.connect_place_to_transition(&in_place, &t_id, None);
    petri_net.connect_transition_to_place(&t_id, &out_place, None);

    nodes_data.insert(node.id, (in_place, out_place));
    next_place_id += 2;
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
