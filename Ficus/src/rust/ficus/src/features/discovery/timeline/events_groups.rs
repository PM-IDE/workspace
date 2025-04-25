use crate::event_log::core::event::event::Event;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::discovery::{LogPoint, LogTimelineDiagram, TraceThread, TraceThreadEvent};
use fancy_regex::Regex;
use getset::{Getters, MutGetters, Setters};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct TraceEventsGroup {
  start_point: LogPoint,
  end_point: LogPoint,
}

impl TraceEventsGroup {
  pub fn start_point(&self) -> &LogPoint {
    &self.start_point
  }

  pub fn end_point(&self) -> &LogPoint {
    &self.end_point
  }
}

pub fn discover_events_groups(threads: &Vec<&TraceThread>, event_group_delta: u64, control_flow_regexes: Option<&Vec<Regex>>) -> Vec<TraceEventsGroup> {
  let mut groups = vec![];

  let mut last_stamp: Option<u64> = None;
  let mut last_trace_group: Option<TraceEventsGroup> = None;

  let mut events = ThreadsSequentialEvents::new(threads);
  let mut last_seen_point: Option<(usize, usize)> = None;

  let mut add_to_groups = |last_trace_group: Option<TraceEventsGroup>, last_seen_point: Option<(usize, usize)>| {
    let mut adjusted_last_group = last_trace_group.unwrap().clone();
    adjusted_last_group.end_point = LogPoint::new(last_seen_point.unwrap().0, last_seen_point.unwrap().1);

    groups.push(adjusted_last_group);
  };

  while let Some((event, trace_index, event_index)) = events.next() {
    if let Some(control_flow_regexes) = control_flow_regexes {
      if !control_flow_regexes.iter().any(|regex| regex.is_match(event.original_event().borrow().name()).unwrap_or(false)) {
        continue;
      }
    }

    let create_events_group = || {
      Some(TraceEventsGroup {
        start_point: LogPoint::new(trace_index, event_index),
        end_point: LogPoint::new(trace_index, event_index),
      })
    };

    if last_stamp.is_some() {
      if *event.stamp() - last_stamp.unwrap() > event_group_delta {
        add_to_groups(last_trace_group.clone(), last_seen_point.clone());
        last_trace_group = create_events_group();
      }
    } else {
      last_trace_group = create_events_group();
    }

    last_seen_point = Some((trace_index, event_index));
    last_stamp = Some(event.stamp().clone());
  }

  add_to_groups(last_trace_group.clone(), last_seen_point.clone());

  groups
}

struct ThreadsSequentialEvents<'a> {
  threads: &'a Vec<&'a TraceThread>,
  indices: Vec<usize>,
}

impl<'a> ThreadsSequentialEvents<'a> {
  pub fn new(threads: &'a Vec<&'a TraceThread>) -> Self {
    Self {
      threads,
      indices: vec![0; threads.len()],
    }
  }

  pub fn next(&mut self) -> Option<(&TraceThreadEvent, usize, usize)> {
    let mut min_index = 0;

    while min_index < self.indices.len() && self.indices[min_index] >= self.threads[min_index].events().len() {
      min_index += 1;
    }

    if min_index >= self.indices.len() {
      return None;
    }

    for i in (min_index + 1)..self.indices.len() {
      if self.indices[i] >= self.threads[i].events().len() {
        continue;
      }

      let stamp = self.get_stamp(i);
      if stamp < self.get_stamp(min_index) {
        min_index = i;
      }
    }

    if self.indices[min_index] >= self.threads[min_index].events().len() {
      None
    } else {
      self.indices[min_index] += 1;
      Some((
        self.threads.get(min_index).unwrap().events().get(self.indices[min_index] - 1).as_ref().unwrap(),
        min_index,
        self.indices[min_index] - 1
      ))
    }
  }

  fn get_stamp(&self, index: usize) -> u64 {
    *self.get_trace_event(index).stamp()
  }

  fn get_trace_event(&self, index: usize) -> &TraceThreadEvent {
    self.threads.get(index).unwrap().events().get(self.indices[index]).as_ref().unwrap()
  }
}

#[derive(Clone, Debug, Getters, MutGetters, Setters)]
pub struct EventGroup {
  #[getset(get = "pub", get_mut = "pub")] control_flow_events: Vec<Rc<RefCell<XesEventImpl>>>,
  #[getset(get = "pub", get_mut = "pub")] statistic_events: Vec<Rc<RefCell<XesEventImpl>>>,
  #[getset(get = "pub", get_mut = "pub", set = "pub")] after_group_events: Option<Vec<Rc<RefCell<XesEventImpl>>>>,
}

impl EventGroup {
  pub fn empty() -> Self {
    Self {
      control_flow_events: vec![],
      statistic_events: vec![],
      after_group_events: None,
    }
  }

  pub fn all_events(&self) -> Vec<&Rc<RefCell<XesEventImpl>>> {
    self.control_flow_events.iter().chain(&self.statistic_events).collect()
  }
}

pub fn enumerate_event_groups(log: &LogTimelineDiagram) -> Vec<Vec<EventGroup>> {
  let mut result = vec![];

  for trace_diagram in log.traces() {
    let mut group_index = 0;
    let threads_refs: Vec<&TraceThread> = trace_diagram.threads().iter().map(|x| x).collect();
    let get_stamp = |point: &LogPoint| {
      threads_refs.get(*point.trace_index()).unwrap().events().get(*point.event_index()).unwrap().stamp()
    };

    let mut events = ThreadsSequentialEvents::new(&threads_refs);

    let mut events_groups = trace_diagram.events_groups().clone();
    events_groups.sort_by(|f, s| get_stamp(f.start_point()).cmp(&get_stamp(s.start_point())));

    let mut trace_groups: Vec<EventGroup> = vec![];
    let mut current_group = None;

    let try_put_after_event_to_last_group = |event: Rc<RefCell<XesEventImpl>>, trace_groups: &mut Vec<EventGroup>| {
      if trace_groups.is_empty() {
        return;
      }

      if let Some(after_events) = trace_groups.last_mut().unwrap().after_group_events_mut() {
        after_events.push(event);
      } else {
        trace_groups.last_mut().unwrap().set_after_group_events(Some(vec![event]));
      }
    };

    while let Some((event, trace_index, event_index)) = events.next() {
      if group_index >= events_groups.len() {
        try_put_after_event_to_last_group(event.original_event().clone(), &mut trace_groups);
        continue;
      }

      let current_group_info = events_groups.get(group_index).unwrap();
      let start_point = current_group_info.start_point();

      if trace_index == *start_point.trace_index() && event_index == *start_point.event_index() {
        current_group = Some(EventGroup::empty());
      } else if current_group.is_none() {
        try_put_after_event_to_last_group(event.original_event().clone(), &mut trace_groups);
      }

      if let Some(current_group) = current_group.as_mut() {
        if log.is_control_flow_event(event.original_event().borrow().name().as_str()) {
          current_group.control_flow_events_mut().push(event.original_event().clone());
        } else {
          current_group.statistic_events_mut().push(event.original_event().clone());
        }
      }

      if trace_index == *current_group_info.end_point.trace_index() && event_index == *current_group_info.end_point.event_index() {
        if let Some(current_group) = current_group {
          trace_groups.push(current_group);
        }

        current_group = None;
        group_index += 1;
      }
    }

    result.push(trace_groups);
  }

  result
}