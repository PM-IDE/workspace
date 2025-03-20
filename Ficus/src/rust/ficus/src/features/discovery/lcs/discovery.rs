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
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::rc::Rc;

pub enum DiscoverLCSGraphError {
  NoArtificialStartEndEvents
}

impl Display for DiscoverLCSGraphError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      DiscoverLCSGraphError::NoArtificialStartEndEvents => f.write_str("All traces in event log must have artificial start-end events")
    }
  }
}

pub fn discover_lcs_graph(log: &XesEventLogImpl) -> Result<DefaultGraph, DiscoverLCSGraphError> {
  assert_all_traces_have_artificial_start_end_events(log)?;
  let name_extractor = |e: &Rc<RefCell<XesEventImpl>>| HeapedOrOwned::Heaped(e.borrow().name_pointer().clone());

  let log = log.traces().iter().map(|t| t.borrow().events().clone()).collect();

  Ok(discover_lcs_graph_internal(&log, &name_extractor))
}

fn discover_lcs_graph_internal<T: PartialEq + Clone>(log: &Vec<Vec<T>>, name_extractor: &impl Fn(&T) -> HeapedOrOwned<String>) -> DefaultGraph {
  let mut graph = DefaultGraph::empty();

  let root_sequence = discover_root_sequence(log);

  let lcs_node_ids = initialize_lcs_graph_with_root_sequence(&root_sequence, &mut graph, name_extractor);
  adjust_lcs_graph_with_traces(log, &root_sequence, &lcs_node_ids, &mut graph, name_extractor);

  graph
}

pub fn initialize_lcs_graph_with_root_sequence<T: PartialEq + Clone>(
  root_sequence: &Vec<T>,
  graph: &mut DefaultGraph,
  name_extractor: impl Fn(&T) -> HeapedOrOwned<String>,
) -> Vec<u64> {
  let mut prev_node_id = None;
  let mut lcs_node_ids = vec![];

  for event in root_sequence {
    let node_id = graph.add_node(Some(name_extractor(event)));
    lcs_node_ids.push(node_id);

    if let Some(prev_node_id) = prev_node_id.as_ref() {
      graph.connect_nodes(prev_node_id, &node_id, NodesConnectionData::empty());
    }

    prev_node_id = Some(node_id);
  }

  lcs_node_ids
}

pub fn adjust_lcs_graph_with_traces<T: PartialEq + Clone>(
  traces: &Vec<Vec<T>>,
  lcs: &Vec<T>,
  lcs_node_ids: &Vec<u64>,
  graph: &mut DefaultGraph,
  name_extractor: impl Fn(&T) -> HeapedOrOwned<String>,
) {
  let mut adjustments = vec![vec![]; lcs_node_ids.len()];
  for trace in traces {
    let trace_lcs = find_longest_common_subsequence(trace, &lcs, trace.len(), lcs.len());

    let mut lcs_index = 0;
    let mut index = 0;

    while index < trace.len() {
      if index == trace_lcs.first_indices()[lcs_index] {
        let second_indices = trace_lcs.second_indices();
        if lcs_index >= 1 && second_indices[lcs_index - 1] + 1 != second_indices[lcs_index] {
          graph.connect_nodes(&lcs_node_ids[second_indices[lcs_index - 1]], &lcs_node_ids[second_indices[lcs_index]], NodesConnectionData::empty());
        }

        lcs_index += 1;
        index += 1;
        continue;
      }

      let mut current_adjustments = vec![];
      while index < trace.len() && index != trace_lcs.first_indices()[lcs_index] {
        current_adjustments.push(trace.get(index).unwrap().clone());
        index += 1;
      }

      adjustments.get_mut(lcs_index - 1).unwrap().push(current_adjustments);

      index += 1;
      lcs_index += 1;
    }
  }

  add_adjustments_to_graph(adjustments, graph, lcs_node_ids, name_extractor);
}

fn add_adjustments_to_graph<T: PartialEq + Clone>(
  adjustments: Vec<Vec<Vec<T>>>,
  graph: &mut DefaultGraph,
  lcs_node_ids: &Vec<u64>,
  name_extractor: impl Fn(&T) -> HeapedOrOwned<String>,
) {
  for (index, adjustment) in adjustments.into_iter().enumerate() {
    for events in adjustment {
      let mut current_node_id = lcs_node_ids[index];

      for event in events {
        let connected_node_ids = graph.outgoing_nodes(&current_node_id);
        let mut found_existing_node = false;

        for id in connected_node_ids {
          let node = graph.node(id).unwrap();
          if let Some(data) = node.data.as_ref() {
            if data.eq(&name_extractor(&event)) {
              current_node_id = *node.id();
              found_existing_node = true;
            }
          }
        }

        if !found_existing_node {
          let added_node_id = graph.add_node(Some(name_extractor(&event)));
          graph.connect_nodes(&current_node_id, &added_node_id, NodesConnectionData::empty());
          current_node_id = added_node_id;
        }
      }

      graph.connect_nodes(&current_node_id, &lcs_node_ids[index + 1], NodesConnectionData::empty());
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

fn discover_root_sequence<T: PartialEq + Clone>(log: &Vec<Vec<T>>) -> Vec<T> {
  if log.is_empty() {
    return vec![];
  }

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

  let mut root_lcs_distance = f64::MAX;
  let mut indices = (0, 0);
  for (first_index, first_trace) in log.iter().enumerate() {
    for (second_index, second_trace) in log.iter().enumerate() {
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

  if root_distance <= root_lcs_distance {
    log.get(root_trace_index).unwrap().iter().map(|c| c.clone()).collect()
  } else {
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
}