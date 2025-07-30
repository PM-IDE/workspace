use crate::features::discovery::timeline::events_groups::{discover_events_groups, TraceEventsGroup};
use crate::features::discovery::timeline::utils::{extract_thread_id, get_stamp};
use crate::pipelines::errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError};
use crate::{
  event_log::core::event_log::EventLog,
  event_log::core::trace::trace::Trace,
  event_log::xes::xes_event::XesEventImpl,
  event_log::xes::xes_event_log::XesEventLogImpl,
};
use derive_new::new;
use fancy_regex::Regex;
use getset::{Getters, MutGetters};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, Clone, Getters, new)]
pub struct LogTimelineDiagram {
  #[getset(get = "pub")] thread_attribute: String,
  #[getset(get = "pub")] time_attribute: Option<String>,
  #[getset(get = "pub")] control_flow_regexes: Option<Vec<Regex>>,
  #[getset(get = "pub")] traces: Vec<TraceTimelineDiagram>,
}

impl LogTimelineDiagram {
  pub fn is_control_flow_event(&self, event_class: &str) -> bool {
    if let Some(regexes) = self.control_flow_regexes.as_ref() {
      regexes.iter().any(|r| r.is_match(event_class).unwrap_or(false))
    } else {
      true
    }
  }
}

#[derive(Debug, Clone, Getters, new)]
pub struct LogPoint {
  #[getset(get = "pub")] trace_index: usize,
  #[getset(get = "pub")] event_index: usize,
}

#[derive(Debug, Clone, Getters, new)]
pub struct TraceTimelineDiagram {
  #[getset(get = "pub")] threads: Vec<TraceThread>,
  #[getset(get = "pub")] events_groups: Vec<TraceEventsGroup>,
}

#[derive(Debug, Clone, Getters, MutGetters)]
pub struct TraceThread {
  #[getset(get = "pub", get_mut = "pub")] events: Vec<TraceThreadEvent>,
}

impl TraceThread {
  pub fn empty() -> Self {
    Self {
      events: vec![]
    }
  }
}

#[derive(Debug, Clone, Getters, new)]
pub struct TraceThreadEvent {
  #[getset(get = "pub")] original_event: Rc<RefCell<XesEventImpl>>,
  #[getset(get = "pub")] stamp: u64,
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
  control_flow_regexes: Option<&Vec<Regex>>,
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
        let events_groups = discover_events_groups_internal(&vec![&thread], event_group_delta, control_flow_regexes);
        fragments.push(TraceTimelineDiagram {
          threads: vec![thread],
          events_groups,
        });
      }

      fragments
    }
    false => {
      let events_groups = discover_events_groups_internal(&threads.iter().collect(), event_group_delta, control_flow_regexes);
      vec![TraceTimelineDiagram {
        threads,
        events_groups,
      }]
    }
  };

  Ok(LogTimelineDiagram {
    control_flow_regexes: control_flow_regexes.cloned(),
    thread_attribute: "Trace".to_string(),
    traces: timeline_fragments,
    time_attribute: match time_attribute {
      None => None,
      Some(s) => Some(s.to_owned()),
    },
  })
}

fn discover_events_groups_internal(
  threads: &Vec<&TraceThread>,
  event_group_delta: Option<u64>,
  control_flow_regexes: Option<&Vec<Regex>>,
) -> Vec<TraceEventsGroup> {
  if let Some(event_group_delta) = event_group_delta {
    discover_events_groups(threads, event_group_delta, control_flow_regexes)
  } else {
    vec![]
  }
}

pub fn discover_timeline_diagram(
  log: &XesEventLogImpl,
  thread_attribute: &str,
  time_attribute: Option<&String>,
  event_group_delta: Option<u64>,
  control_flow_regexes: Option<&Vec<Regex>>,
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

    let events_groups = discover_events_groups_internal(&threads.values().collect(), event_group_delta, control_flow_regexes);

    traces.push(TraceTimelineDiagram {
      threads: threads.into_iter().map(|(_, v)| v).collect(),
      events_groups,
    })
  }

  Ok(LogTimelineDiagram {
    control_flow_regexes: control_flow_regexes.cloned(),
    thread_attribute: thread_attribute.to_string(),
    time_attribute: match time_attribute {
      None => None,
      Some(s) => Some(s.to_owned()),
    },
    traces,
  })
}
