use crate::utils::graph::graph_edge::GraphEdge;
use crate::utils::graph::graph_node::GraphNode;
use std::fmt::Display;
use std::{collections::HashMap, sync::atomic::AtomicU64};

pub(crate) static NEXT_ID: AtomicU64 = AtomicU64::new(0);
pub type DefaultGraph = Graph<String, String>;

pub struct NodesConnectionData<TEdgeData> {
    data: Option<TEdgeData>,
    weight: f64,
}

impl<TEdgeData> NodesConnectionData<TEdgeData> {
    pub fn new(data: Option<TEdgeData>, weight: f64) -> Self {
        Self { data, weight }
    }

    pub fn zero_weight(data: Option<TEdgeData>) -> Self {
        Self { data, weight: 0f64 }
    }

    pub fn empty() -> Self {
        Self { data: None, weight: 0f64 }
    }

    pub fn data(&self) -> Option<&TEdgeData> {
        self.data.as_ref()
    }

    pub fn weight(&self) -> f64 {
        self.weight.clone()
    }
}

pub struct Graph<TNodeData, TEdgeData>
where
    TNodeData: ToString,
    TEdgeData: ToString,
{
    pub(crate) nodes: HashMap<u64, GraphNode<TNodeData>>,
    pub(crate) connections: HashMap<u64, HashMap<u64, NodesConnectionData<TEdgeData>>>,
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
        }
    }

    pub fn node(&self, id: &u64) -> Option<&GraphNode<TNodeData>> {
        self.nodes.get(id)
    }

    pub fn all_nodes(&self) -> Vec<&GraphNode<TNodeData>> {
        (&self.nodes).values().into_iter().collect()
    }

    pub fn all_edges(&self) -> Vec<GraphEdge<&TEdgeData>> {
        let mut edges = vec![];
        for (first, connections) in &self.connections {
            for (second, data) in connections {
                edges.push(GraphEdge::new(*first, *second, data.weight, data.data.as_ref()))
            }
        }

        edges
    }

    pub fn add_node(&mut self, node_data: Option<TNodeData>) -> u64 {
        let new_node = GraphNode::new(node_data);
        let id = *new_node.id();
        self.nodes.insert(*new_node.id(), new_node);

        id
    }

    pub fn connect_nodes(&mut self, first_node_id: &u64, second_node_id: &u64, connection_data: NodesConnectionData<TEdgeData>) {
        if self.are_nodes_connected(first_node_id, second_node_id) {
            return;
        }

        if let Some(_) = self.nodes.get(first_node_id) {
            if let Some(_) = self.nodes.get(second_node_id) {
                if let Some(connections) = self.connections.get_mut(first_node_id) {
                    connections.insert(second_node_id.to_owned(), connection_data);
                } else {
                    let new_connections = HashMap::from_iter(vec![(second_node_id.to_owned(), connection_data)]);
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
}
