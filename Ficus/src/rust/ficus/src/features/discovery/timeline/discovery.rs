use crate::{
  event_log::{
    core::{event_log::EventLog, trace::trace::Trace},
    xes::{xes_event::XesEventImpl, xes_event_log::XesEventLogImpl},
  },
  features::discovery::timeline::{
    events_groups::{TraceEventsGroup, discover_events_groups},
    utils::{extract_thread_id, get_stamp},
  },
  pipelines::errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError},
};
use derive_new::new;
use fancy_regex::Regex;
use getset::{Getters, MutGetters};
use std::{
  cell::RefCell,
  collections::HashMap,
  error::Error,
  fmt::{Debug, Display, Formatter},
  ops::Deref,
  rc::Rc,
};
use std::sync::Arc;

#[derive(Debug, Clone, Getters, new)]
pub struct LogTimelineDiagram {
  #[getset(get = "pub")]
  thread_attribute: Arc<str>,
  #[getset(get = "pub")]
  time_attribute: Option<Arc<str>>,
  #[getset(get = "pub")]
  control_flow_regexes: Option<Vec<Regex>>,
  #[getset(get = "pub")]
  traces: Vec<TraceTimelineDiagram>,
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
  #[getset(get = "pub")]
  trace_index: usize,
  #[getset(get = "pub")]
  event_index: usize,
}

#[derive(Debug, Clone, Getters, new)]
pub struct TraceTimelineDiagram {
  #[getset(get = "pub")]
  threads: Vec<TraceThread>,
  #[getset(get = "pub")]
  events_groups: Vec<TraceEventsGroup>,
}

#[derive(Debug, Clone, Getters, MutGetters, Default)]
pub struct TraceThread {
  #[getset(get = "pub", get_mut = "pub")]
  events: Vec<TraceThreadEvent>,
}

#[derive(Debug, Clone, Getters, new)]
pub struct TraceThreadEvent {
  #[getset(get = "pub")]
  original_event: Rc<RefCell<XesEventImpl>>,
  #[getset(get = "pub")]
  stamp: i64,
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
      LogThreadsDiagramError::NotSupportedEventStamp => f.write_str("NotSupportedEventStamp"),
    }
  }
}

impl Error for LogThreadsDiagramError {}

impl From<LogThreadsDiagramError> for PipelinePartExecutionError {
  fn from(val: LogThreadsDiagramError) -> Self {
    PipelinePartExecutionError::Raw(RawPartExecutionError::new(val.to_string()))
  }
}

pub fn discover_traces_timeline_diagram(
  log: &XesEventLogImpl,
  time_attribute: Option<&Arc<str>>,
  event_group_delta: Option<u64>,
  discover_event_groups_in_each_trace: bool,
  control_flow_regexes: Option<&Vec<Regex>>,
) -> Result<LogTimelineDiagram, LogThreadsDiagramError> {
  let mut threads = vec![];

  for trace in log.traces().iter().map(|t| t.borrow()) {
    let mut thread_events = vec![];
    let time_attribute = time_attribute.map(|a| a.as_ref());
    let min_stamp = get_stamp(&trace.events().first().unwrap().borrow(), time_attribute)?;

    for event in trace.events() {
      thread_events.push(TraceThreadEvent {
        original_event: event.clone(),
        stamp: get_stamp(&event.borrow(), time_attribute)? - min_stamp,
      })
    }

    threads.push(TraceThread { events: thread_events });
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
      vec![TraceTimelineDiagram { threads, events_groups }]
    }
  };

  Ok(LogTimelineDiagram {
    control_flow_regexes: control_flow_regexes.cloned(),
    thread_attribute: Arc::from("Trace".to_string()),
    traces: timeline_fragments,
    time_attribute: time_attribute.cloned(),
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
  thread_attribute: &Arc<str>,
  time_attribute: Option<&Arc<str>>,
  event_group_delta: Option<u64>,
  control_flow_regexes: Option<&Vec<Regex>>,
) -> Result<LogTimelineDiagram, LogThreadsDiagramError> {
  let mut traces = vec![];

  for trace in log.traces() {
    let trace = trace.borrow();
    if trace.events().is_empty() {
      continue;
    }

    let time_attribute = time_attribute.map(|a| a.as_ref());
    let min_stamp = get_stamp(&trace.events().first().unwrap().borrow(), time_attribute)?;
    let mut threads: HashMap<Option<Arc<str>>, TraceThread> = HashMap::new();

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
      threads: threads.into_values().collect(),
      events_groups,
    })
  }

  Ok(LogTimelineDiagram {
    control_flow_regexes: control_flow_regexes.cloned(),
    thread_attribute: thread_attribute.clone(),
    time_attribute: time_attribute.cloned(),
    traces,
  })
}
