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
use log::error;

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

  let mut graph = DefaultGraph::empty();

  let lcs = discover_root_sequence(log);
  let mut prev_node_id = None;
  let mut lcs_node_ids = vec![];

  for event in &lcs {
    let node_id = graph.add_node(Some(HeapedOrOwned::Heaped(event.borrow().name_pointer().clone())));
    lcs_node_ids.push(node_id);

    if let Some(prev_node_id) = prev_node_id.as_ref() {
      graph.connect_nodes(prev_node_id, &node_id, NodesConnectionData::empty());
    }

    prev_node_id = Some(node_id);
  }

  for trace in log.traces().iter().map(|t| t.borrow()) {
    let trace_lcs = find_longest_common_subsequence(trace.events(), &lcs, trace.events().len(), lcs.len());

    let mut lcs_index = 0;
    let mut index = 0;

    while index < trace.events().len() {
      if index == trace_lcs.first_indices()[lcs_index] {
        let second_indices = trace_lcs.second_indices();
        if lcs_index >= 1 && second_indices[lcs_index - 1] + 1 != second_indices[lcs_index] {
          graph.connect_nodes(&lcs_node_ids[second_indices[lcs_index - 1]], &lcs_node_ids[second_indices[lcs_index]], NodesConnectionData::empty());
        }

        lcs_index += 1;
        index += 1;
        continue;
      }

      let mut current_node_id = lcs_node_ids[lcs_index];
      while index < trace.events().len() && index != trace_lcs.first_indices()[lcs_index] {
        let event = trace.events().get(index).unwrap();

        let connected_node_ids = graph.outgoing_nodes(&current_node_id);
        let mut found_existing_node = false;

        for id in connected_node_ids {
          let node = graph.node(id).unwrap();
          if let Some(data) = node.data.as_ref() {
            if data.eq(&HeapedOrOwned::Heaped(event.borrow().name_pointer().clone())) {
              current_node_id = *node.id();
              found_existing_node = true;
            }
          }
        }

        if !found_existing_node {
          let added_node_id = graph.add_node(Some(HeapedOrOwned::Heaped(event.borrow().name_pointer().clone())));
          graph.connect_nodes(&current_node_id, &added_node_id, NodesConnectionData::empty());
          current_node_id = added_node_id;
        }

        index += 1;
      }

      if lcs_index + 1 < lcs_node_ids.len() {
        graph.connect_nodes(&current_node_id, &lcs_node_ids[lcs_index + 1], NodesConnectionData::empty());
      } else {
        error!("Can not connect new path to next LCS node");
      }

      index += 1;
      lcs_index += 1;
    }
  }

  Ok(graph)
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

fn discover_root_sequence(log: &XesEventLogImpl) -> Vec<Rc<RefCell<XesEventImpl>>> {
  if log.traces().is_empty() {
    return vec![];
  }

  let mut root_trace_index = 0;
  let mut root_distance = f64::MAX;
  for (index, trace) in log.traces().iter().map(|t| t.borrow()).enumerate() {
    let trace_events = trace.events();

    let mut summed_distance = 0.;
    for trace in log.traces().iter().map(|t| t.borrow()) {
      let other_trace_events = trace.events();
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
  for (first_index, first_trace) in log.traces().iter().map(|t| t.borrow()).enumerate() {
    for (second_index, second_trace) in log.traces().iter().map(|t| t.borrow()).enumerate() {
      let lcs = find_longest_common_subsequence(first_trace.events(), second_trace.events(), first_trace.events().len(), second_trace.events().len())
        .lcs().into_iter().map(|c| (*c).clone()).collect::<Vec<Rc<RefCell<XesEventImpl>>>>();

      let mut distance = 0.;
      for trace in log.traces().iter().map(|t| t.borrow()) {
        let lcs_length = find_longest_common_subsequence_length(&lcs, trace.events(), lcs.len(), trace.events().len());
        distance += calculate_lcs_distance(lcs_length, lcs.len(), trace.events().len());
      }

      if distance < root_lcs_distance {
        root_lcs_distance = distance;
        indices = (first_index, second_index);
      }
    }
  }

  if root_distance <= root_lcs_distance {
    log.traces().get(root_trace_index).unwrap().borrow().events().iter().map(|c| c.clone()).collect()
  } else {
    let first_trace = log.traces().get(indices.0).unwrap();
    let second_trace = log.traces().get(indices.1).unwrap();

    let first_trace_len = first_trace.borrow().events().len();
    let second_trace_len = second_trace.borrow().events().len();

    find_longest_common_subsequence(first_trace.borrow().events(), second_trace.borrow().events(), first_trace_len, second_trace_len)
      .lcs()
      .into_iter()
      .map(|c| (*c).clone())
      .collect::<Vec<Rc<RefCell<XesEventImpl>>>>()
  }
}