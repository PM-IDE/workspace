use crate::features::discovery::timeline::discovery::{LogPoint, TraceThread, TraceThreadEvent};

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

pub fn discover_events_groups(threads: &Vec<&TraceThread>, event_group_delta: u64) -> Vec<TraceEventsGroup> {
  let mut groups = vec![];

  let mut last_stamp: Option<u64> = None;
  let mut last_trace_group: Option<TraceEventsGroup> = None;

  let mut events = ThreadsSequentialEvents::new(threads);
  let mut last_seen_point: Option<(usize, usize)> = None;

  let mut add_to_groups = |last_trace_group: Option<TraceEventsGroup>, last_seen_point: Option<(usize, usize)>| {
    let mut adjusted_last_group = last_trace_group.unwrap().clone();
    adjusted_last_group.end_point = LogPoint {
      trace_index: last_seen_point.unwrap().0,
      event_index: last_seen_point.unwrap().1,
    };

    groups.push(adjusted_last_group);
  };

  while let Some((event, trace_index, event_index)) = events.next() {
    let create_events_group = || {
      Some(TraceEventsGroup {
        start_point: LogPoint {
          event_index,
          trace_index,
        },
        end_point: LogPoint {
          event_index,
          trace_index,
        },
      })
    };

    if last_stamp.is_some() {
      if event.stamp - last_stamp.unwrap() > event_group_delta {
        add_to_groups(last_trace_group.clone(), last_seen_point.clone());
        last_trace_group = create_events_group();
      }
    } else {
      last_trace_group = create_events_group();
    }

    last_seen_point = Some((trace_index, event_index));
    last_stamp = Some(event.stamp.clone());
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

    while min_index < self.indices.len() && self.indices[min_index] >= self.threads[min_index].events.len() {
      min_index += 1;
    }

    if min_index >= self.indices.len() {
      return None;
    }

    for i in (min_index + 1)..self.indices.len() {
      if self.indices[i] >= self.threads[i].events.len() {
        continue;
      }

      let stamp = self.get_stamp(i);
      if stamp < self.get_stamp(min_index) {
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
