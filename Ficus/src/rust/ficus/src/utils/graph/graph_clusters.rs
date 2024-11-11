use crate::utils::graph::graph::{Graph, NodesConnectionData};
use crate::utils::graph::graph_edge::GraphEdge;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;

impl<TNodeData, TEdgeData> Graph<TNodeData, TEdgeData>
where
    TNodeData: ToString,
    TEdgeData: ToString + Display,
{
    pub fn merge_nodes_into_one(
        &mut self,
        cluster_nodes: &HashSet<u64>,
        node_data_merger: impl Fn(Vec<Option<&TNodeData>>) -> Option<TNodeData>,
        edge_data_merger: impl Fn(&Vec<&GraphEdge<TEdgeData>>) -> NodesConnectionData<TEdgeData>,
    ) {
        let nodes_data: Vec<Option<&TNodeData>> = cluster_nodes.iter().map(|id| self.node(id).unwrap().data()).collect();
        let new_node_id = self.add_node(node_data_merger(nodes_data));

        let new_incoming_edges_merged = self.find_incoming_edges(cluster_nodes, &edge_data_merger);
        let new_outgoing_edges_merged = self.find_outgoing_edges(cluster_nodes, &edge_data_merger);

        self.adjust_transitions_for_cluster(
            cluster_nodes,
            new_node_id.clone(),
            new_incoming_edges_merged,
            new_outgoing_edges_merged,
        );
    }

    fn find_incoming_edges(
        &self,
        cluster_nodes: &HashSet<u64>,
        edge_data_merger: &impl Fn(&Vec<&GraphEdge<TEdgeData>>) -> NodesConnectionData<TEdgeData>,
    ) -> HashMap<u64, NodesConnectionData<TEdgeData>> {
        let mut new_incoming_edges = HashMap::new();
        for node in self.all_nodes() {
            let node_id = node.id();
            if !cluster_nodes.contains(&node_id) {
                let mut edges = vec![];
                if let Some(connections) = self.connections.get(&node_id) {
                    for cluster_node in cluster_nodes {
                        if let Some(connection_data) = connections.get(cluster_node) {
                            edges.push(connection_data);
                        }
                    }

                    if edges.len() > 0 {
                        new_incoming_edges.insert(*node_id, edges);
                    }
                }
            }
        }

        let mut new_incoming_edges_merged = HashMap::new();
        for (id, edges_data) in new_incoming_edges {
            new_incoming_edges_merged.insert(id, edge_data_merger(&edges_data));
        }

        new_incoming_edges_merged
    }

    fn find_outgoing_edges(
        &self,
        cluster_nodes: &HashSet<u64>,
        edge_data_merger: &impl Fn(&Vec<&GraphEdge<TEdgeData>>) -> NodesConnectionData<TEdgeData>,
    ) -> HashMap<u64, NodesConnectionData<TEdgeData>> {
        let mut new_outgoing_edges: HashMap<u64, Vec<&GraphEdge<TEdgeData>>> = HashMap::new();
        for cluster_node in cluster_nodes {
            if let Some(connections) = self.connections.get(cluster_node) {
                for (connection, connection_data) in connections {
                    if !cluster_nodes.contains(connection) {
                        let data = connection_data;
                        if let Some(new_edges) = new_outgoing_edges.get_mut(connection) {
                            new_edges.push(data);
                        } else {
                            new_outgoing_edges.insert(connection.to_owned(), vec![data]);
                        }
                    }
                }
            }
        }

        let mut new_outgoing_edges_merged = HashMap::new();
        for (id, edges_data) in new_outgoing_edges {
            new_outgoing_edges_merged.insert(id, edge_data_merger(&edges_data));
        }

        new_outgoing_edges_merged
    }

    fn adjust_transitions_for_cluster(
        &mut self,
        cluster_nodes: &HashSet<u64>,
        new_node_id: u64,
        new_incoming_edges_merged: HashMap<u64, NodesConnectionData<TEdgeData>>,
        new_outgoing_edges_merged: HashMap<u64, NodesConnectionData<TEdgeData>>,
    ) {
        for new_edge in new_incoming_edges_merged {
            if let Some(connections) = self.connections.get_mut(&new_edge.0) {
                let edge = GraphEdge::new(new_edge.0, new_node_id, new_edge.1.weight(), new_edge.1.data);
                connections.insert(new_node_id.clone(), edge);
            }
        }

        let mut new_node_connections = HashMap::new();
        for new_edge in new_outgoing_edges_merged {
            let edge = GraphEdge::new(new_node_id, new_edge.0, new_edge.1.weight, new_edge.1.data);
            new_node_connections.insert(new_edge.0, edge);
        }

        self.connections.insert(new_node_id, new_node_connections);

        for cluster_node in cluster_nodes {
            self.connections.remove(cluster_node);
        }

        for key in self.connections.keys().into_iter().map(|c| *c).collect::<Vec<u64>>() {
            if let Some(connections) = self.connections.get_mut(&key) {
                for cluster_node_id in cluster_nodes {
                    connections.remove(cluster_node_id);
                }
            }
        }

        for cluster_node_id in cluster_nodes {
            self.nodes.remove(cluster_node_id);
        }
    }
}
