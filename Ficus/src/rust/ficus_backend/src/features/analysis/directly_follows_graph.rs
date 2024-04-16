use crate::features::analysis::event_log_info::EventLogInfo;
use crate::utils::graph::graph::{DefaultGraph, Graph};
use std::collections::HashMap;

pub fn construct_dfg(info: &EventLogInfo) -> DefaultGraph {
    let mut graph = Graph::empty();
    let mut nodes_to_ids = HashMap::new();

    for class in info.all_event_classes() {
        nodes_to_ids.insert(class, graph.add_node(Some(class.to_owned())));
    }

    for class in info.all_event_classes() {
        if let Some(followers) = info.dfg_info().get_followed_events(class) {
            for (follower, count) in followers.iter() {
                let first_id = nodes_to_ids.get(class).unwrap();
                let second_id = nodes_to_ids.get(follower).unwrap();
                graph.connect_nodes(first_id, second_id, Some(count.to_string()));
            }
        }
    }

    graph
}
