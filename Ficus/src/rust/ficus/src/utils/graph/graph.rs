use crate::utils::graph::graph_edge::GraphEdge;
use crate::utils::graph::graph_node::GraphNode;
use crate::utils::references::HeapedOrOwned;
use crate::utils::user_data::user_data::{UserData, UserDataImpl};
use std::fmt::Display;
use std::{collections::HashMap, sync::atomic::AtomicU64};
use getset::{Getters, Setters};

pub(crate) static NEXT_ID: AtomicU64 = AtomicU64::new(0);
pub type DefaultGraph = Graph<HeapedOrOwned<String>, HeapedOrOwned<String>>;

pub struct NodesConnectionData<TEdgeData> {
  pub(super) data: Option<TEdgeData>,
  pub(super) weight: f64,
  pub(super) user_data: Option<UserDataImpl>
}

impl<TEdgeData> NodesConnectionData<TEdgeData> {
  pub fn new(data: Option<TEdgeData>, weight: f64, user_data: Option<UserDataImpl>) -> Self {
    Self { data, weight, user_data }
  }

  pub fn zero_weight(data: Option<TEdgeData>) -> Self {
    Self { data, weight: 0f64, user_data: None }
  }

  pub fn empty() -> Self {
    Self { data: None, weight: 0f64, user_data: None }
  }

  pub fn data(&self) -> Option<&TEdgeData> {
    self.data.as_ref()
  }

  pub fn weight(&self) -> f64 {
    self.weight.clone()
  }
}

#[derive(Debug, Clone)]
pub enum GraphKind {
  Dag
}

#[derive(Debug, Getters, Setters)]
pub struct Graph<TNodeData, TEdgeData>
where
  TNodeData: ToString,
  TEdgeData: ToString,
{
  pub(crate) nodes: HashMap<u64, GraphNode<TNodeData>>,
  pub(crate) connections: HashMap<u64, HashMap<u64, GraphEdge<TEdgeData>>>,
  pub(crate) user_data: UserDataImpl,
  #[getset(get="pub", set="pub")] pub(crate) kind: Option<GraphKind>
}

impl<TNodeData: Clone + ToString, TEdgeData: Clone + ToString> Clone for Graph<TNodeData, TEdgeData> {
  fn clone(&self) -> Self {
    Self {
      nodes: self.nodes.clone(),
      connections: self.connections.clone(),
      user_data: self.user_data.clone(),
      kind: self.kind.clone()
    }
  }
}

impl<TNodeData, TEdgeData> Graph<TNodeData, TEdgeData>
where
  TNodeData: ToString,
  TEdgeData: ToString + Display,
{
  pub fn empty() -> Self {
    Self {
      connections: HashMap::new(),
      nodes: HashMap::new(),
      user_data: UserDataImpl::new(),
      kind: None
    }
  }

  pub fn node(&self, id: &u64) -> Option<&GraphNode<TNodeData>> {
    self.nodes.get(id)
  }

  pub fn node_mut(&mut self, id: &u64) -> Option<&mut GraphNode<TNodeData>> {
    self.nodes.get_mut(id)
  }

  pub fn all_nodes(&self) -> Vec<&GraphNode<TNodeData>> {
    self.nodes.values().into_iter().collect()
  }

  pub fn all_nodes_mut(&mut self) -> Vec<&mut GraphNode<TNodeData>> {
    self.nodes.values_mut().into_iter().collect()
  }

  pub fn all_edges(&self) -> Vec<&GraphEdge<TEdgeData>> {
    let mut edges = vec![];
    for (_, connections) in &self.connections {
      for (_, edge) in connections {
        edges.push(edge)
      }
    }

    edges
  }

  pub fn edge(&self, first_node_id: &u64, second_node_id: &u64) -> Option<&GraphEdge<TEdgeData>> {
    if let Some(connections) = self.connections.get(first_node_id) {
      if let Some(edge) = connections.get(second_node_id) {
        return Some(edge);
      }
    }

    None
  }

  pub fn edge_mut(&mut self, first_node_id: &u64, second_node_id: &u64) -> Option<&mut GraphEdge<TEdgeData>> {
    if let Some(connections) = self.connections.get_mut(first_node_id) {
      if let Some(edge) = connections.get_mut(second_node_id) {
        return Some(edge);
      }
    }

    None
  }

  pub fn add_node(&mut self, node_data: Option<TNodeData>) -> u64 {
    self.add_node_internal(GraphNode::new(node_data))
  }

  pub fn delete_node(&mut self, id: &u64) -> bool {
    self.nodes.remove(id).is_some()
  }

  fn add_node_internal(&mut self, new_node: GraphNode<TNodeData>) -> u64 {
    let id = *new_node.id();
    self.nodes.insert(*new_node.id(), new_node);

    id
  }

  pub fn add_node_with_user_data(&mut self, node_data: Option<TNodeData>, user_data: UserDataImpl) -> u64 {
    self.add_node_internal(GraphNode::new_with_user_data(node_data, user_data))
  }

  pub fn connect_nodes(&mut self, first_node_id: &u64, second_node_id: &u64, connection_data: NodesConnectionData<TEdgeData>) {
    if self.are_nodes_connected(first_node_id, second_node_id) {
      return;
    }

    if let Some(_) = self.nodes.get(first_node_id) {
      if let Some(_) = self.nodes.get(second_node_id) {
        let edge = GraphEdge::new(*first_node_id, *second_node_id, connection_data.weight, connection_data.data, connection_data.user_data);
        if let Some(connections) = self.connections.get_mut(first_node_id) {
          connections.insert(second_node_id.to_owned(), edge);
        } else {
          let new_connections = HashMap::from_iter(vec![(second_node_id.to_owned(), edge)]);
          self.connections.insert(first_node_id.to_owned(), new_connections);
        }
      }
    }
  }

  pub fn are_nodes_connected(&self, first_node_id: &u64, second_node_id: &u64) -> bool {
    if let Some(connections) = self.connections.get(first_node_id) {
      connections.contains_key(second_node_id)
    } else {
      false
    }
  }

  pub fn disconnect_nodes(&mut self, first_node_id: &u64, second_node_id: &u64) -> bool {
    if let Some(connections) = self.connections.get_mut(first_node_id) {
      connections.remove(second_node_id).is_some()
    } else {
      false
    }
  }

  pub fn all_connected_nodes(&self, node_id: &u64) -> Vec<&u64> {
    let mut connected_nodes = match self.connections.get(node_id) {
      None => vec![],
      Some(connections) => connections.keys().collect(),
    };

    for (node_id, connections) in &self.connections {
      if connections.contains_key(node_id) {
        connected_nodes.push(node_id);
      }
    }

    connected_nodes
  }

  pub fn outgoing_nodes(&self, node_id: &u64) -> Vec<&u64> {
    match self.connections.get(node_id) {
      None => vec![],
      Some(outgoing_edges) => outgoing_edges.keys().collect(),
    }
  }

  pub fn incoming_edges(&self, node_id: &u64) -> Vec<&u64> {
    let mut result = vec![];
    for (candidate, connections) in &self.connections {
      if connections.contains_key(node_id) {
        result.push(candidate);
      }
    }

    result
  }

  pub fn serialize_edges_deterministic(&self, add_weight: bool) -> String {
    let get_node_repr = |id| {
      match self.node(id).as_ref().unwrap().data.as_ref() {
        None => "None".to_string(),
        Some(data) => data.to_string()
      }
    };

    let mut serialized_connection = vec![];
    for (from_node, to_nodes) in &self.connections {
      for (to_node, data) in to_nodes {
        let mut edge = format!("[{}]--[{}]", get_node_repr(from_node), get_node_repr(to_node));
        if add_weight {
          edge += format!("({})", data.weight).as_str();
        }

        serialized_connection.push(edge);
      }
    }

    serialized_connection.sort();
    serialized_connection.join("\n")
  }

  pub fn user_data(&self) -> &UserDataImpl {
    &self.user_data
  }

  pub fn user_data_mut(&mut self) -> &mut UserDataImpl {
    &mut self.user_data
  }
}

impl<TNodeData, TEdgeData> Graph<TNodeData, TEdgeData>
where
  TNodeData: ToString + Clone,
  TEdgeData: ToString + Display,
{
  pub fn add_node_from_another_node(&mut self, other_node: &GraphNode<TNodeData>) -> u64 {
    self.add_node_internal(GraphNode::new_with_user_data(other_node.data.clone(), other_node.user_data.clone()))
  }
}