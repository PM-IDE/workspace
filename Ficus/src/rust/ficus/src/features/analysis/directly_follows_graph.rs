use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::features::analysis::log_info::event_log_info::EventLogInfo;
use crate::utils::graph::graph::{DefaultGraph, Graph, NodesConnectionData};
use crate::utils::references::HeapedOrOwned;
use std::collections::HashMap;
use log::warn;

pub fn construct_dfg(info: &dyn EventLogInfo) -> DefaultGraph {
    let mut graph = Graph::empty();
    let mut classes_to_node_ids = HashMap::new();

    for class in info.all_event_classes() {
        let node_data = Some(HeapedOrOwned::Owned(class.to_owned()));
        classes_to_node_ids.insert(class, graph.add_node(node_data));
    }

    for class in info.all_event_classes() {
        if let Some(followers) = info.dfg_info().get_followed_events(class) {
            for (follower, count) in followers.iter() {
                if let Some(first_id) = classes_to_node_ids.get(class) {
                    if let Some(second_id) = classes_to_node_ids.get(follower) {
                        let data = Some(HeapedOrOwned::Owned(count.to_string()));
                        let connection_data = NodesConnectionData::new(data, *count as f64);

                        graph.connect_nodes(first_id, second_id, connection_data);
                    } else {
                        warn!("Failed to get graph node for follower {}", follower);
                    }
                } else {
                    warn!("Failed to get graph node for class {}", class)
                }
            }
        }
    }

    graph
}

pub fn construct_dfg_by_attribute(log: &XesEventLogImpl, attribute: &str) -> DefaultGraph {
    let mut graph = Graph::empty();
    let mut classes_to_nodes_ids = HashMap::new();
    let mut dfg_map = HashMap::new();

    for trace in log.traces() {
        let trace = trace.borrow();
        let mut last_seen_events: HashMap<Option<HeapedOrOwned<String>>, HeapedOrOwned<String>> = HashMap::new();

        for event in trace.events() {
            let event = event.borrow();

            if !classes_to_nodes_ids.contains_key(event.name()) {
                let node_data = Some(HeapedOrOwned::Heaped(event.name_pointer().to_owned()));
                let node_id = graph.add_node(node_data);
                classes_to_nodes_ids.insert(event.name().to_owned(), node_id);
            }

            let attribute_value = match event.payload_map() {
                None => None,
                Some(map) => match map.get(attribute) {
                    None => None,
                    Some(value) => Some(value.to_string_repr().to_owned()),
                },
            };

            let event_name_boxed = HeapedOrOwned::Heaped(event.name_pointer().clone());
            match last_seen_events.get(&attribute_value) {
                Some(last_seen_class) => {
                    let first_node_id = classes_to_nodes_ids.get(last_seen_class.as_str()).unwrap();
                    let second_node_id = classes_to_nodes_ids.get(event.name()).unwrap();

                    *dfg_map.entry((*first_node_id, *second_node_id)).or_insert(0u64) += 1;
                    *last_seen_events.get_mut(&attribute_value).unwrap() = event_name_boxed;
                }
                None => {
                    last_seen_events.insert(attribute_value, event_name_boxed);
                }
            };
        }
    }

    for ((first_node_id, second_node_id), count) in dfg_map {
        let edge_data = Some(HeapedOrOwned::Owned(count.to_string()));
        let connection_data = NodesConnectionData::new(edge_data, count as f64);

        graph.connect_nodes(&first_node_id, &second_node_id, connection_data);
    }

    graph
}
