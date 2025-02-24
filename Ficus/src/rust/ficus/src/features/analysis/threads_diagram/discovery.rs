use crate::event_log::core::event::event::{Event, EventPayloadValue};
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LogTimelineDiagram {
  traces: Vec<TraceTimelineDiagram>,
}

impl LogTimelineDiagram {
  pub fn traces(&self) -> &Vec<TraceTimelineDiagram> {
    &self.traces
  }
}

#[derive(Debug, Clone)]
pub struct TraceTimelineDiagram {
  threads: Vec<TraceThread>,
}

impl TraceTimelineDiagram {
  pub fn threads(&self) -> &Vec<TraceThread> {
    &self.threads
  }
}

#[derive(Debug, Clone)]
pub struct TraceThread {
  events: Vec<TraceThreadEvent>,
}

impl TraceThread {
  pub fn events(&self) -> &Vec<TraceThreadEvent> {
    &self.events
  }
}

#[derive(Debug, Clone)]
pub struct TraceThreadEvent {
  name: String,
  stamp: u64,
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
  time_attribute: Option<&str>,
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

      let thread_id = if let Some(map) = event.payload_map() {
        if let Some(value) = map.get(thread_attribute) {
          Some(value.to_string_repr().as_str().to_owned())
        } else {
          None
        }
      } else {
        None
      };

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

    traces.push(TraceTimelineDiagram {
      threads: threads.into_iter().map(|(_, v)| v).collect(),
    })
  }

  Ok(LogTimelineDiagram { traces })
}

fn get_stamp(event: &XesEventImpl, attribute: Option<&str>) -> Result<u64, LogThreadsDiagramError> {
  if let Some(attribute) = attribute {
    if let Some(map) = event.payload_map() {
      if let Some(value) = map.get(attribute) {
        match value {
          EventPayloadValue::Int32(v) => return Ok(*v as u64),
          EventPayloadValue::Int64(v) => return Ok(*v as u64),
          EventPayloadValue::Uint32(v) => return Ok(*v as u64),
          EventPayloadValue::Uint64(v) => return Ok(*v),
          _ => {}
        };
      }
    }
  }

  let utc_stamp = event.timestamp().timestamp_nanos_opt();
  if utc_stamp.is_none() || utc_stamp.unwrap() < 0 {
    Err(LogThreadsDiagramError::NotSupportedEventStamp)
  } else {
    Ok(utc_stamp.unwrap() as u64)
  }
}
