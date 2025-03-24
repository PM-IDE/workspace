use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::event_log::xes::xes_trace::XesTraceImpl;
use crate::features::mutations::mutations::{ARTIFICIAL_END_EVENT_NAME, ARTIFICIAL_START_EVENT_NAME};
use crate::utils::distance::distance::calculate_lcs_distance;
use crate::utils::graph::graph::{DefaultGraph, NodesConnectionData};
use crate::utils::lcs::{find_longest_common_subsequence, find_longest_common_subsequence_length};
use crate::utils::references::HeapedOrOwned;
use std::cell::RefCell;
use std::collections::HashMap;
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

pub fn discover_lcs_graph_from_event_log(log: &XesEventLogImpl, root_sequence_kind: RootSequenceKind) -> Result<DefaultGraph, DiscoverLCSGraphError> {
  assert_all_traces_have_artificial_start_end_events(log)?;
  let name_extractor = |e: &Rc<RefCell<XesEventImpl>>| HeapedOrOwned::Heaped(e.borrow().name_pointer().clone());

  let artificial_start_end_events_factory = || (
    Rc::new(RefCell::new(XesEventImpl::new_with_min_date(ARTIFICIAL_START_EVENT_NAME.to_string()))),
    Rc::new(RefCell::new(XesEventImpl::new_with_min_date(ARTIFICIAL_END_EVENT_NAME.to_string()))),
  );

  let log = log.traces().iter().map(|t| t.borrow().events().clone()).collect();

  Ok(discover_lcs_graph(&log, &name_extractor, &artificial_start_end_events_factory, root_sequence_kind))
}

pub fn discover_lcs_graph<T: PartialEq + Clone + Debug>(
  log: &Vec<Vec<T>>,
  name_extractor: &dyn Fn(&T) -> HeapedOrOwned<String>,
  artificial_start_end_events_factory: &dyn Fn() -> (T, T),
  root_sequence_kind: RootSequenceKind,
) -> DefaultGraph {
  let root_sequence = discover_root_sequence(log, root_sequence_kind);

  if root_sequence.len() == 2 {
    return handle_recursion_exit_case(log, &root_sequence, name_extractor);
  }

  let mut graph = DefaultGraph::empty();
  let lcs_node_ids = initialize_lcs_graph_with_root_sequence(&root_sequence, &mut graph, &name_extractor);

  adjust_lcs_graph_with_traces(log, &root_sequence, &lcs_node_ids, &mut graph,
                               &name_extractor, artificial_start_end_events_factory, root_sequence_kind);

  graph
}

fn handle_recursion_exit_case<T: PartialEq + Clone + Debug>(
  log: &Vec<Vec<T>>,
  root_sequence: &Vec<T>,
  name_extractor: &dyn Fn(&T) -> HeapedOrOwned<String>,
) -> DefaultGraph {
  let mut graph = DefaultGraph::empty();
  let start_node = graph.add_node(Some(name_extractor(root_sequence.first().unwrap())));
  let end_node = graph.add_node(Some(name_extractor(root_sequence.last().unwrap())));

  for trace in log {
    let mut prev_node_id = start_node;
    for event in trace.iter().skip(1).take(trace.len() - 2) {
      let node_id = graph.add_node(Some(name_extractor(event)));
      graph.connect_nodes(&prev_node_id, &node_id, NodesConnectionData::empty());
      prev_node_id = node_id;
    }

    graph.connect_nodes(&prev_node_id, &end_node, NodesConnectionData::empty());
  }

  graph
}

fn initialize_lcs_graph_with_root_sequence<T: PartialEq + Clone>(
  root_sequence: &Vec<T>,
  graph: &mut DefaultGraph,
  name_extractor: &dyn Fn(&T) -> HeapedOrOwned<String>,
) -> Vec<u64> {
  let mut prev_node_id = None;
  let mut root_sequence_node_ids = vec![];

  for event in root_sequence {
    let node_id = graph.add_node(Some(name_extractor(event)));
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
  name_extractor: &dyn Fn(&T) -> HeapedOrOwned<String>,
  artificial_start_end_events_factory: &dyn Fn() -> (T, T),
  root_sequence_kind: RootSequenceKind,
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

  add_adjustments_to_graph(&adjustments, graph, name_extractor, artificial_start_end_events_factory, root_sequence_kind);
}

fn add_adjustments_to_graph<T: PartialEq + Clone + Debug>(
  adjustments: &HashMap<(u64, u64), Vec<Vec<T>>>,
  graph: &mut DefaultGraph,
  name_extractor: &dyn Fn(&T) -> HeapedOrOwned<String>,
  artificial_start_end_events_factory: &dyn Fn() -> (T, T),
  root_sequence_kind: RootSequenceKind,
) {
  for ((start_root_node_id, end_root_node_id), adjustments) in adjustments {
    let adjustment_log = create_log_from_adjustments(&adjustments, artificial_start_end_events_factory);
    let sub_graph = discover_lcs_graph(&adjustment_log, &name_extractor, &artificial_start_end_events_factory, root_sequence_kind);

    merge_subgraph_into_model(graph, &sub_graph, *start_root_node_id, *end_root_node_id, name_extractor, artificial_start_end_events_factory);
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
  sub_graph: &DefaultGraph,
  start_graph_node_id: u64,
  end_graph_node_id: u64,
  name_extractor: &dyn Fn(&T) -> HeapedOrOwned<String>,
  artificial_start_end_events_factory: &dyn Fn() -> (T, T),
) {
  let (start_node_id, end_node_id) = find_start_end_node_ids(&sub_graph, name_extractor, artificial_start_end_events_factory);
  let mut sub_graph_nodes_to_nodes = HashMap::new();

  for node in sub_graph.all_nodes() {
    if *node.id() != start_node_id && *node.id() != end_node_id {
      sub_graph_nodes_to_nodes.insert(node.id(), graph.add_node(node.data.clone()));
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