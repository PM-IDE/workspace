use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::event_log::xes::xes_trace::XesTraceImpl;
use crate::features::analysis::patterns::activity_instances::create_vector_of_underlying_events;
use crate::features::mutations::mutations::{ARTIFICIAL_END_EVENT_NAME, ARTIFICIAL_START_EVENT_NAME};
use crate::pipelines::keys::context_keys::{CORRESPONDING_TRACE_DATA_KEY, SOFTWARE_DATA_KEY};
use crate::utils::distance::distance::calculate_lcs_distance;
use crate::utils::graph::graph::{DefaultGraph, NodesConnectionData};
use crate::utils::lcs::{find_longest_common_subsequence, find_longest_common_subsequence_length};
use crate::utils::references::HeapedOrOwned;
use crate::utils::user_data::user_data::{UserData, UserDataImpl};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::rc::Rc;
use std::str::FromStr;

pub enum DiscoverLCSGraphError {
  NoArtificialStartEndEvents
}

#[derive(Clone, Copy)]
pub enum RootSequenceKind {
  FindBest,
  LCS,
  PairwiseLCS,
  Trace,
}

impl FromStr for RootSequenceKind {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "FindBest" => Ok(Self::FindBest),
      "LCS" => Ok(Self::LCS),
      "PairwiseLCS" => Ok(Self::PairwiseLCS),
      "Trace" => Ok(Self::Trace),
      _ => Err(())
    }
  }
}

impl Display for DiscoverLCSGraphError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      DiscoverLCSGraphError::NoArtificialStartEndEvents => f.write_str("All traces in event log must have artificial start-end events")
    }
  }
}

pub struct DiscoveryContext<'a, T> {
  name_extractor: &'a dyn Fn(&T) -> HeapedOrOwned<String>,
  artificial_start_end_events_factory: &'a dyn Fn() -> (T, T),
  root_sequence_kind: RootSequenceKind,
  event_to_graph_node_info_transfer: &'a dyn Fn(&T, &mut UserDataImpl, bool) -> (),
  underlying_events_extractor: &'a dyn Fn(&T) -> Option<Vec<T>>,
}

impl<'a, T> DiscoveryContext<'a, T> {
  pub fn new(
    name_extractor: &'a dyn Fn(&T) -> HeapedOrOwned<String>,
    artificial_start_end_events_factory: &'a dyn Fn() -> (T, T),
    root_sequence_kind: RootSequenceKind,
    event_to_graph_node_info_transfer: &'a dyn Fn(&T, &mut UserDataImpl, bool) -> (),
    underlying_events_extractor: &'a dyn Fn(&T) -> Option<Vec<T>>,
  ) -> Self {
    Self {
      name_extractor,
      artificial_start_end_events_factory,
      root_sequence_kind,
      event_to_graph_node_info_transfer,
      underlying_events_extractor,
    }
  }
}

pub fn discover_root_sequence_graph_from_event_log(log: &XesEventLogImpl, root_sequence_kind: RootSequenceKind) -> Result<DefaultGraph, DiscoverLCSGraphError> {
  assert_all_traces_have_artificial_start_end_events(log)?;
  set_corresponding_trace_data(log);

  let name_extractor = |e: &Rc<RefCell<XesEventImpl>>| HeapedOrOwned::Heaped(e.borrow().name_pointer().clone());

  let artificial_start_end_events_factory = || (
    Rc::new(RefCell::new(XesEventImpl::new_with_min_date(ARTIFICIAL_START_EVENT_NAME.to_string()))),
    Rc::new(RefCell::new(XesEventImpl::new_with_min_date(ARTIFICIAL_END_EVENT_NAME.to_string()))),
  );

  let event_to_graph_node_info_transfer = |event: &Rc<RefCell<XesEventImpl>>, user_data_impl: &mut UserDataImpl, belongs_to_root_sequence: bool| {
    if let Some(software_data) = event.borrow().user_data().concrete(SOFTWARE_DATA_KEY.key()) {
      user_data_impl.put_concrete(SOFTWARE_DATA_KEY.key(), software_data.clone());
    }

    if let Some(corresponding_trace_data) = event.borrow().user_data().concrete(CORRESPONDING_TRACE_DATA_KEY.key()) {
      let mut corresponding_trace_data = corresponding_trace_data.clone();
      corresponding_trace_data.set_belongs_to_root_sequence(belongs_to_root_sequence);

      user_data_impl.put_concrete(CORRESPONDING_TRACE_DATA_KEY.key(), corresponding_trace_data);
    }
  };

  let underlying_events_extractor = |event: &Rc<RefCell<XesEventImpl>>| {
    let underlying_events = create_vector_of_underlying_events::<XesEventLogImpl>(event);
    match underlying_events.is_empty() {
      true => None,
      false => Some(underlying_events)
    }
  };

  let context = DiscoveryContext {
    name_extractor: &name_extractor,
    artificial_start_end_events_factory: &artificial_start_end_events_factory,
    root_sequence_kind,
    event_to_graph_node_info_transfer: &event_to_graph_node_info_transfer,
    underlying_events_extractor: &underlying_events_extractor,
  };

  let log = log.traces().iter().map(|t| t.borrow().events().clone()).collect();
  Ok(discover_root_sequence_graph(&log, &context))
}

#[derive(Clone, Debug)]
pub struct CorrespondingTraceData {
  trace_id: u64,
  event_index: u64,
  belongs_to_root_sequence: bool,
}

impl CorrespondingTraceData {
  pub fn belongs_to_root_sequence(&self) -> bool {
    self.belongs_to_root_sequence
  }

  pub fn set_belongs_to_root_sequence(&mut self, value: bool) { self.belongs_to_root_sequence = value }

  pub fn trace_id(&self) -> u64 {
    self.trace_id
  }

  pub fn event_index(&self) -> u64 {
    self.event_index
  }
}

fn set_corresponding_trace_data(log: &XesEventLogImpl) {
  for (trace_index, trace) in log.traces().iter().enumerate() {
    for (event_index, event) in trace.borrow().events().iter().enumerate() {
      event.borrow_mut().user_data_mut().put_concrete(CORRESPONDING_TRACE_DATA_KEY.key(), CorrespondingTraceData {
        belongs_to_root_sequence: false,
        trace_id: trace_index as u64,
        event_index: event_index as u64,
      })
    }
  }
}

pub fn discover_root_sequence_graph<T: PartialEq + Clone + Debug>(
  log: &Vec<Vec<T>>,
  context: &DiscoveryContext<T>,
) -> DefaultGraph {
  discover_root_sequence_graph_internal(log, context, true)
}

fn discover_root_sequence_graph_internal<T: PartialEq + Clone + Debug>(
  log: &Vec<Vec<T>>,
  context: &DiscoveryContext<T>,
  first_iteration: bool,
) -> DefaultGraph {
  let root_sequence = discover_root_sequence(log, context.root_sequence_kind);

  if root_sequence.len() == 2 {
    return handle_recursion_exit_case(log, &root_sequence, context);
  }

  let mut graph = DefaultGraph::empty();
  let root_sequence_nodes_ids = initialize_lcs_graph_with_root_sequence(&root_sequence, &mut graph, &context, first_iteration);

  adjust_lcs_graph_with_traces(log, &root_sequence, &root_sequence_nodes_ids, &mut graph, context);

  if first_iteration {
    adjust_weights_and_connections(context, log, &mut graph);
    merge_sequences_of_nodes(&mut graph);
  }

  graph
}

fn merge_sequences_of_nodes(graph: &mut DefaultGraph) {
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
    let start_node = **graph.incoming_edges(current_sequence.first().unwrap()).first().unwrap();
    let end_node = **graph.outgoing_nodes(current_sequence.last().unwrap()).first().unwrap();

    graph.disconnect_nodes(&start_node, current_sequence.first().unwrap());
    graph.disconnect_nodes(current_sequence.last().unwrap(), &end_node);

    for i in 0..current_sequence.len() - 1 {
      graph.disconnect_nodes(&current_sequence[i], &current_sequence[i + 1]);
      graph.delete_node(&current_sequence[i]);
      graph.delete_node(&current_sequence[i + 1]);
    }

    let label = current_sequence.iter().map(|id| id.to_string()).collect::<Vec<String>>().join("\n");
    let added_node_id = graph.add_node(Some(HeapedOrOwned::Owned(label)));

    graph.connect_nodes(&start_node, &added_node_id, NodesConnectionData::empty());
    graph.connect_nodes(&added_node_id, &end_node, NodesConnectionData::empty());
  }
}

fn adjust_weights_and_connections<T: PartialEq + Clone + Debug>(context: &DiscoveryContext<T>, log: &Vec<Vec<T>>, graph: &mut DefaultGraph) {
  let name_extractor = context.name_extractor;
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

fn handle_recursion_exit_case<T: PartialEq + Clone + Debug>(
  log: &Vec<Vec<T>>,
  root_sequence: &Vec<T>,
  context: &DiscoveryContext<T>,
) -> DefaultGraph {
  let mut graph = DefaultGraph::empty();
  let name_extractor = context.name_extractor;
  let start_node = graph.add_node(Some(name_extractor(root_sequence.first().unwrap())));
  let end_node = graph.add_node(Some(name_extractor(root_sequence.last().unwrap())));

  for trace in log {
    let mut prev_node_id = start_node;
    for event in trace.iter().skip(1).take(trace.len() - 2) {
      let node_id = create_new_graph_node(&mut graph, event, false, context);
      graph.connect_nodes(&prev_node_id, &node_id, NodesConnectionData::empty());
      prev_node_id = node_id;
    }

    graph.connect_nodes(&prev_node_id, &end_node, NodesConnectionData::empty());
  }

  graph
}

fn create_new_graph_node<T>(graph: &mut DefaultGraph, event: &T, is_root_sequence: bool, context: &DiscoveryContext<T>) -> u64 {
  let name_extractor = context.name_extractor;
  let node_id = graph.add_node(Some(name_extractor(event)));

  let node = graph.node_mut(&node_id).unwrap();
  let transfer = context.event_to_graph_node_info_transfer;
  transfer(event, node.user_data_mut(), is_root_sequence);

  node_id
}

fn initialize_lcs_graph_with_root_sequence<T: PartialEq + Clone>(
  root_sequence: &Vec<T>,
  graph: &mut DefaultGraph,
  context: &DiscoveryContext<T>,
  is_first_iteration_root_sequence: bool,
) -> Vec<u64> {
  let mut prev_node_id = None;
  let mut root_sequence_node_ids = vec![];

  for event in root_sequence {
    let node_id = create_new_graph_node(graph, event, is_first_iteration_root_sequence, context);
    root_sequence_node_ids.push(node_id);

    if let Some(prev_node_id) = prev_node_id.as_ref() {
      graph.connect_nodes(prev_node_id, &node_id, NodesConnectionData::empty());
    }

    prev_node_id = Some(node_id);
  }

  root_sequence_node_ids
}

fn adjust_lcs_graph_with_traces<T: PartialEq + Clone + Debug>(
  traces: &Vec<Vec<T>>,
  lcs: &Vec<T>,
  root_sequence_node_ids: &Vec<u64>,
  graph: &mut DefaultGraph,
  context: &DiscoveryContext<T>,
) {
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

      let key = (root_sequence_node_ids[second_indices[lcs_index - 1]], root_sequence_node_ids[second_indices[lcs_index]]);

      adjustments.entry(key).or_insert(vec![]).push(adjustment_events);

      index += 1;
      lcs_index += 1;
    }
  }

  add_adjustments_to_graph(&adjustments, graph, context);
}

fn add_adjustments_to_graph<T: PartialEq + Clone + Debug>(
  adjustments: &HashMap<(u64, u64), Vec<Vec<T>>>,
  graph: &mut DefaultGraph,
  context: &DiscoveryContext<T>,
) {
  for ((start_root_node_id, end_root_node_id), adjustments) in adjustments {
    let adjustment_log = create_log_from_adjustments(&adjustments, context.artificial_start_end_events_factory);
    let sub_graph = discover_root_sequence_graph_internal(&adjustment_log, context, false);

    merge_subgraph_into_model(graph, sub_graph, *start_root_node_id, *end_root_node_id, context);
  }
}

fn create_log_from_adjustments<T: PartialEq + Clone + Debug>(
  adjustments: &Vec<Vec<T>>,
  artificial_start_end_events_factory: impl Fn() -> (T, T),
) -> Vec<Vec<T>> {
  let mut adjustment_log = vec![];
  for adjustment in adjustments {
    let (art_start, art_end) = artificial_start_end_events_factory();
    let mut adjustment_trace = vec![art_start];
    for event in adjustment {
      adjustment_trace.push(event.clone());
    }

    adjustment_trace.push(art_end);
    adjustment_log.push(adjustment_trace);
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
  graph: &mut DefaultGraph,
  sub_graph: DefaultGraph,
  start_graph_node_id: u64,
  end_graph_node_id: u64,
  context: &DiscoveryContext<T>,
) {
  let (start_node_id, end_node_id) = find_start_end_node_ids(&sub_graph, context.name_extractor, context.artificial_start_end_events_factory);
  let mut sub_graph_nodes_to_nodes = HashMap::new();

  for node in sub_graph.all_nodes() {
    if *node.id() != start_node_id && *node.id() != end_node_id {
      sub_graph_nodes_to_nodes.insert(node.id(), graph.add_node_with_user_data(node.data.clone(), node.user_data().clone()));
    }
  }

  for edge in sub_graph.all_edges() {
    let from_node = if *edge.from_node() == start_node_id {
      start_graph_node_id
    } else {
      sub_graph_nodes_to_nodes[edge.from_node()]
    };

    let to_node = if *edge.to_node() == end_node_id {
      end_graph_node_id
    } else {
      sub_graph_nodes_to_nodes[edge.to_node()]
    };

    graph.connect_nodes(&from_node, &to_node, NodesConnectionData::empty());
  }
}

fn assert_all_traces_have_artificial_start_end_events(log: &XesEventLogImpl) -> Result<(), DiscoverLCSGraphError> {
  for trace in log.traces().iter().map(|t| t.borrow()) {
    if !check_trace_have_artificial_start_end_events(trace.deref()) {
      return Err(DiscoverLCSGraphError::NoArtificialStartEndEvents);
    }
  }

  Ok(())
}

fn check_trace_have_artificial_start_end_events(trace: &XesTraceImpl) -> bool {
  trace.events().len() >= 2 &&
    trace.events().first().unwrap().borrow().name().as_str() == ARTIFICIAL_START_EVENT_NAME &&
    trace.events().last().unwrap().borrow().name().as_str() == ARTIFICIAL_END_EVENT_NAME
}

pub fn discover_root_sequence<T: PartialEq + Clone + Debug>(log: &Vec<Vec<T>>, root_sequence_kind: RootSequenceKind) -> Vec<T> {
  if log.is_empty() {
    return vec![];
  }

  match root_sequence_kind {
    RootSequenceKind::FindBest => {
      let (root_trace_index, root_distance) = find_trace_candidate_for_root_sequence(log);
      let (indices, root_pair_wise_lcs_distance) = find_traces_pairwise_lcs_candidate_for_root_sequence(log);
      let (lcs, root_lcs_distance) = find_lcs_candidate_for_root_sequence(log);

      let min_distance = root_distance.min(root_pair_wise_lcs_distance).min(root_lcs_distance);
      if root_distance == min_distance {
        log.get(root_trace_index).unwrap().iter().map(|c| c.clone()).collect()
      } else if root_pair_wise_lcs_distance == min_distance {
        create_root_sequence_from_lcs(log, indices)
      } else {
        lcs
      }
    }
    RootSequenceKind::LCS => find_lcs_candidate_for_root_sequence(log).0,
    RootSequenceKind::PairwiseLCS => create_root_sequence_from_lcs(log, find_traces_pairwise_lcs_candidate_for_root_sequence(log).0),
    RootSequenceKind::Trace => log.get(find_trace_candidate_for_root_sequence(log).0).unwrap().iter().map(|c| c.clone()).collect()
  }
}

fn find_trace_candidate_for_root_sequence<T: PartialEq + Clone + Debug>(log: &Vec<Vec<T>>) -> (usize, f64) {
  let mut root_trace_index = 0;
  let mut root_distance = f64::MAX;
  for (index, trace_events) in log.iter().enumerate() {
    let mut summed_distance = 0.;
    for other_trace_events in log.iter() {
      let lcs = find_longest_common_subsequence_length(trace_events, other_trace_events, trace_events.len(), other_trace_events.len());
      let distance = calculate_lcs_distance(lcs, trace_events.len(), other_trace_events.len());

      summed_distance += distance;
    }

    if summed_distance < root_distance {
      root_distance = summed_distance;
      root_trace_index = index;
    }
  }

  (root_trace_index, root_distance)
}

fn find_traces_pairwise_lcs_candidate_for_root_sequence<T: PartialEq + Clone + Debug>(log: &Vec<Vec<T>>) -> ((usize, usize), f64) {
  let mut root_lcs_distance = f64::MAX;
  let mut indices = (0, 0);
  for (first_index, first_trace) in log.iter().enumerate() {
    for (second_index, second_trace) in log.iter().enumerate() {
      if first_index == second_index {
        continue;
      }

      let lcs = find_longest_common_subsequence(first_trace, second_trace, first_trace.len(), second_trace.len())
        .lcs().into_iter().map(|c| (*c).clone()).collect::<Vec<T>>();

      let mut distance = 0.;
      for trace in log.iter() {
        let lcs_length = find_longest_common_subsequence_length(&lcs, trace, lcs.len(), trace.len());
        distance += calculate_lcs_distance(lcs_length, lcs.len(), trace.len());
      }

      if distance < root_lcs_distance {
        root_lcs_distance = distance;
        indices = (first_index, second_index);
      }
    }
  }

  (indices, root_lcs_distance)
}

fn find_lcs_candidate_for_root_sequence<T: PartialEq + Clone + Debug>(log: &Vec<Vec<T>>) -> (Vec<T>, f64) {
  let mut lcs = log.first().unwrap().into_iter().map(|e| (*e).clone()).collect();

  for trace in log.iter().skip(1) {
    lcs = find_longest_common_subsequence(&lcs, trace, lcs.len(), trace.len()).lcs().into_iter().map(|e| (*e).clone()).collect();
  }

  let mut distance = 0.;
  for trace in log {
    distance += calculate_lcs_distance(lcs.len(), lcs.len(), trace.len());
  }

  (lcs, distance)
}

fn create_root_sequence_from_lcs<T: PartialEq + Clone + Debug>(log: &Vec<Vec<T>>, indices: (usize, usize)) -> Vec<T> {
  let first_trace = log.get(indices.0).unwrap();
  let second_trace = log.get(indices.1).unwrap();

  let first_trace_len = first_trace.len();
  let second_trace_len = second_trace.len();

  find_longest_common_subsequence(first_trace, second_trace, first_trace_len, second_trace_len)
    .lcs()
    .into_iter()
    .map(|c| (*c).clone())
    .collect::<Vec<T>>()
}