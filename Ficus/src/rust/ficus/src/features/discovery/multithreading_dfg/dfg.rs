use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::event_log::xes::xes_trace::XesTraceImpl;
use crate::features::discovery::timeline::events_groups::EventGroup;
use crate::features::discovery::timeline::software_data::extraction_config::SoftwareDataExtractionConfig;
use crate::features::discovery::timeline::utils::extract_thread_id;
use crate::utils::graph::graph::{DefaultGraph, NodesConnectionData};
use crate::utils::references::HeapedOrOwned;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use fancy_regex::Regex;

pub enum MultithreadedTracePartsCreationStrategy {
  Regexes(Vec<Regex>),
  Default,
}

pub fn discover_multithreaded_dfg(log: &XesEventLogImpl, thread_attribute: &str, strategy: &MultithreadedTracePartsCreationStrategy) -> DefaultGraph {
  let mut dfg = HashMap::new();
  for trace in log.traces() {
    let trace_dfg = discover_multithreading_dfg_for_trace(&trace.borrow(), thread_attribute, strategy);
    for ((first, second), count) in trace_dfg {
      *dfg.entry((first.to_owned(), second.to_owned())).or_insert(0) += count;
    }
  }

  let mut graph = DefaultGraph::empty();
  let mut added_nodes = HashMap::new();

  for ((first, second), count) in dfg {
    let first_node_id = add_node(first, &mut graph, &mut added_nodes);
    let second_node_id = add_node(second, &mut graph, &mut added_nodes);

    graph.connect_nodes(&first_node_id, &second_node_id, NodesConnectionData::new(None, count as f64, None));
  }

  graph
}

pub fn enumerate_multithreaded_events_groups(
  log: &XesEventLogImpl,
  config: &SoftwareDataExtractionConfig,
  thread_attribute: &str,
  strategy: &MultithreadedTracePartsCreationStrategy
) -> Result<Vec<Vec<EventGroup>>, String> {
  let mut groups = vec![];
  let regexes = config.control_flow_regexes()?;
  let regexes = regexes.as_ref();

  let is_control_flow_event = |event: &XesEventImpl| {
    regexes.is_none() || regexes.unwrap().iter().any(|r| r.is_match(event.name()).unwrap_or(false))
  };

  for trace in log.traces() {
    let trace = trace.borrow();
    let parts = enumerate_trace_parts(&trace, thread_attribute, strategy);

    let mut index = 0;
    let mut trace_groups = vec![];

    for part in parts {
      match part {
        TracePart::Multithreaded(_) => {
          let mut group = EventGroup::empty();
          for event in &trace.events()[index..index + part.length()] {
            if is_control_flow_event(&event.borrow()) {
              group.control_flow_events_mut().push(event.clone());
            } else {
              group.statistic_events_mut().push(event.clone());
            }
          }

          trace_groups.push(group);
        }
        TracePart::Sequential(_) => {
          let mut last_group = None;
          for event in &trace.events()[index..index + part.length()] {
            if is_control_flow_event(&event.borrow()) {
              if let Some(group) = last_group {
                trace_groups.push(group);
              }

              last_group = Some(EventGroup::empty());
              last_group.as_mut().unwrap().control_flow_events_mut().push(event.clone());
            } else {
              if let Some(group) = last_group.as_mut() { 
                group.statistic_events_mut().push(event.clone());
              }
            }
          }
          
          if let Some(group) = last_group {
            trace_groups.push(group);
          }
        }
      }

      index += part.length();
    }

    groups.push(trace_groups);
  }

  Ok(groups)
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

#[derive(Debug)]
enum TracePart {
  Multithreaded(usize),
  Sequential(usize),
}

impl TracePart {
  pub fn length(&self) -> usize {
    match self {
      TracePart::Multithreaded(length) => *length,
      TracePart::Sequential(length) => *length
    }
  }

  pub fn process(
    &self,
    trace: &XesTraceImpl,
    thread_attribute: &str,
    last_event_classes: &mut HashSet<String>,
    dfg: &mut HashMap<(String, String), usize>,
    index: usize,
  ) {
    match self {
      TracePart::Multithreaded(_) => self.process_multithreaded_part(trace, thread_attribute, last_event_classes, dfg, index),
      TracePart::Sequential(_) => self.process_sequential_part(trace, last_event_classes, dfg, index)
    }
  }

  fn process_multithreaded_part(
    &self,
    trace: &XesTraceImpl,
    thread_attribute: &str,
    last_event_classes: &mut HashSet<String>,
    dfg: &mut HashMap<(String, String), usize>,
    index: usize,
  ) {
    let events = &trace.events()[index..index + self.length()];
    let mut events_by_threads = HashMap::new();

    for event in events {
      let thread_id = extract_thread_id::<XesEventImpl>(&event.borrow(), thread_attribute);
      events_by_threads.entry(thread_id).or_insert(vec![]).push(event.clone());
    }

    for last_seen_class in last_event_classes.iter() {
      for first_event in events_by_threads.values().map(|es| es.first().unwrap()) {
        *dfg.entry((last_seen_class.to_owned(), first_event.borrow().name().to_owned())).or_insert(0) += 1;
      }
    }

    last_event_classes.clear();
    for last_event in events_by_threads.values().map(|es| es.last().unwrap()) {
      last_event_classes.insert(last_event.borrow().name().to_string());
    }

    for events in events_by_threads.values() {
      Self::add_dfg_relations_from_trace(events, dfg);
    }
  }

  fn process_sequential_part(
    &self,
    trace: &XesTraceImpl,
    last_event_classes: &mut HashSet<String>,
    dfg: &mut HashMap<(String, String), usize>,
    index: usize,
  ) {
    let events = &trace.events()[index..index + self.length()];
    for last_seen_class in last_event_classes.iter() {
      *dfg.entry((last_seen_class.to_owned(), events.first().unwrap().borrow().name().to_owned())).or_insert(0) += 1;
    }

    last_event_classes.clear();
    last_event_classes.insert(events.last().unwrap().borrow().name().to_string());

    Self::add_dfg_relations_from_trace(events, dfg);
  }

  fn add_dfg_relations_from_trace(events: &[Rc<RefCell<XesEventImpl>>], dfg: &mut HashMap<(String, String), usize>) {
    for i in 0..events.len() - 1 {
      *dfg.entry((events[i].borrow().name().to_owned(), events[i + 1].borrow().name().to_owned())).or_insert(0) += 1;
    }
  }
}

fn discover_multithreading_dfg_for_trace(
  trace: &XesTraceImpl,
  thread_attribute: &str,
  strategy: &MultithreadedTracePartsCreationStrategy,
) -> HashMap<(String, String), usize> {
  let trace_parts = enumerate_trace_parts(trace, thread_attribute, strategy);

  let mut index = 0;
  let mut last_event_classes = HashSet::new();
  let mut dfg = HashMap::new();

  for part in trace_parts {
    part.process(trace, thread_attribute, &mut last_event_classes, &mut dfg, index);
    index += part.length();
  }

  dfg
}

fn enumerate_trace_parts(trace: &XesTraceImpl, thread_attribute: &str, strategy: &MultithreadedTracePartsCreationStrategy) -> Vec<TracePart> {
  let events_threads = if let MultithreadedTracePartsCreationStrategy::Default = strategy {
    let mut events_threads = HashMap::new();
    for event in trace.events() {
      let thread_id = extract_thread_id::<XesEventImpl>(&event.borrow(), thread_attribute);
      events_threads.entry(event.borrow().name().as_str().to_string()).or_insert(HashSet::new()).insert(thread_id);
    }

    Some(events_threads)
  } else {
    None
  };

  let is_sequential = |index: usize| {
    let event = trace.events().get(index).unwrap().borrow();
    let name = event.name().as_str();

    match strategy {
      MultithreadedTracePartsCreationStrategy::Regexes(regexes) => {
        regexes.iter().any(|r| r.is_match(name).unwrap_or(false))
      }
      MultithreadedTracePartsCreationStrategy::Default => {
        events_threads.as_ref().unwrap().get(name).unwrap().len() == 1
      }
    }
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

  trace_parts
}