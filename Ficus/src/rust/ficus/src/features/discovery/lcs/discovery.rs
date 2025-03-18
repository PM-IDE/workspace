use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::features::mutations::mutations::{ARTIFICIAL_END_EVENT_NAME, ARTIFICIAL_START_EVENT_NAME};
use crate::utils::graph::graph::{DefaultGraph, NodesConnectionData};
use crate::utils::lcs::find_longest_common_subsequence;
use crate::utils::references::HeapedOrOwned;
use std::cell::RefCell;
use std::rc::Rc;

enum DiscoverLCSGraphError {
  NoArtificialStartEndEvents
}

pub fn discover_lcs_graph(log: &XesEventLogImpl) -> Result<DefaultGraph, DiscoverLCSGraphError> {
  assert_all_traces_have_artificial_start_end_events(log)?;

  let mut graph = DefaultGraph::empty();

  let lcs = discover_lcs(log);
  let indices = vec![1; log.traces().len()];

  let mut last_lcs_node_id = graph.add_node(Some(HeapedOrOwned::Owned(ARTIFICIAL_START_EVENT_NAME.to_string())));

  for event in lcs {
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
    if trace.events().len() == 0 {
      continue;
    }

    if trace.events().first().unwrap().borrow().name().as_str() != ARTIFICIAL_START_EVENT_NAME {
      return Err(DiscoverLCSGraphError::NoArtificialStartEndEvents);
    }

    if trace.events().last().unwrap().borrow().name().as_str() != ARTIFICIAL_END_EVENT_NAME {
      return Err(DiscoverLCSGraphError::NoArtificialStartEndEvents);
    }
  }
  
  Ok(())
}

fn discover_lcs(log: &XesEventLogImpl) -> Vec<Rc<RefCell<XesEventImpl>>> {
  if log.traces().is_empty() {
    return vec![];
  }

  let mut current_lcs: Vec<Rc<RefCell<XesEventImpl>>> = log.traces().first().unwrap().borrow().events().iter().map(|e| e.clone()).collect();
  for trace in log.traces().iter().skip(1) {
    let trace_events = trace.borrow().events().iter().map(|e| e.clone()).collect();
    current_lcs = find_longest_common_subsequence(&current_lcs, &trace_events, current_lcs.len(), trace_events.len())
      .lcs()
      .iter()
      .map(|e| (*e).clone())
      .collect();
  }
  
  current_lcs
}