use crate::utils::context_key::DefaultContextKey;
use crate::utils::graph::graph::{Graph, NodesConnectionData};
use crate::utils::user_data::user_data::UserData;
use enum_display_derive::Display;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::{Debug, Display};

#[derive(Display)]
pub enum GraphsMergingError {
  MissingStartNode,
  MissingEndNode,
}

const START_NODE_ID: &'static str = "START_NODE_ID";
const END_NODE_ID: &'static str = "END_NODE_ID";

lazy_static!(
  pub static ref START_NODE_ID_KEY: DefaultContextKey<u64> = DefaultContextKey::new(START_NODE_ID);
  pub static ref END_NODE_ID_KEY: DefaultContextKey<u64> = DefaultContextKey::new(END_NODE_ID);
);

pub fn merge_graphs<TNodeData: Display + Clone, TEdgeData: Display + Clone>(
  graphs: &Vec<Graph<TNodeData, TEdgeData>>
) -> Result<Graph<TNodeData, TEdgeData>, GraphsMergingError> {
  let mut merged_graph = Graph::empty();

  let merged_start_node_id = merged_graph.add_node(None);
  let merged_end_node_id = merged_graph.add_node(None);

  for graph in graphs {
    let start_node_id = match graph.user_data().concrete(START_NODE_ID_KEY.key()) {
      None => return Err(GraphsMergingError::MissingStartNode),
      Some(id) => *id
    };

    let end_node_id = match graph.user_data().concrete(END_NODE_ID_KEY.key()) {
      None => return Err(GraphsMergingError::MissingEndNode),
      Some(id) => *id
    };

    let mut ids_map = HashMap::new();
    for node in graph.all_nodes() {
      ids_map.insert(*node.id(), merged_graph.add_node_with_user_data(node.data().cloned(), node.user_data().clone()));
    }

    for edge in graph.all_edges() {
      let first_id = ids_map.get(edge.from_node()).unwrap();
      let second_id = ids_map.get(edge.to_node()).unwrap();
      let data = NodesConnectionData::new(edge.data().as_ref().cloned(), *edge.weight(), Some(edge.user_data().clone()));

      merged_graph.connect_nodes(first_id, second_id, data);
    }

    merged_graph.connect_nodes(&merged_start_node_id, &start_node_id, NodesConnectionData::empty());
    merged_graph.connect_nodes(&end_node_id, &merged_end_node_id, NodesConnectionData::empty());
  }

  Ok(merged_graph)
}
