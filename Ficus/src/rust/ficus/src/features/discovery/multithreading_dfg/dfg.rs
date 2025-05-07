use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::event_log::xes::xes_trace::XesTraceImpl;
use crate::features::discovery::timeline::utils::extract_thread_id;
use std::collections::{HashMap, HashSet};
use crate::utils::graph::graph::{DefaultGraph, NodesConnectionData};
use crate::utils::references::HeapedOrOwned;

pub fn discover_multithreaded_dfg(log: &XesEventLogImpl, thread_attribute: &str) -> DefaultGraph {
  let mut dfg = HashMap::new();
  for trace in log.traces() {
    let trace_dfg = discover_multithreading_dfg_for_trace(&trace.borrow(), thread_attribute);
    for ((first, second), count) in trace_dfg {
      *dfg.entry((first.to_owned(), second.to_owned())).or_insert(0) += count;
    }
  }
  
  let mut graph = DefaultGraph::empty();
  let mut added_nodes = HashMap::new();
  
  for ((first, second), count) in dfg {
    let first_node_id = add_node(first, &mut graph, &mut added_nodes);
    let second_node_id = add_node(second, &mut graph, &mut added_nodes);

    graph.connect_nodes(&first_node_id, &second_node_id, NodesConnectionData::new(None, count as f64));
  }

  graph
}

fn add_node(name: String, graph: &mut DefaultGraph, added_nodes: &mut HashMap<String, u64>) -> u64 {
  if let Some(node_id) = added_nodes.get(name.as_str()) {
    *node_id
  } else {
    let node_id = graph.add_node(Some(HeapedOrOwned::Owned(name.clone())));
    added_nodes.insert(name, node_id);
    node_id
  }
}

enum TracePart {
  Multithreaded(usize),
  Sequential(usize),
}

fn discover_multithreading_dfg_for_trace(trace: &XesTraceImpl, thread_attribute: &str) -> HashMap<(String, String), usize> {
  let mut events_threads = HashMap::new();
  for event in trace.events() {
    let thread_id = extract_thread_id::<XesEventImpl>(&event.borrow(), thread_attribute);
    events_threads.entry(event.borrow().name().as_str().to_string()).or_insert(HashSet::new()).insert(thread_id);
  }

  let is_sequential = |index: usize| {
    events_threads.get(trace.events().get(index).unwrap().borrow().name().as_str()).unwrap().len() == 1
  };

  let mut trace_parts = vec![];
  let mut index = 0;
  loop {
    if index >= trace.events().len() {
      break;
    }

    let first_group_event_sequential = is_sequential(index);

    let group_start_index = index;
    let mut group_current_index = index + 1;

    while group_current_index < trace.events().len() && !(first_group_event_sequential ^ is_sequential(group_current_index)) {
      group_current_index += 1;
    }

    let length = group_current_index - group_start_index;
    trace_parts.push(match first_group_event_sequential {
      true => TracePart::Sequential(length),
      false => TracePart::Multithreaded(length)
    });

    index = group_current_index;
  }

  index = 0;
  let mut last_event_classes = HashSet::new();
  let mut dfg = HashMap::new();
  let mut increment = |first: &String, second: &String| {
    *dfg.entry((first.to_owned(), second.to_owned())).or_insert(0) += 1;
  };

  for part in trace_parts {
    let length = match part {
      TracePart::Multithreaded(length) => length,
      TracePart::Sequential(length) => length
    };

    match part {
      TracePart::Multithreaded(_) => {
        let events = &trace.events()[index..index + length];
        let mut events_by_threads = HashMap::new();

        for event in events {
          let thread_id = extract_thread_id::<XesEventImpl>(&event.borrow(), thread_attribute);
          events_by_threads.entry(thread_id).or_insert(vec![]).push(event.clone());
        }

        for last_seen_class in &last_event_classes {
          for first_event in events_by_threads.values().map(|es| es.first().unwrap()) {
            increment(last_seen_class, first_event.borrow().name());
          }
        }

        last_event_classes.clear();
        for last_event in events_by_threads.values().map(|es| es.last().unwrap()) {
          last_event_classes.insert(last_event.borrow().name().to_string());
        }

        for events in events_by_threads.values() {
          for i in 0..events.len() - 1 {
            increment(events[i].borrow().name(), events[i + 1].borrow().name());
          }
        }
      }
      TracePart::Sequential(_) => {
        let events = &trace.events()[index..index + length];
        for last_seen_class in &last_event_classes {
          increment(last_seen_class, events.first().unwrap().borrow().name())
        }

        last_event_classes.clear();
        last_event_classes.insert(events.last().unwrap().borrow().name().to_string());

        for i in 0..events.len() - 1 {
          increment(events[i].borrow().name(), events[i + 1].borrow().name());
        }
      }
    }
    
    index += length;
  }

  dfg
}