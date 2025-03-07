use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::features::analysis::threads_diagram::events_groups::{discover_events_groups, TraceEventsGroup};
use crate::features::analysis::threads_diagram::utils::{extract_thread_id, get_stamp};
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct LogTimelineDiagram {
  pub(super) traces: Vec<TraceTimelineDiagram>,
}

impl LogTimelineDiagram {
  pub fn traces(&self) -> &Vec<TraceTimelineDiagram> {
    &self.traces
  }
}

#[derive(Debug, Clone)]
pub struct LogPoint {
  pub(super) trace_index: usize,
  pub(super) event_index: usize,
}

impl LogPoint {
  pub fn trace_index(&self) -> usize { self.trace_index }
  pub fn event_index(&self) -> usize { self.event_index }
}

#[derive(Debug, Clone)]
pub struct TraceTimelineDiagram {
  pub(super) threads: Vec<TraceThread>,
  pub(super) events_groups: Vec<TraceEventsGroup>,
}

impl TraceTimelineDiagram {
  pub fn threads(&self) -> &Vec<TraceThread> {
    &self.threads
  }

  pub fn events_groups(&self) -> &Vec<TraceEventsGroup> { &self.events_groups }
}

#[derive(Debug, Clone)]
pub struct TraceThread {
  pub(super) events: Vec<TraceThreadEvent>,
}

impl TraceThread {
  pub fn events(&self) -> &Vec<TraceThreadEvent> {
    &self.events
  }
}

#[derive(Debug, Clone)]
pub struct TraceThreadEvent {
  pub(super) name: String,
  pub(super) stamp: u64,
}

impl TraceThreadEvent {
  pub fn name(&self) -> &str {
    self.name.as_str()
  }

  pub fn stamp(&self) -> u64 {
    self.stamp
  }
}

pub enum LogThreadsDiagramError {
  NotSupportedEventStamp,
}

pub fn discover_timeline_diagram(
  log: &XesEventLogImpl,
  thread_attribute: &str,
  time_attribute: Option<&String>,
  event_group_delta: Option<u64>,
) -> Result<LogTimelineDiagram, LogThreadsDiagramError> {
  let mut traces = vec![];

  for trace in log.traces() {
    let trace = trace.borrow();
    if trace.events().is_empty() {
      continue;
    }

    let min_stamp = get_stamp(&trace.events().first().unwrap().borrow(), time_attribute)?;
    let mut threads: HashMap<Option<String>, TraceThread> = HashMap::new();

    for i in 0..trace.events().len() {
      let event = trace.events().get(i).expect("Must be in range");
      let event = event.borrow();

      let thread_id = extract_thread_id(event.deref(), thread_attribute);

      let thread_event = TraceThreadEvent {
        name: event.name().to_owned(),
        stamp: get_stamp(&event, time_attribute)? - min_stamp,
      };

      if let Some(thread) = threads.get_mut(&thread_id) {
        thread.events.push(thread_event);
      } else {
        threads.insert(
          thread_id,
          TraceThread {
            events: vec![thread_event],
          },
        );
      }
    }

    let events_groups = if let Some(event_group_delta) = event_group_delta {
      discover_events_groups(&threads.values().collect(), event_group_delta)
    } else {
      vec![]
    };

    traces.push(TraceTimelineDiagram {
      threads: threads.into_iter().map(|(_, v)| v).collect(),
      events_groups,
    })
  }

  Ok(LogTimelineDiagram { traces })
}
