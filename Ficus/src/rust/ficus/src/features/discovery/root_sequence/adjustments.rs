use crate::features::discovery::petri_net::annotations::{PerformanceAnnotationInfo, PerformanceMap, PERFORMANCE_ANNOTATION_INFO_KEY};
use crate::features::discovery::root_sequence::context::DiscoveryContext;
use crate::features::discovery::root_sequence::models::ActivityStartEndTimeData;
use crate::pipelines::keys::context_key::DefaultContextKey;
use crate::pipelines::keys::context_keys::{CORRESPONDING_TRACE_DATA_KEY, SOFTWARE_DATA_KEY, START_END_ACTIVITIES_TIMES_KEY, START_END_ACTIVITY_TIME_KEY};
use crate::utils::graph::graph::{DefaultGraph, NodesConnectionData};
use crate::utils::references::HeapedOrOwned;
use crate::utils::user_data::user_data::UserData;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

pub fn merge_sequences_of_nodes(graph: &mut DefaultGraph, performance_map: Option<PerformanceMap>) {
  let mut processed_nodes = HashSet::new();
  let mut sequences = vec![];

  let check_node = |node_id| {
    graph.incoming_edges(node_id).len() == 1 && graph.outgoing_nodes(node_id).len() == 1
  };

  for node in graph.all_nodes() {
    if processed_nodes.contains(node.id()) || !check_node(node.id()) {
      continue;
    }

    let mut current_sequence = vec![*node.id()];
    let mut current_node_id = node.id();

    loop {
      let prev_node = *graph.incoming_edges(current_node_id).first().unwrap();
      if !check_node(prev_node) {
        break;
      }

      current_sequence.push(*prev_node);
      current_node_id = prev_node;
    }

    current_sequence.reverse();
    current_node_id = node.id();

    loop {
      let next_node = *graph.outgoing_nodes(current_node_id).first().unwrap();
      if !check_node(next_node) {
        break;
      }

      current_sequence.push(*next_node);
      current_node_id = next_node;
    }

    for node in &current_sequence {
      processed_nodes.insert(*node);
    }

    if current_sequence.len() > 1 {
      sequences.push(current_sequence);
    }
  }

  for current_sequence in sequences {
    let first_node = current_sequence.first().unwrap();
    let last_node = current_sequence.last().unwrap();

    let start_node = **graph.incoming_edges(first_node).first().unwrap();
    let end_node = **graph.outgoing_nodes(last_node).first().unwrap();

    let start_node_edge_weight = graph.edge(&start_node, first_node).unwrap().weight();
    let end_node_edge_weight = graph.edge(last_node, &end_node).unwrap().weight();

    graph.disconnect_nodes(&start_node, first_node);
    graph.disconnect_nodes(last_node, &end_node);

    let label = current_sequence.iter().map(|id| id.to_string()).collect::<Vec<String>>().join("\n");
    let added_node_id = graph.add_node(Some(HeapedOrOwned::Owned(label)));

    let software_data = extract_user_data_from(&current_sequence, &graph, &SOFTWARE_DATA_KEY);
    graph.node_mut(&added_node_id).unwrap().user_data_mut().put_concrete(SOFTWARE_DATA_KEY.key(), software_data);

    let trace_data = extract_user_data_from(&current_sequence, &graph, &CORRESPONDING_TRACE_DATA_KEY);
    graph.node_mut(&added_node_id).unwrap().user_data_mut().put_concrete(CORRESPONDING_TRACE_DATA_KEY.key(), trace_data);

    let activities_times = collect_start_end_time_activities_data(&current_sequence, graph);
    graph.node_mut(&added_node_id).unwrap().user_data_mut().put_concrete(START_END_ACTIVITIES_TIMES_KEY.key(), activities_times);

    graph.connect_nodes(&start_node, &added_node_id, NodesConnectionData::new(None, start_node_edge_weight));
    graph.connect_nodes(&added_node_id, &end_node, NodesConnectionData::new(None, end_node_edge_weight));

    if let Some(performance_map) = performance_map.as_ref() {
      put_performance_annotation_info(&start_node, first_node, (&start_node, &added_node_id), performance_map, graph);
      put_performance_annotation_info(last_node, &end_node, (&added_node_id, &end_node), performance_map, graph);
    }

    for i in 0..current_sequence.len() - 1 {
      graph.disconnect_nodes(&current_sequence[i], &current_sequence[i + 1]);
      graph.delete_node(&current_sequence[i]);
      graph.delete_node(&current_sequence[i + 1]);
    }
  }
}

fn put_performance_annotation_info(first_node: &u64, second_node: &u64, edge: (&u64, &u64), performance_map: &PerformanceMap, graph: &mut DefaultGraph) {
  let last_node_name = graph.node(first_node).unwrap().data().unwrap().clone();
  let end_node_name = graph.node(second_node).unwrap().data().unwrap().clone();

  if let Some((sum, count)) = performance_map.get(&(last_node_name, end_node_name)) {
    let performance_info = PerformanceAnnotationInfo::SumAndCount(*sum, *count);
    graph.edge_mut(edge.0, edge.1).unwrap().user_data_mut().put_concrete(PERFORMANCE_ANNOTATION_INFO_KEY.key(), performance_info);
  }
}

fn extract_user_data_from<T: Clone>(nodes: &Vec<u64>, graph: &DefaultGraph, key: &DefaultContextKey<Vec<T>>) -> Vec<T> {
  let mut result = vec![];
  for node in nodes {
    if let Some(data) = graph.node(node).unwrap().user_data().concrete(key.key()) {
      result.extend_from_slice(data.iter().map(|s| (*s).clone()).collect::<Vec<T>>().as_slice())
    }
  }

  result
}

fn collect_start_end_time_activities_data(nodes: &Vec<u64>, graph: &DefaultGraph) -> Vec<ActivityStartEndTimeData> {
  let mut times = vec![];
  for node in nodes {
    if let Some(data) = graph.node(node).unwrap().user_data().concrete(START_END_ACTIVITY_TIME_KEY.key()) {
      times.push(data.clone());
    }
  }

  times
}

pub fn adjust_weights_and_connections<T: PartialEq + Clone + Debug>(context: &DiscoveryContext<T>, log: &Vec<Vec<T>>, graph: &mut DefaultGraph) {
  let name_extractor = context.name_extractor();
  let mut df_relations = HashMap::new();

  for trace in log {
    for i in 0..trace.len() - 1 {
      let first_name = name_extractor(&trace[i]);
      let second_name = name_extractor(&trace[i + 1]);

      *df_relations.entry((Some(first_name), Some(second_name))).or_insert(0) += 1usize;
    }
  }

  let mut new_edges_weights = HashMap::new();
  let mut nodes_to_disconnect = vec![];
  for edge in graph.all_edges() {
    let from_name = graph.node(edge.from_node()).unwrap().data().cloned();
    let to_name = graph.node(edge.to_node()).unwrap().data().cloned();

    let edge_key = (*edge.from_node(), *edge.to_node());
    if let Some(df_count) = df_relations.get(&(from_name, to_name)) {
      new_edges_weights.insert(edge_key, *df_count as f64);
    } else {
      nodes_to_disconnect.push(edge_key)
    }
  }

  for (edge_key, new_weight) in new_edges_weights {
    graph.edge_mut(&edge_key.0, &edge_key.1).unwrap().weight = new_weight;
  }

  for (from_node, to_node) in &nodes_to_disconnect {
    graph.disconnect_nodes(from_node, to_node);
  }
}
