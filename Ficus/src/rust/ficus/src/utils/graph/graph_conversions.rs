use crate::utils::graph::graph::{DefaultGraph, Graph};
use crate::utils::graph::graph_edge::GraphEdge;
use crate::utils::graph::graph_node::GraphNode;
use crate::utils::references::HeapedOrOwned;
use std::collections::HashMap;
use std::fmt::Display;

impl<TNodeData, TEdgeData> Graph<TNodeData, TEdgeData>
where
  TNodeData: ToString,
  TEdgeData: ToString + Display,
{
  pub fn to_default_graph(self) -> DefaultGraph {
    DefaultGraph {
      nodes: self.to_default_graph_nodes(),
      connections: self.to_default_graph_connections(),
    }
  }

  #[rustfmt::skip]
    fn to_default_graph_nodes(&self) -> HashMap<u64, GraphNode<HeapedOrOwned<String>>> {
        self.nodes.iter().map(|pair| {
            (
                *pair.0,
                GraphNode {
                    id: pair.1.id.to_owned(),
                    data: match &pair.1.data {
                        None => None,
                        Some(data) => Some(HeapedOrOwned::Owned(data.to_string())),
                    },
                    user_data: pair.1.user_data.clone()
                },
            )
        }).collect()
    }

  #[rustfmt::skip]
    fn to_default_graph_connections(&self) -> HashMap<u64, HashMap<u64, GraphEdge<HeapedOrOwned<String>>>> {
        self.connections.iter().map(|pair| {
            (
                *pair.0,
                pair.1.iter().map(|pair| {
                    (
                        *pair.0,
                        GraphEdge::new(
                            pair.1.first_node_id, 
                            pair.1.second_node_id,
                            pair.1.weight,
                            match pair.1.data() {
                                None => None,
                                Some(data) => Some(HeapedOrOwned::Owned(data.to_string())),
                            }
                        )
                    )
                }).collect(),
            )
        }).collect()
    }
}
