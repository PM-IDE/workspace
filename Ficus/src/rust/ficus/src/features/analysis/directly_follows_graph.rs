use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::features::analysis::event_log_info::EventLogInfo;
use crate::utils::graph::graph::{DefaultGraph, Graph, NodesConnectionData};
use std::collections::HashMap;

pub fn construct_dfg(info: &EventLogInfo) -> DefaultGraph {
    let mut graph = Graph::empty();
    let mut classes_to_node_ids = HashMap::new();

    for class in info.all_event_classes() {
        classes_to_node_ids.insert(class, graph.add_node(Some(class.to_owned())));
    }

    for class in info.all_event_classes() {
        if let Some(followers) = info.dfg_info().get_followed_events(class) {
            for (follower, count) in followers.iter() {
                let first_id = classes_to_node_ids.get(class).unwrap();
                let second_id = classes_to_node_ids.get(follower).unwrap();
                let connection_data = NodesConnectionData::new(Some(count.to_string()), *count as f64);

                graph.connect_nodes(first_id, second_id, connection_data);
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
        let mut last_seen_events: HashMap<Option<String>, String> = HashMap::new();

        for event in trace.events() {
            let event = event.borrow();

            if !classes_to_nodes_ids.contains_key(event.name()) {
                let node_id = graph.add_node(Some(event.name().to_owned()));
                classes_to_nodes_ids.insert(event.name().to_owned(), node_id);
            }

            let attribute_value = match event.payload_map() {
                None => None,
                Some(map) => match map.get(attribute) {
                    None => None,
                    Some(value) => Some(value.to_string_repr().to_owned()),
                },
            };

            match last_seen_events.get(&attribute_value) {
                Some(last_seen_class) => {
                    let first_node_id = classes_to_nodes_ids.get(last_seen_class).unwrap();
                    let second_node_id = classes_to_nodes_ids.get(event.name()).unwrap();

                    *dfg_map.entry((*first_node_id, *second_node_id)).or_insert(0u64) += 1;
                    *last_seen_events.get_mut(&attribute_value).unwrap() = event.name().to_owned();
                }
                None => {
                    last_seen_events.insert(attribute_value, event.name().to_owned());
                }
            };
        }
    }

    for ((first_node_id, second_node_id), count) in dfg_map {
        let connection_data = NodesConnectionData::new(Some(count.to_string()), count as f64);
        graph.connect_nodes(&first_node_id, &second_node_id, connection_data);
    }

    graph
}
