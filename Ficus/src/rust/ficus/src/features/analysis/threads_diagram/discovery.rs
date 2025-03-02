use crate::event_log::core::event::event::{Event, EventPayloadValue};
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use std::collections::HashMap;
use std::ops::Deref;

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
pub struct LogPoint {
  trace_index: usize,
  event_index: usize
}

#[derive(Debug, Clone)]
pub struct TraceEventsGroup {
  start_point: LogPoint,
  end_point: LogPoint
}

#[derive(Debug, Clone)]
pub struct TraceTimelineDiagram {
  threads: Vec<TraceThread>,
  events_groups: Vec<TraceEventsGroup>
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
  event_group_delta: u64
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

    let events_groups = discover_events_groups(&threads.values().collect(), event_group_delta);
    traces.push(TraceTimelineDiagram {
      threads: threads.into_iter().map(|(_, v)| v).collect(),
      events_groups
    })
  }

  Ok(LogTimelineDiagram { traces })
}

fn discover_events_groups(threads: &Vec<&TraceThread>, event_group_delta: u64) -> Vec<TraceEventsGroup> {
  let mut groups = vec![];

  let mut last_stamp: Option<u64> = None;
  let mut last_trace_group: Option<TraceEventsGroup> = None;

  let mut events = ThreadsSequentialEvents::new(threads);

  while let Some((event, trace_index, event_index)) = events.next() {
    if last_stamp.is_some() {
      if event.stamp - last_stamp.unwrap() > event_group_delta {
        let mut adjusted_last_group = last_trace_group.unwrap().clone();
        adjusted_last_group.end_point = LogPoint {
          event_index,
          trace_index
        };

        groups.push(adjusted_last_group);
        last_trace_group = None;
        last_stamp = None;
        continue;
      }
    } else {
      last_trace_group = Some(TraceEventsGroup {
        start_point: LogPoint {
          event_index,
          trace_index
        },
        end_point: LogPoint {
          event_index,
          trace_index
        }
      })
    }

    last_stamp = Some(event.stamp.clone());
  }

  groups
}

struct ThreadsSequentialEvents<'a> {
  threads: &'a Vec<&'a TraceThread>,
  indices: Vec<usize>
}

impl<'a> ThreadsSequentialEvents<'a> {
  pub fn new(threads: &'a Vec<&'a TraceThread>) -> Self {
    Self {
      threads,
      indices: vec![0; threads.len()]
    }
  }

  pub fn next(&mut self) -> Option<(&TraceThreadEvent, usize, usize)> {
    let mut min_stamp = 0;
    let mut min_index = 0;

    for i in 1..self.indices.len() {
      if self.indices[i] >= self.threads[i].events.len() {
        continue;
      }

      let stamp = self.get_stamp(i);
      if stamp < min_stamp {
        min_stamp = stamp;
        min_index = i;
      }
    }

    if self.indices[min_index] >= self.threads[min_index].events.len() {
      None
    } else {
      self.indices[min_index] += 1;
      Some((
        self.threads.get(min_index).unwrap().events.get(self.indices[min_index] - 1).as_ref().unwrap(),
        min_index,
        self.indices[min_index] - 1
      ))
    }
  }

  fn get_stamp(&self, index: usize) -> u64 {
    self.get_trace_event(index).stamp
  }

  fn get_trace_event(&self, index: usize) -> &TraceThreadEvent {
    self.threads.get(index).unwrap().events.get(self.indices[index]).as_ref().unwrap()
  }
}

pub fn extract_thread_id<TEvent: Event>(event: &TEvent, thread_attribute: &str) -> Option<String> {
  if let Some(map) = event.payload_map() {
    if let Some(value) = map.get(thread_attribute) {
      Some(value.to_string_repr().as_str().to_owned())
    } else {
      None
    }
  } else {
    None
  }
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
