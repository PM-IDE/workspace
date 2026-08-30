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
  let start_place = Place::with_name(ARTIFICIAL_START_EVENT_NAME.to_string());
  let start_place = petri_net.add_place(start_place);

  let mut q = VecDeque::new();
  q.push_back((start_place, start_node));

  let mut nodes_to_places = HashMap::<u64, u64>::default();

  while let Some((from_place, node)) = q.pop_front() {
    let node = graph.node(&node).expect("must be in graph");
    let name = Arc::clone(node.data.as_ref().expect("must have name for all transitions"));
    let t_id = if let Some(t) = petri_net
      .get_outgoing_transitions(&from_place)
      .iter()
      .find(|t| t.name() == name.as_ref())
    {
      t.id()
    } else {
      println!("{from_place:?}, {name:?}, {:?}", petri_net.get_outgoing_transitions(&from_place));
      petri_net.add_transition(Transition::empty(name.clone(), false, None))
    };

    let to_place = nodes_to_places.get(&node.id()).copied().unwrap_or_else(|| {
      let next_id = petri_net.all_places().len();
      petri_net.add_place(Place::with_name(format!("From-{name}-{next_id}")))
    });

    if !petri_net.transition(&t_id).outgoing_arcs().iter().any(|a| a.place_id() == to_place) {
      petri_net.connect_place_to_transition(&from_place, &t_id, None);
      petri_net.connect_transition_to_place(&t_id, &to_place, None);

      nodes_to_places.insert(node.id, to_place);
    }

    for &out_node in graph.outgoing_nodes(node.id()) {
      q.push_back((to_place, out_node));
    }
  }

  Ok(petri_net)
}
