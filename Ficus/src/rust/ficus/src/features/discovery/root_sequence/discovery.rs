use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::event_log::xes::xes_trace::XesTraceImpl;
use crate::features::analysis::patterns::activity_instances::create_vector_of_underlying_events;
use crate::features::discovery::petri_net::annotations::{create_performance_map, PerformanceMap};
use crate::features::discovery::root_sequence::adjustments::{adjust_weights_and_connections, merge_sequences_of_nodes};
use crate::features::discovery::root_sequence::context::DiscoveryContext;
use crate::features::discovery::root_sequence::models::CorrespondingTraceData;
use crate::features::discovery::root_sequence::root_sequence::discover_root_sequence;
use crate::features::mutations::mutations::{ARTIFICIAL_END_EVENT_NAME, ARTIFICIAL_START_EVENT_NAME};
use crate::pipelines::keys::context_keys::{CORRESPONDING_TRACE_DATA_KEY, SOFTWARE_DATA_KEY, START_END_ACTIVITY_TIME_KEY};
use crate::utils::graph::graph::{DefaultGraph, NodesConnectionData};
use crate::utils::lcs::find_longest_common_subsequence;
use crate::utils::references::HeapedOrOwned;
use crate::utils::user_data::user_data::{ExecuteWithUserData, UserData, UserDataImpl, UserDataOwner};
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

pub fn discover_root_sequence_graph_from_event_log(
  log: &XesEventLogImpl,
  root_sequence_kind: RootSequenceKind,
  merge_sequences_of_events: bool,
) -> Result<DefaultGraph, DiscoverLCSGraphError> {
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
      user_data_impl.put_concrete(CORRESPONDING_TRACE_DATA_KEY.key(), corresponding_trace_data.iter().map(|mut d| {
        let mut data = d.clone();
        data.set_belongs_to_root_sequence(belongs_to_root_sequence);
        data
      }).collect());
    }

    if let Some(start_end_activity_time) = event.borrow().user_data().concrete(START_END_ACTIVITY_TIME_KEY.key()) {
      user_data_impl.put_concrete(START_END_ACTIVITY_TIME_KEY.key(), start_end_activity_time.clone())
    }
  };

  let underlying_events_extractor = |event: &Rc<RefCell<XesEventImpl>>| {
    let underlying_events = create_vector_of_underlying_events::<XesEventLogImpl>(event);
    match underlying_events.is_empty() {
      true => None,
      false => Some(underlying_events)
    }
  };

  let context = DiscoveryContext::new(
    &name_extractor,
    &artificial_start_end_events_factory,
    root_sequence_kind,
    &event_to_graph_node_info_transfer,
    &underlying_events_extractor,
  );

  let performance_map = create_performance_map(log);

  let log = log.traces().iter().map(|t| t.borrow().events().clone()).collect();

  Ok(discover_root_sequence_graph(&log, &context, merge_sequences_of_events, Some(performance_map)))
}

impl ExecuteWithUserData for Rc<RefCell<XesEventImpl>> {
  fn execute_with_user_data(&self, func: &mut dyn FnMut(&UserDataImpl) -> ()) {
    func(self.borrow().user_data());
  }

  fn execute_with_user_data_mut(&mut self, func: &mut dyn FnMut(&mut UserDataImpl)) {
    func(self.borrow_mut().user_data_mut());
  }
}

fn set_corresponding_trace_data(log: &XesEventLogImpl) {
  for (trace_index, trace) in log.traces().iter().enumerate() {
    for (event_index, event) in trace.borrow().events().iter().enumerate() {
      event.borrow_mut().user_data_mut().put_concrete(CORRESPONDING_TRACE_DATA_KEY.key(), vec![
        CorrespondingTraceData::new(trace_index as u64, event_index as u64, false)
      ])
    }
  }
}

pub fn discover_root_sequence_graph<T: PartialEq + Clone + Debug + ExecuteWithUserData>(
  log: &Vec<Vec<T>>,
  context: &DiscoveryContext<T>,
  merge_sequences_of_events: bool,
  performance_map: Option<PerformanceMap>,
) -> DefaultGraph {
  let mut graph = discover_root_sequence_graph_internal(log, context, true);

  adjust_weights_and_connections(context, log, &mut graph);

  if merge_sequences_of_events {
    merge_sequences_of_nodes(&mut graph, performance_map);
  }

  graph
}

fn discover_root_sequence_graph_internal<T: PartialEq + Clone + Debug + ExecuteWithUserData>(
  log: &Vec<Vec<T>>,
  context: &DiscoveryContext<T>,
  first_iteration: bool,
) -> DefaultGraph {
  let root_sequence = discover_root_sequence(log, context.root_sequence_kind());

  if root_sequence.len() == 2 {
    return handle_recursion_exit_case(log, &root_sequence, context);
  }

  let mut graph = DefaultGraph::empty();
  let root_sequence_nodes_ids = initialize_lcs_graph_with_root_sequence(&root_sequence, &mut graph, &context, first_iteration);

  adjust_lcs_graph_with_traces(log, &root_sequence, &root_sequence_nodes_ids, &mut graph, context);

  graph
}

fn handle_recursion_exit_case<T: PartialEq + Clone + Debug + ExecuteWithUserData>(
  log: &Vec<Vec<T>>,
  root_sequence: &Vec<T>,
  context: &DiscoveryContext<T>,
) -> DefaultGraph {
  let mut graph = DefaultGraph::empty();
  let name_extractor = context.name_extractor();
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

fn create_new_graph_node<T: ExecuteWithUserData>(graph: &mut DefaultGraph, event: &T, is_root_sequence: bool, context: &DiscoveryContext<T>) -> u64 {
  let name_extractor = context.name_extractor();
  let node_id = graph.add_node(Some(name_extractor(event)));

  let node = graph.node_mut(&node_id).unwrap();
  let transfer = context.event_to_graph_node_info_transfer();
  transfer(event, node.user_data_mut(), is_root_sequence);

  node_id
}

fn initialize_lcs_graph_with_root_sequence<T: PartialEq + Clone + ExecuteWithUserData>(
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

fn adjust_lcs_graph_with_traces<T: PartialEq + Clone + Debug + ExecuteWithUserData>(
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

      let from_node_id = root_sequence_node_ids[second_indices[lcs_index - 1]];
      let to_node_id = root_sequence_node_ids[second_indices[lcs_index]];

      adjustments.entry(from_node_id).or_insert(HashMap::new()).entry(to_node_id).or_insert(vec![]).push(adjustment_events);

      index += 1;
      lcs_index += 1;
    }
  }

  add_adjustments_to_graph(&adjustments, graph, context);
}

fn add_adjustments_to_graph<T: PartialEq + Clone + Debug + ExecuteWithUserData>(
  adjustments: &HashMap<u64, HashMap<u64, Vec<Vec<T>>>>,
  graph: &mut DefaultGraph,
  context: &DiscoveryContext<T>,
) {
  for (start_root_node_id, adjustments) in adjustments {
    let adjustment_log = create_log_from_adjustments(adjustments.iter().collect(), context.artificial_start_end_events_factory());
    let sub_graph = discover_root_sequence_graph_internal(&adjustment_log, context, false);

    merge_subgraph_into_model(adjustments, graph, sub_graph, *start_root_node_id, context);
  }
}

fn create_log_from_adjustments<T: PartialEq + Clone + Debug + ExecuteWithUserData>(
  end_root_sequence_nodes_to_adjustments: Vec<(&u64, &Vec<Vec<T>>)>,
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
  adjustments: &HashMap<u64, Vec<Vec<T>>>,
  graph: &mut DefaultGraph,
  sub_graph: DefaultGraph,
  start_graph_node_id: u64,
  context: &DiscoveryContext<T>,
) {
  let (start_node_id, end_node_id) = find_start_end_node_ids(&sub_graph, context.name_extractor(), context.artificial_start_end_events_factory());
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

    if *edge.to_node() != end_node_id {
      graph.connect_nodes(&from_node, &sub_graph_nodes_to_nodes[edge.to_node()], NodesConnectionData::empty());
    }
  }

  for (end_node_id, log) in adjustments {
    for trace in log {
      let mut current_node = start_graph_node_id;
      for event in trace.iter() {
        let event_name = context.name_extractor()(event);
        for outgoing_node in graph.outgoing_nodes(&current_node) {
          let node_data = graph.node(outgoing_node).unwrap().data().unwrap();
          if node_data == &event_name {
            current_node = *outgoing_node
          }
        }
      }

      graph.connect_nodes(&current_node, end_node_id, NodesConnectionData::empty());
    }
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