use crate::utils::graph::{
  graph::{DefaultGraph, Graph},
  graph_edge::GraphEdge,
  graph_node::GraphNode,
};
use std::{collections::HashMap, fmt::Display, sync::Arc};

impl<TNodeData, TEdgeData> Graph<TNodeData, TEdgeData>
where
  TNodeData: ToString,
  TEdgeData: ToString + Display,
{
  pub fn to_default_graph(self) -> DefaultGraph {
    DefaultGraph {
      nodes: self.to_default_graph_nodes(),
      connections: self.to_default_graph_connections(),
      user_data: Default::default(),
      kind: self.kind().clone(),
    }
  }

  #[rustfmt::skip]
  fn to_default_graph_nodes(&self) -> HashMap<u64, GraphNode<Arc<str>>> {
        self.nodes.iter().map(|pair| {
            (
                *pair.0,
                GraphNode {
                    id: pair.1.id.to_owned(),
                    data: pair.1.data.as_ref().map(|data| Arc::from(data.to_string())),
                    user_data: pair.1.user_data.clone()
                },
            )
        }).collect()
    }

  #[rustfmt::skip]
  fn to_default_graph_connections(&self) -> HashMap<u64, HashMap<u64, GraphEdge<Arc<str>>>> {
        self.connections.iter().map(|pair| {
            (
                *pair.0,
                pair.1.iter().map(|pair| {
                    (
                        *pair.0,
                        GraphEdge::new(
                            pair.1.from_node,
                            pair.1.to_node,
                            pair.1.weight,
                            pair.1.data().as_ref().map(|data| Arc::from(data.to_string())),
                            Some(pair.1.user_data.clone())
                        )
                    )
                }).collect(),
            )
        }).collect()
    }
}
