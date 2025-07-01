use crate::features::discovery::petri_net::annotations::{PerformanceAnnotationInfo, PerformanceMap, PERFORMANCE_ANNOTATION_INFO_KEY};
use crate::features::discovery::root_sequence::context::DiscoveryContext;
use crate::features::discovery::root_sequence::context_keys::{EDGE_SOFTWARE_DATA, EDGE_SOFTWARE_DATA_KEY, NODE_CORRESPONDING_TRACE_DATA_KEY, NODE_INNER_GRAPH_KEY, NODE_SOFTWARE_DATA_KEY, NODE_START_END_ACTIVITIES_TIMES_KEY, NODE_START_END_ACTIVITY_TIME_KEY};
use crate::features::discovery::root_sequence::discovery::{replay_sequence_with_history, EVENT_UNIQUE_ID_KEY};
use crate::features::discovery::root_sequence::models::{ActivityStartEndTimeData, DiscoverRootSequenceGraphError, EventCoordinates, EventWithUniqueId, NodeAdditionalDataContainer};
use crate::utils::context_key::DefaultContextKey;
use crate::utils::graph::graph::{DefaultGraph, NodesConnectionData};
use crate::utils::references::HeapedOrOwned;
use crate::utils::user_data::user_data::UserData;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

pub fn merge_sequences_of_nodes(graph: &mut DefaultGraph, performance_map: Option<PerformanceMap>) {
  for current_sequence in discover_sequences_to_merge(graph) {
    merge_nodes_sequence(current_sequence, performance_map.as_ref(), graph);
  }
}

fn discover_sequences_to_merge(graph: &DefaultGraph) -> Vec<Vec<u64>> {
  let mut processed_nodes = HashSet::new();
  let mut sequences = vec![];

  let check_node = |node_id| {
    graph.incoming_edges(node_id).len() == 1 && graph.outgoing_nodes(node_id).len() == 1
  };

  enum EnumerationDirection {
    Left,
    Right,
  }

  let iterate_nodes = |mut node_id: u64, direction: EnumerationDirection, current_sequence: &mut Vec<u64>| {
    loop {
      let next_node = match direction {
        EnumerationDirection::Left => *graph.incoming_edges(&node_id).first().unwrap(),
        EnumerationDirection::Right => *graph.outgoing_nodes(&node_id).first().unwrap()
      };

      if !check_node(next_node) {
        return;
      }

      current_sequence.push(*next_node);
      node_id = *next_node;
    }
  };

  for node in graph.all_nodes() {
    if processed_nodes.contains(node.id()) || !check_node(node.id()) {
      continue;
    }

    let mut current_sequence = vec![*node.id()];

    iterate_nodes(*node.id(), EnumerationDirection::Left, &mut current_sequence);

    current_sequence.reverse();

    iterate_nodes(*node.id(), EnumerationDirection::Right, &mut current_sequence);

    for node in &current_sequence {
      processed_nodes.insert(*node);
    }

    if current_sequence.len() > 1 {
      sequences.push(current_sequence);
    }
  }

  sequences
}

fn merge_nodes_sequence(nodes: Vec<u64>, performance_map: Option<&PerformanceMap>, graph: &mut DefaultGraph) {
  let nodes_ids = NeededNodesIds::new(graph, &nodes);

  let added_node_id = create_merged_node(&nodes, graph);

  put_activities_times(&added_node_id, &nodes, graph);
  put_trace_data(&added_node_id, &nodes, graph);
  put_software_data(&added_node_id, &nodes, graph);

  connect_added_merged_node_to_graph(&nodes_ids, &added_node_id, graph);

  put_performance_additional_infos(&nodes_ids, &added_node_id, performance_map, graph);

  disconnect_start_end_nodes(&nodes_ids, graph);
  disconnect_and_delete_nodes(&nodes, graph);
}

struct NeededNodesIds {
  pub first_node: u64,
  pub last_node: u64,
  pub start_node: u64,
  pub end_node: u64,
}

impl NeededNodesIds {
  pub fn new(graph: &DefaultGraph, sequence: &Vec<u64>) -> Self {
    let first_node = *sequence.first().unwrap();
    let last_node = *sequence.last().unwrap();

    let start_node = **graph.incoming_edges(&first_node).first().unwrap();
    let end_node = **graph.outgoing_nodes(&last_node).first().unwrap();

    Self {
      first_node,
      last_node,
      start_node,
      end_node,
    }
  }
}

fn disconnect_start_end_nodes(nodes_ids: &NeededNodesIds, graph: &mut DefaultGraph) {
  graph.disconnect_nodes(&nodes_ids.start_node, &nodes_ids.first_node);
  graph.disconnect_nodes(&nodes_ids.last_node, &nodes_ids.end_node);
}

fn connect_added_merged_node_to_graph(nodes_ids: &NeededNodesIds, added_node: &u64, graph: &mut DefaultGraph) {
  let start_node_edge_weight = *graph.edge(&nodes_ids.start_node, &nodes_ids.first_node).unwrap().weight();
  let end_node_edge_weight = *graph.edge(&nodes_ids.last_node, &nodes_ids.end_node).unwrap().weight();

  graph.connect_nodes(&nodes_ids.start_node, &added_node, NodesConnectionData::new(None, start_node_edge_weight, None));
  graph.connect_nodes(&added_node, &nodes_ids.end_node, NodesConnectionData::new(None, end_node_edge_weight, None));
}

fn create_merged_node(nodes: &Vec<u64>, graph: &mut DefaultGraph) -> u64 {
  let label = nodes.iter().map(|id| id.to_string()).collect::<Vec<String>>().join("\n");
  let node_id = graph.add_node(Some(HeapedOrOwned::Owned(label)));

  let mut inner_graph = DefaultGraph::empty();
  let mut prev_added_node_id = None;

  for node in nodes {
    let added_node_id = inner_graph.add_node_from_another_node(graph.node(node).unwrap());

    if let Some((prev_added_node_id, prev_node_id)) = prev_added_node_id {
      let edge = graph.edge(&prev_node_id, node).unwrap();
      let connection_data = NodesConnectionData::new(edge.data().as_ref().cloned(), *edge.weight(), Some(edge.user_data().clone()));
      inner_graph.connect_nodes(&prev_added_node_id, &added_node_id, connection_data);
    }

    prev_added_node_id = Some((added_node_id, *node));
  }

  graph.node_mut(&node_id).unwrap().user_data_mut().put_concrete(NODE_INNER_GRAPH_KEY.key(), inner_graph);

  node_id
}

fn put_activities_times(added_node_id: &u64, nodes: &Vec<u64>, graph: &mut DefaultGraph) {
  let activities_times = collect_start_end_time_activities_data(nodes, graph);
  graph.node_mut(&added_node_id).unwrap().user_data_mut().put_concrete(NODE_START_END_ACTIVITIES_TIMES_KEY.key(), activities_times);
}

fn put_trace_data(added_node_id: &u64, nodes: &Vec<u64>, graph: &mut DefaultGraph) {
  let trace_data = extract_user_data_from(nodes, &graph, &NODE_CORRESPONDING_TRACE_DATA_KEY);
  graph.node_mut(&added_node_id).unwrap().user_data_mut().put_concrete(NODE_CORRESPONDING_TRACE_DATA_KEY.key(), trace_data);
}

fn put_software_data(added_node_id: &u64, nodes: &Vec<u64>, graph: &mut DefaultGraph) {
  let software_data = extract_user_data_from(nodes, &graph, &NODE_SOFTWARE_DATA_KEY);
  graph.node_mut(&added_node_id).unwrap().user_data_mut().put_concrete(NODE_SOFTWARE_DATA_KEY.key(), software_data);
}

fn disconnect_and_delete_nodes(nodes: &Vec<u64>, graph: &mut DefaultGraph) {
  for i in 0..nodes.len() - 1 {
    graph.disconnect_nodes(&nodes[i], &nodes[i + 1]);
    graph.delete_node(&nodes[i]);
    graph.delete_node(&nodes[i + 1]);
  }
}

fn put_performance_additional_infos(nodes_ids: &NeededNodesIds, added_node_id: &u64, performance_map: Option<&PerformanceMap>, graph: &mut DefaultGraph) {
  if let Some(performance_map) = performance_map.as_ref() {
    put_performance_annotation_info(&nodes_ids.start_node, &nodes_ids.first_node, (&nodes_ids.start_node, &added_node_id), performance_map, graph);
    put_performance_annotation_info(&nodes_ids.last_node, &nodes_ids.end_node, (&added_node_id, &nodes_ids.end_node), performance_map, graph);
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

fn collect_start_end_time_activities_data(nodes: &Vec<u64>, graph: &DefaultGraph) -> Vec<NodeAdditionalDataContainer<ActivityStartEndTimeData>> {
  let mut times = vec![];
  for node in nodes {
    if let Some(data) = graph.node(node).unwrap().user_data().concrete(NODE_START_END_ACTIVITY_TIME_KEY.key()) {
      times.push(data.clone());
    }
  }

  times
}

pub fn adjust_connections<T: PartialEq + Clone + Debug>(
  context: &DiscoveryContext<T>,
  log: &Vec<Vec<EventWithUniqueId<T>>>,
  graph: &mut DefaultGraph,
) {
  let name_extractor = context.name_extractor();
  let mut df_relations = HashMap::new();

  for trace in log {
    for i in 0..trace.len() - 1 {
      let first_name = name_extractor(&trace[i].event());
      let second_name = name_extractor(&trace[i + 1].event());

      *df_relations.entry((Some(first_name), Some(second_name))).or_insert(0) += 1usize;
    }
  }

  let mut nodes_to_disconnect = vec![];
  for edge in graph.all_edges() {
    let from_name = graph.node(edge.from_node()).unwrap().data().cloned();
    let to_name = graph.node(edge.to_node()).unwrap().data().cloned();

    let edge_key = (*edge.from_node(), *edge.to_node());
    if df_relations.get(&(from_name, to_name)).is_none() {
      nodes_to_disconnect.push(edge_key)
    }
  }

  for (from_node, to_node) in &nodes_to_disconnect {
    graph.disconnect_nodes(from_node, to_node);
  }
}

pub fn adjust_weights<T: PartialEq + Clone + Debug>(
  log: &Vec<Vec<EventWithUniqueId<T>>>,
  graph: &mut DefaultGraph,
  start_node_id: u64,
) -> Result<(), DiscoverRootSequenceGraphError> {
  let mut edges_weights = HashMap::new();
  for trace in log {
    let replay_history = replay_sequence_with_history(graph, start_node_id, &trace[1..])?;
    for i in 0..replay_history.len() - 1 {
      let from_node = replay_history[i];
      let to_node = replay_history[i + 1];

      *edges_weights.entry((from_node, to_node)).or_insert(0.) += 1.;
    }
  }

  for ((from_node, to_node), weight) in edges_weights {
    graph.edge_mut(&from_node, &to_node).unwrap().weight = weight;
  }

  Ok(())
}

pub fn find_next_node(graph: &DefaultGraph, current_node: u64, next_event_id: u64) -> Result<u64, DiscoverRootSequenceGraphError> {
  let next_nodes = graph.outgoing_nodes(&current_node)
    .into_iter()
    .filter_map(|n| match graph.node(n).unwrap().user_data().get(EVENT_UNIQUE_ID_KEY.key()).unwrap_or(&vec![]).contains(&next_event_id) {
      true => Some(*n),
      false => None
    })
    .collect::<Vec<u64>>();

  if next_nodes.len() != 1 {
    Err(DiscoverRootSequenceGraphError::NotSingleCandidateForNextNode)
  } else {
    Ok(*next_nodes.first().unwrap())
  }
}

pub fn adjust_edges_data<T: PartialEq + Clone + Debug>(
  context: &DiscoveryContext<T>,
  log: &Vec<Vec<EventWithUniqueId<T>>>,
  graph: &mut DefaultGraph,
  start_node_id: u64
) -> Result<(), DiscoverRootSequenceGraphError> {
  for trace in log {
    let replay_history = replay_sequence_with_history(graph, start_node_id, &trace[1..])?;

    for i in 0..replay_history.len() - 1 {
      let edge = graph.edge_mut(&replay_history[i], &replay_history[i + 1]).unwrap();
      context.event_to_edge_data_transfer()(trace[i].event(), edge.user_data_mut())
    }
  }

  Ok(())
}