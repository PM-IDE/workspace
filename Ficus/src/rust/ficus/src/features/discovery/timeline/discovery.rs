use crate::features::discovery::timeline::events_groups::{discover_events_groups, TraceEventsGroup};
use crate::features::discovery::timeline::utils::{extract_thread_id, get_stamp};
use crate::pipelines::errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError};
use crate::{
  event_log::core::event_log::EventLog,
  event_log::core::trace::trace::Trace,
  event_log::xes::xes_event::XesEventImpl,
  event_log::xes::xes_event_log::XesEventLogImpl,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct LogTimelineDiagram {
  pub(in crate::features) thread_attribute: String,
  pub(in crate::features) time_attribute: Option<String>,
  pub(in crate::features) traces: Vec<TraceTimelineDiagram>,
}

impl LogTimelineDiagram {
  pub fn traces(&self) -> &Vec<TraceTimelineDiagram> {
    &self.traces
  }
  pub fn thread_attribute(&self) -> &str { self.thread_attribute.as_str() }
  pub fn time_attribute(&self) -> Option<&String> { self.time_attribute.as_ref() }
}

#[derive(Debug, Clone)]
pub struct LogPoint {
  pub(in crate::features) trace_index: usize,
  pub(in crate::features) event_index: usize,
}

impl LogPoint {
  pub fn trace_index(&self) -> usize { self.trace_index }
  pub fn event_index(&self) -> usize { self.event_index }
}

#[derive(Debug, Clone)]
pub struct TraceTimelineDiagram {
  pub(in crate::features) threads: Vec<TraceThread>,
  pub(in crate::features) events_groups: Vec<TraceEventsGroup>,
}

impl TraceTimelineDiagram {
  pub fn threads(&self) -> &Vec<TraceThread> {
    &self.threads
  }

  pub fn events_groups(&self) -> &Vec<TraceEventsGroup> { &self.events_groups }
}

#[derive(Debug, Clone)]
pub struct TraceThread {
  pub(in crate::features) events: Vec<TraceThreadEvent>,
}

impl TraceThread {
  pub fn empty() -> Self {
    Self {
      events: vec![]
    }
  }

  pub fn events(&self) -> &Vec<TraceThreadEvent> {
    &self.events
  }
  pub fn events_mut(&mut self) -> &mut Vec<TraceThreadEvent> { &mut self.events }
}

#[derive(Debug, Clone)]
pub struct TraceThreadEvent {
  pub(in crate::features) original_event: Rc<RefCell<XesEventImpl>>,
  pub(in crate::features) stamp: u64,
}

impl TraceThreadEvent {
  pub fn new(original_event: Rc<RefCell<XesEventImpl>>, stamp: u64) -> Self {
    Self {
      original_event,
      stamp,
    }
  }

  pub fn original_event(&self) -> &Rc<RefCell<XesEventImpl>> {
    &self.original_event
  }

  pub fn stamp(&self) -> u64 {
    self.stamp
  }
}

pub enum LogThreadsDiagramError {
  NotSupportedEventStamp,
}

impl Debug for LogThreadsDiagramError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    Display::fmt(self, f)
  }
}

impl Display for LogThreadsDiagramError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      LogThreadsDiagramError::NotSupportedEventStamp => f.write_str("NotSupportedEventStamp")
    }
  }
}

impl Error for LogThreadsDiagramError {}

impl Into<PipelinePartExecutionError> for LogThreadsDiagramError {
  fn into(self) -> PipelinePartExecutionError {
    PipelinePartExecutionError::Raw(RawPartExecutionError::new(self.to_string()))
  }
}

pub fn discover_traces_timeline_diagram(
  log: &XesEventLogImpl,
  time_attribute: Option<&String>,
  event_group_delta: Option<u64>,
  discover_event_groups_in_each_trace: bool,
) -> Result<LogTimelineDiagram, LogThreadsDiagramError> {
  let mut threads = vec![];

  for trace in log.traces().iter().map(|t| t.borrow()) {
    let mut thread_events = vec![];
    let min_stamp = get_stamp(&trace.events().first().unwrap().borrow(), time_attribute)?;

    for event in trace.events() {
      thread_events.push(TraceThreadEvent {
        original_event: event.clone(),
        stamp: get_stamp(&event.borrow(), time_attribute)? - min_stamp,
      })
    }

    threads.push(TraceThread {
      events: thread_events
    });
  }

  let timeline_fragments = match discover_event_groups_in_each_trace {
    true => {
      let mut fragments = vec![];
      for thread in threads {
        let events_groups = discover_events_groups_internal(&vec![&thread], event_group_delta);
        fragments.push(TraceTimelineDiagram {
          threads: vec![thread],
          events_groups,
        });
      }

      fragments
    }
    false => {
      let events_groups = discover_events_groups_internal(&threads.iter().collect(), event_group_delta);
      vec![TraceTimelineDiagram {
        threads,
        events_groups,
      }]
    }
  };

  Ok(LogTimelineDiagram {
    thread_attribute: "Trace".to_string(),
    time_attribute: match time_attribute {
      None => None,
      Some(s) => Some(s.to_owned()),
    },
    traces: timeline_fragments,
  })
}

fn discover_events_groups_internal(threads: &Vec<&TraceThread>, event_group_delta: Option<u64>) -> Vec<TraceEventsGroup> {
  if let Some(event_group_delta) = event_group_delta {
    discover_events_groups(threads, event_group_delta)
  } else {
    vec![]
  }
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
      let thread_id = extract_thread_id(event.borrow().deref(), thread_attribute);

      let thread_event = TraceThreadEvent {
        original_event: event.clone(),
        stamp: get_stamp(&event.borrow(), time_attribute)? - min_stamp,
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

  Ok(LogTimelineDiagram {
    thread_attribute: thread_attribute.to_string(),
    time_attribute: match time_attribute {
      None => None,
      Some(s) => Some(s.to_owned()),
    },
    traces,
  })
}
