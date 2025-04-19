use crate::features::discovery::petri_net::annotations::PerformanceMap;
use crate::features::discovery::root_sequence::adjustments::{adjust_connections, adjust_edges_data, adjust_weights, find_next_node, merge_sequences_of_nodes};
use crate::features::discovery::root_sequence::context::DiscoveryContext;
use crate::features::discovery::root_sequence::models::{DiscoverRootSequenceGraphError, EventWithUniqueId};
use crate::features::discovery::root_sequence::root_sequence::discover_root_sequence;
use crate::pipelines::keys::context_key::DefaultContextKey;
use crate::utils::graph::graph::{DefaultGraph, NodesConnectionData};
use crate::utils::graph::graph_node::GraphNode;
use crate::utils::lcs::find_longest_common_subsequence;
use crate::utils::references::HeapedOrOwned;
use crate::utils::user_data::user_data::UserData;
use lazy_static::lazy_static;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;

lazy_static!(
   pub(super) static ref EVENT_UNIQUE_ID_KEY: DefaultContextKey<Vec<u64>> = DefaultContextKey::new("EVENT_UNIQUE_ID");
);

#[derive(Debug)]
pub struct RootSequenceGraphDiscoveryResult {
  graph: DefaultGraph,
  start_node_id: Option<u64>,
  end_node_id: Option<u64>,
}

impl RootSequenceGraphDiscoveryResult {
  pub fn new(graph: DefaultGraph, start_node_id: Option<u64>, end_node_id: Option<u64>) -> Self {
    Self {
      graph,
      start_node_id,
      end_node_id,
    }
  }

  pub fn graph(&self) -> &DefaultGraph {
    &self.graph
  }

  pub fn graph_move(self) -> DefaultGraph {
    self.graph
  }

  pub fn graph_mut(&mut self) -> &mut DefaultGraph {
    &mut self.graph
  }

  pub fn start_node_id(&self) -> Option<u64> {
    self.start_node_id.clone()
  }

  pub fn end_node_id(&self) -> Option<u64> {
    self.end_node_id.clone()
  }
}

pub fn discover_root_sequence_graph<T: PartialEq + Clone + Debug>(
  log: &Vec<Vec<EventWithUniqueId<T>>>,
  context: &DiscoveryContext<T>,
  merge_sequences_of_events: bool,
  performance_map: Option<PerformanceMap>,
) -> Result<RootSequenceGraphDiscoveryResult, DiscoverRootSequenceGraphError> {
  let mut result = discover_root_sequence_graph_internal(log, context, true)?;

  adjust_connections(context, log, &mut result.graph);

  if let Some(start_node_id) = result.start_node_id {
    adjust_weights(log, &mut result.graph, start_node_id)?;
    adjust_edges_data(context, log, &mut result.graph, start_node_id)?;
  }

  if merge_sequences_of_events {
    merge_sequences_of_nodes(&mut result.graph, performance_map);
  }

  Ok(result)
}

fn discover_root_sequence_graph_internal<T: PartialEq + Clone + Debug>(
  log: &Vec<Vec<EventWithUniqueId<T>>>,
  context: &DiscoveryContext<T>,
  first_iteration: bool,
) -> Result<RootSequenceGraphDiscoveryResult, DiscoverRootSequenceGraphError> {
  let root_sequence = discover_root_sequence(log, context.root_sequence_kind());

  if root_sequence.len() == 2 {
    return Ok(handle_recursion_exit_case(log, &root_sequence, context));
  }

  let mut graph = DefaultGraph::empty();
  let root_sequence_nodes_ids = initialize_lcs_graph_with_root_sequence(log, &root_sequence, &mut graph, &context, first_iteration);

  adjust_lcs_graph_with_traces(log, &root_sequence, &root_sequence_nodes_ids, &mut graph, context)?;

  Ok(RootSequenceGraphDiscoveryResult::new(graph, root_sequence_nodes_ids.first().cloned(), root_sequence_nodes_ids.last().cloned()))
}

fn handle_recursion_exit_case<T: PartialEq + Clone + Debug>(
  log: &Vec<Vec<EventWithUniqueId<T>>>,
  root_sequence: &Vec<EventWithUniqueId<T>>,
  context: &DiscoveryContext<T>,
) -> RootSequenceGraphDiscoveryResult {
  let mut graph = DefaultGraph::empty();

  let start_node = create_new_graph_node(&mut graph, root_sequence.first().unwrap(), false, context, false);
  let end_node = create_new_graph_node(&mut graph, root_sequence.last().unwrap(), false, context, false);

  for trace in log {
    transfer_unique_event_id(graph.node_mut(&start_node).unwrap(), trace.first().unwrap());
    transfer_unique_event_id(graph.node_mut(&end_node).unwrap(), trace.last().unwrap());
  }

  for trace in log {
    let mut prev_node_id = start_node;
    for event in trace.iter().skip(1).take(trace.len() - 2) {
      let node_id = create_new_graph_node(&mut graph, event, false, context, true);
      graph.connect_nodes(&prev_node_id, &node_id, NodesConnectionData::empty());
      prev_node_id = node_id;
    }

    graph.connect_nodes(&prev_node_id, &end_node, NodesConnectionData::empty());
  }

  RootSequenceGraphDiscoveryResult::new(graph, Some(start_node), Some(end_node))
}

pub(super) fn create_new_graph_node<T: PartialEq + Clone + Debug>(
  graph: &mut DefaultGraph,
  event: &EventWithUniqueId<T>,
  is_root_sequence: bool,
  context: &DiscoveryContext<T>,
  transfer_context_values: bool,
) -> u64 {
  let name_extractor = context.name_extractor();
  let node_id = graph.add_node(Some(name_extractor(event.event())));

  if transfer_context_values {
    transfer_user_data(graph, event, node_id, is_root_sequence, context);
  }

  node_id
}

fn transfer_user_data<T: PartialEq + Clone + Debug>(
  graph: &mut DefaultGraph,
  event: &EventWithUniqueId<T>,
  node_id: u64,
  is_root_sequence: bool,
  context: &DiscoveryContext<T>,
) {
  let mut node = graph.node_mut(&node_id).unwrap();
  let transfer = context.event_to_graph_node_info_transfer();
  transfer(event.event(), node.user_data_mut(), is_root_sequence);

  transfer_unique_event_id(&mut node, event);
}

fn transfer_unique_event_id<T: PartialEq + Clone + Debug>(node: &mut GraphNode<HeapedOrOwned<String>>, event: &EventWithUniqueId<T>) {
  if let Some(node_ids) = node.user_data_mut().concrete_mut(EVENT_UNIQUE_ID_KEY.key()) {
    node_ids.push(*event.id());
  } else {
    node.user_data_mut().put_concrete(EVENT_UNIQUE_ID_KEY.key(), vec![*event.id()]);
  }
}

fn initialize_lcs_graph_with_root_sequence<T: PartialEq + Clone + Debug>(
  log: &Vec<Vec<EventWithUniqueId<T>>>,
  root_sequence: &Vec<EventWithUniqueId<T>>,
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
    for (trace_index, root_sequence_index) in lcs.first_indices().iter().zip(lcs.second_indices().iter()) {
      let event = trace.get(*trace_index).unwrap();
      transfer_user_data(graph, event, root_sequence_node_ids[*root_sequence_index], is_first_iteration_root_sequence, context);
    }
  }

  root_sequence_node_ids
}

fn adjust_lcs_graph_with_traces<T: PartialEq + Clone + Debug>(
  traces: &Vec<Vec<EventWithUniqueId<T>>>,
  lcs: &Vec<EventWithUniqueId<T>>,
  root_sequence_node_ids: &Vec<u64>,
  graph: &mut DefaultGraph,
  context: &DiscoveryContext<T>,
) -> Result<(), DiscoverRootSequenceGraphError> {
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

  let mut adjustments: Vec<(u64, Vec<(u64, Vec<Vec<EventWithUniqueId<T>>>)>)> = adjustments
    .into_iter()
    .map(|(k, v)| {
      let mut values: Vec<(u64, Vec<Vec<EventWithUniqueId<T>>>)> = v.into_iter().collect();
      values.sort_by(|f, s| f.0.cmp(&s.0));
      (k, values)
    }).collect();

  adjustments.sort_by(|f, s| f.0.cmp(&s.0));

  add_adjustments_to_graph(&adjustments, graph, context)
}

fn add_adjustments_to_graph<T: PartialEq + Clone + Debug>(
  adjustments: &Vec<(u64, Vec<(u64, Vec<Vec<EventWithUniqueId<T>>>)>)>,
  graph: &mut DefaultGraph,
  context: &DiscoveryContext<T>,
) -> Result<(), DiscoverRootSequenceGraphError> {
  for (start_root_node_id, adjustments) in adjustments {
    let adjustment_log = create_log_from_adjustments(adjustments, context.artificial_start_end_events_factory());
    let result = discover_root_sequence_graph_internal(&adjustment_log, context, false)?;

    merge_subgraph_into_model(adjustments, graph, result.graph_move(), *start_root_node_id, context)?;
  }

  Ok(())
}

fn create_log_from_adjustments<T: PartialEq + Clone + Debug>(
  end_root_sequence_nodes_to_adjustments: &Vec<(u64, Vec<Vec<EventWithUniqueId<T>>>)>,
  artificial_start_end_events_factory: impl Fn() -> (T, T),
) -> Vec<Vec<EventWithUniqueId<T>>> {
  let mut adjustment_log = vec![];

  for (_, adjustments) in end_root_sequence_nodes_to_adjustments {
    for adjustment in adjustments {
      if adjustment.is_empty() {
        continue;
      }

      let (art_start, art_end) = artificial_start_end_events_factory();
      let mut adjustment_trace = vec![EventWithUniqueId::new(art_start)];
      for event in adjustment {
        adjustment_trace.push(event.clone());
      }

      adjustment_trace.push(EventWithUniqueId::new(art_end));
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
  adjustments: &Vec<(u64, Vec<Vec<EventWithUniqueId<T>>>)>,
  graph: &mut DefaultGraph,
  sub_graph: DefaultGraph,
  start_graph_node_id: u64,
  context: &DiscoveryContext<T>,
) -> Result<(), DiscoverRootSequenceGraphError> {
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
      let final_node = replay_sequence(graph, start_graph_node_id, trace.as_slice())?;
      graph.connect_nodes(&final_node, end_node_id, NodesConnectionData::empty());
    }
  }

  Ok(())
}

fn replay_sequence<T: PartialEq + Clone + Debug>(
  graph: &DefaultGraph,
  start_node_id: u64,
  sequence: &[EventWithUniqueId<T>],
) -> Result<u64, DiscoverRootSequenceGraphError> {
  let mut replay_states = VecDeque::from_iter([(start_node_id, 0usize)]);

  loop {
    if replay_states.is_empty() {
      return Err(DiscoverRootSequenceGraphError::FailedToReplaySequence);
    }

    let (current_node_id, event_index) = replay_states.pop_back().unwrap();
    if event_index == sequence.len() {
      return Ok(current_node_id);
    }

    let next_node = find_next_node(graph, current_node_id, *sequence[event_index].id())?;
    replay_states.push_back((next_node, event_index + 1));
  }
}

struct ReplayHistoryEntry {
  pub node_id: u64,
  pub parent: Option<usize>,
}

impl ReplayHistoryEntry {
  pub fn new(node_id: u64, parent: Option<usize>) -> Self {
    Self {
      node_id,
      parent,
    }
  }
}

pub(super) fn replay_sequence_with_history<T: PartialEq + Clone + Debug>(
  graph: &DefaultGraph,
  start_node_id: u64,
  sequence: &[EventWithUniqueId<T>],
) -> Result<Vec<u64>, DiscoverRootSequenceGraphError> {
  let mut replay_states = VecDeque::from_iter([(start_node_id, 0usize, 0usize)]);
  let mut replay_history = vec![ReplayHistoryEntry::new(start_node_id, None)];

  loop {
    if replay_states.is_empty() {
      return Err(DiscoverRootSequenceGraphError::FailedToReplaySequence);
    }

    let (current_node_id, event_index, history_end_index) = replay_states.pop_back().unwrap();
    if event_index == sequence.len() {
      let mut history = vec![];
      let mut current_history_index = Some(history_end_index);
      loop {
        if current_history_index.is_none() {
          break;
        }

        history.push(replay_history[current_history_index.unwrap()].node_id);
        current_history_index = replay_history[current_history_index.unwrap()].parent;
      }

      history.reverse();

      return Ok(history);
    }

    let next_node = find_next_node(graph, current_node_id, *sequence[event_index].id())?;

    replay_history.push(ReplayHistoryEntry::new(next_node, Some(history_end_index)));
    replay_states.push_back((next_node, event_index + 1, replay_history.len() - 1));
  }
}