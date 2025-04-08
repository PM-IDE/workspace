use crate::features::discovery::petri_net::annotations::{PerformanceMap};
use crate::features::discovery::root_sequence::adjustments::{adjust_connections, adjust_weights, find_next_nodes, merge_sequences_of_nodes};
use crate::features::discovery::root_sequence::context::DiscoveryContext;
use crate::features::discovery::root_sequence::root_sequence::discover_root_sequence;
use crate::utils::graph::graph::{DefaultGraph, NodesConnectionData};
use crate::utils::lcs::find_longest_common_subsequence;
use crate::utils::references::HeapedOrOwned;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use crate::features::discovery::root_sequence::models::DiscoverLCSGraphError;

pub fn discover_root_sequence_graph<T: PartialEq + Clone + Debug>(
  log: &Vec<Vec<T>>,
  context: &DiscoveryContext<T>,
  merge_sequences_of_events: bool,
  performance_map: Option<PerformanceMap>,
) -> Result<DefaultGraph, DiscoverLCSGraphError> {
  let (start_node_id, mut graph) = discover_root_sequence_graph_internal(log, context, true)?;

  adjust_connections(context, log, &mut graph);
  adjust_weights(context, log, &mut graph, start_node_id);

  if merge_sequences_of_events {
    merge_sequences_of_nodes(&mut graph, performance_map);
  }

  Ok(graph)
}

fn discover_root_sequence_graph_internal<T: PartialEq + Clone + Debug>(
  log: &Vec<Vec<T>>,
  context: &DiscoveryContext<T>,
  first_iteration: bool,
) -> Result<(u64, DefaultGraph), DiscoverLCSGraphError> {
  let root_sequence = discover_root_sequence(log, context.root_sequence_kind());

  if root_sequence.len() == 2 {
    return Ok(handle_recursion_exit_case(log, &root_sequence, context));
  }

  let mut graph = DefaultGraph::empty();
  let root_sequence_nodes_ids = initialize_lcs_graph_with_root_sequence(log, &root_sequence, &mut graph, &context, first_iteration);

  adjust_lcs_graph_with_traces(log, &root_sequence, &root_sequence_nodes_ids, &mut graph, context)?;

  Ok((*root_sequence_nodes_ids.first().unwrap(), graph))
}

fn handle_recursion_exit_case<T: PartialEq + Clone + Debug>(
  log: &Vec<Vec<T>>,
  root_sequence: &Vec<T>,
  context: &DiscoveryContext<T>,
) -> (u64, DefaultGraph) {
  let mut graph = DefaultGraph::empty();
  let name_extractor = context.name_extractor();
  let start_node = graph.add_node(Some(name_extractor(root_sequence.first().unwrap())));
  let end_node = graph.add_node(Some(name_extractor(root_sequence.last().unwrap())));

  for trace in log {
    let mut prev_node_id = start_node;
    for event in trace.iter().skip(1).take(trace.len() - 2) {
      let node_id = create_new_graph_node(&mut graph, event, false, context, true);
      graph.connect_nodes(&prev_node_id, &node_id, NodesConnectionData::empty());
      prev_node_id = node_id;
    }

    graph.connect_nodes(&prev_node_id, &end_node, NodesConnectionData::empty());
  }

  (start_node, graph)
}

pub(super) fn create_new_graph_node<T>(
  graph: &mut DefaultGraph,
  event: &T,
  is_root_sequence: bool,
  context: &DiscoveryContext<T>,
  transfer_context_values: bool,
) -> u64 {
  let name_extractor = context.name_extractor();
  let node_id = graph.add_node(Some(name_extractor(event)));

  if transfer_context_values {
    transfer_user_data(graph, event, node_id, is_root_sequence, context);
  }

  node_id
}

fn transfer_user_data<T>(graph: &mut DefaultGraph, event: &T, node_id: u64, is_root_sequence: bool, context: &DiscoveryContext<T>) {
  let node = graph.node_mut(&node_id).unwrap();
  let transfer = context.event_to_graph_node_info_transfer();
  transfer(event, node.user_data_mut(), is_root_sequence);
}

fn initialize_lcs_graph_with_root_sequence<T: PartialEq + Clone>(
  log: &Vec<Vec<T>>,
  root_sequence: &Vec<T>,
  graph: &mut DefaultGraph,
  context: &DiscoveryContext<T>,
  is_first_iteration_root_sequence: bool,
) -> Vec<u64> {
  let mut prev_node_id = None;
  let mut root_sequence_node_ids = vec![];

  for event in root_sequence {
    let node_id = create_new_graph_node(graph, event, is_first_iteration_root_sequence, context, false);
    root_sequence_node_ids.push(node_id);

    if let Some(prev_node_id) = prev_node_id.as_ref() {
      graph.connect_nodes(prev_node_id, &node_id, NodesConnectionData::empty());
    }

    prev_node_id = Some(node_id);
  }

  for trace in log {
    let lcs = find_longest_common_subsequence(trace, root_sequence, trace.len(), root_sequence.len());
    for (index, node_id) in lcs.first_indices().iter().zip(&root_sequence_node_ids) {
      let event = trace.get(*index).unwrap();
      transfer_user_data(graph, event, *node_id, is_first_iteration_root_sequence, context);
    }
  }

  root_sequence_node_ids
}

fn adjust_lcs_graph_with_traces<T: PartialEq + Clone + Debug>(
  traces: &Vec<Vec<T>>,
  lcs: &Vec<T>,
  root_sequence_node_ids: &Vec<u64>,
  graph: &mut DefaultGraph,
  context: &DiscoveryContext<T>,
) -> Result<(), DiscoverLCSGraphError> {
  let mut adjustments = HashMap::new();
  for trace in traces {
    let trace_lcs = find_longest_common_subsequence(trace, &lcs, trace.len(), lcs.len());
    let second_indices = trace_lcs.second_indices();

    let mut lcs_index = 0;
    let mut index = 0;

    while index < trace.len() {
      if index == trace_lcs.first_indices()[lcs_index] {
        if lcs_index >= 1 && second_indices[lcs_index - 1] + 1 != second_indices[lcs_index] {
          graph.connect_nodes(&root_sequence_node_ids[second_indices[lcs_index - 1]], &root_sequence_node_ids[second_indices[lcs_index]], NodesConnectionData::empty());
        }

        lcs_index += 1;
        index += 1;
        continue;
      }

      let mut adjustment_events = vec![];
      while index < trace.len() && index != trace_lcs.first_indices()[lcs_index] {
        adjustment_events.push(trace.get(index).unwrap().clone());
        index += 1;
      }

      let from_node_id = root_sequence_node_ids[second_indices[lcs_index - 1]];
      let to_node_id = root_sequence_node_ids[second_indices[lcs_index]];

      adjustments.entry(from_node_id).or_insert(HashMap::new()).entry(to_node_id).or_insert(vec![]).push(adjustment_events);

      index += 1;
      lcs_index += 1;
    }
  }

  let mut adjustments: Vec<(u64, Vec<(u64, Vec<Vec<T>>)>)> = adjustments
    .into_iter()
    .map(|(k, v)| {
      let mut values: Vec<(u64, Vec<Vec<T>>)> = v.into_iter().collect();
      values.sort_by(|f, s| f.0.cmp(&s.0));
      (k, values)
    }).collect();

  adjustments.sort_by(|f, s| f.0.cmp(&s.0));

  add_adjustments_to_graph(&adjustments, graph, context)
}

fn add_adjustments_to_graph<T: PartialEq + Clone + Debug>(
  adjustments: &Vec<(u64, Vec<(u64, Vec<Vec<T>>)>)>,
  graph: &mut DefaultGraph,
  context: &DiscoveryContext<T>,
) -> Result<(), DiscoverLCSGraphError> {
  for (start_root_node_id, adjustments) in adjustments {
    let adjustment_log = create_log_from_adjustments(adjustments, context.artificial_start_end_events_factory());
    let (_, sub_graph) = discover_root_sequence_graph_internal(&adjustment_log, context, false)?;

    merge_subgraph_into_model(adjustments, graph, sub_graph, *start_root_node_id, context)?;
  }

  Ok(())
}

fn create_log_from_adjustments<T: PartialEq + Clone + Debug>(
  end_root_sequence_nodes_to_adjustments: &Vec<(u64, Vec<Vec<T>>)>,
  artificial_start_end_events_factory: impl Fn() -> (T, T),
) -> Vec<Vec<T>> {
  let mut adjustment_log = vec![];

  for (_, adjustments) in end_root_sequence_nodes_to_adjustments {
    for adjustment in adjustments {
      if adjustment.is_empty() {
        continue;
      }

      let (art_start, art_end) = artificial_start_end_events_factory();
      let mut adjustment_trace = vec![art_start];
      for event in adjustment {
        adjustment_trace.push(event.clone());
      }

      adjustment_trace.push(art_end);
      adjustment_log.push(adjustment_trace);
    }
  }

  adjustment_log
}

fn find_start_end_node_ids<T: PartialEq + Clone + Debug>(
  sub_graph: &DefaultGraph,
  name_extractor: &dyn Fn(&T) -> HeapedOrOwned<String>,
  artificial_start_end_events_factory: &dyn Fn() -> (T, T),
) -> (u64, u64) {
  let (mut start_node_id, mut end_node_id) = (0, 0);
  let (art_start, art_end) = artificial_start_end_events_factory();
  let (art_start_name, art_end_name) = (name_extractor(&art_start), name_extractor(&art_end));

  for node in sub_graph.all_nodes() {
    if let Some(data) = node.data() {
      if data.as_str().eq(art_start_name.as_str()) {
        start_node_id = *node.id();
      }

      if data.as_str().eq(art_end_name.as_str()) {
        end_node_id = *node.id();
      }
    }
  }

  (start_node_id, end_node_id)
}

fn merge_subgraph_into_model<T: PartialEq + Clone + Debug>(
  adjustments: &Vec<(u64, Vec<Vec<T>>)>,
  graph: &mut DefaultGraph,
  sub_graph: DefaultGraph,
  start_graph_node_id: u64,
  context: &DiscoveryContext<T>,
) -> Result<(), DiscoverLCSGraphError> {
  let (start_node_id, end_node_id) = find_start_end_node_ids(&sub_graph, context.name_extractor(), context.artificial_start_end_events_factory());
  let mut sub_graph_nodes_to_nodes = HashMap::new();

  for node in sub_graph.all_nodes() {
    if *node.id() != start_node_id && *node.id() != end_node_id {
      sub_graph_nodes_to_nodes.insert(*node.id(), graph.add_node_with_user_data(node.data.clone(), node.user_data().clone()));
    }
  }

  for edge in sub_graph.all_edges() {
    let from_node = if *edge.from_node() == start_node_id {
      start_graph_node_id
    } else {
      sub_graph_nodes_to_nodes[edge.from_node()]
    };

    if *edge.to_node() != end_node_id {
      graph.connect_nodes(&from_node, &sub_graph_nodes_to_nodes[edge.to_node()], NodesConnectionData::empty());
    }
  }

  for (end_node_id, log) in adjustments {
    for trace in log {
      let final_node = replay_sequence(context, graph, start_graph_node_id, trace.as_slice())?;
      graph.connect_nodes(&final_node, end_node_id, NodesConnectionData::empty());
    }
  }
  
  Ok(())
}

fn replay_sequence<T>(context: &DiscoveryContext<T>, graph: &DefaultGraph, start_node_id: u64, sequence: &[T]) -> Result<u64, DiscoverLCSGraphError> {
  let mut replay_states = VecDeque::from_iter([(start_node_id, 0usize)]);

  loop {
    if replay_states.is_empty() {
      return Err(DiscoverLCSGraphError::FailedToReplaySequence);
    }

    let (current_node_id, event_index) = replay_states.pop_back().unwrap();
    if event_index == sequence.len() {
      return Ok(current_node_id);
    }

    let outgoing_nodes = find_next_nodes(graph, current_node_id, &context.name_extractor()(&sequence[event_index]));
    for next_node in outgoing_nodes {
      replay_states.push_back((next_node, event_index + 1));
    }
  }
}