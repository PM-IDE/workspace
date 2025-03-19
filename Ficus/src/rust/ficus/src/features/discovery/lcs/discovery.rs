use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::event_log::xes::xes_trace::XesTraceImpl;
use crate::features::mutations::mutations::{ARTIFICIAL_END_EVENT_NAME, ARTIFICIAL_START_EVENT_NAME};
use crate::utils::distance::distance::calculate_lcs_distance;
use crate::utils::graph::graph::{DefaultGraph, NodesConnectionData};
use crate::utils::lcs::find_longest_common_subsequence_length;
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

  let mut graph = DefaultGraph::empty();

  let lcs = discover_root_sequence(log);
  let mut indices = vec![1; log.traces().len()];

  let mut last_lcs_node_id = graph.add_node(Some(HeapedOrOwned::Owned(ARTIFICIAL_START_EVENT_NAME.to_string())));

  for event in lcs.iter().skip(1) {
    let mut events_before = vec![];

    for (index, trace) in log.traces().iter().enumerate() {
      let trace = trace.borrow();
      let events = trace.events();

      let mut current_events_before = vec![];
      let mut trace_index = indices[index];

      while trace_index < events.len() && !events[trace_index].borrow().eq(&event.borrow()) {
        current_events_before.push(events[trace_index].clone());
        trace_index += 1;
      }

      indices[index] = trace_index + 1;

      events_before.push(current_events_before);
    }

    let current_lcs_node_id = graph.add_node(Some(HeapedOrOwned::Heaped(event.borrow().name_pointer().clone())));

    for trace_events_before in events_before {
      let mut prev_node_id = last_lcs_node_id;
      for event_before in trace_events_before {
        let node_id = graph.add_node(Some(HeapedOrOwned::Heaped(event_before.borrow().name_pointer().clone())));
        graph.connect_nodes(&prev_node_id, &node_id, NodesConnectionData::empty());
        prev_node_id = node_id;
      }

      graph.connect_nodes(&prev_node_id, &current_lcs_node_id, NodesConnectionData::empty());
    }

    last_lcs_node_id = current_lcs_node_id;
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

  log.traces().get(root_trace_index).unwrap().borrow().events().iter().map(|c| c.clone()).collect()
}