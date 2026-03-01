use crate::{
  event_log::core::{event::event::Event, event_log::EventLog, trace::trace::Trace},
  features::{
    analysis::{
      constants::{FAKE_EVENT_END_NAME, FAKE_EVENT_START_NAME},
      log_info::{
        dfg_info::{DfgInfo, OfflineDfgInfo},
        log_info_creation_dto::EventLogInfoCreationDto,
      },
    },
    discovery::timeline::utils::extract_thread_id,
  },
  utils::hash_map_utils::increase_in_map,
};
use std::{
  cell::RefCell,
  collections::{HashMap, HashSet},
  ops::Deref,
  rc::Rc,
};

pub trait EventLogCounts {
  fn traces_count(&self) -> usize;
  fn events_count(&self) -> usize;
}

struct EventLogCountsImpl {
  traces_count: usize,
  events_count: usize,
}

impl EventLogCounts for EventLogCountsImpl {
  fn traces_count(&self) -> usize {
    self.traces_count
  }

  fn events_count(&self) -> usize {
    self.events_count
  }
}

pub trait EventLogInfo {
  fn counts(&self) -> Option<&dyn EventLogCounts>;
  fn event_classes_count(&self) -> u64;
  fn event_count(&self, event_class: &str) -> u64;
  fn dfg_info(&self) -> &dyn DfgInfo;
  fn all_event_classes(&self) -> Vec<&Rc<str>>;
  fn start_event_classes(&self) -> &HashSet<Rc<str>>;
  fn end_event_classes(&self) -> &HashSet<Rc<str>>;
}

pub struct OfflineEventLogInfo {
  counts: Option<EventLogCountsImpl>,
  event_classes_counts: HashMap<Rc<str>, u64>,
  dfg_info: OfflineDfgInfo,
  start_event_classes: HashSet<Rc<str>>,
  end_event_classes: HashSet<Rc<str>>,
}

impl OfflineEventLogInfo {
  pub fn create_from_relations(relations: &HashMap<(Rc<str>, Rc<str>), u64>, event_classes_count: &HashMap<Rc<str>, u64>) -> Self {
    let dfg_info = OfflineDfgInfo::create_from_relations(relations);

    let start_event_classes = event_classes_count
      .keys()
      .filter(|c| match dfg_info.precedes_events.get(*c) {
        None => true,
        Some(precedes) => precedes.is_empty(),
      })
      .map(|c| c.to_owned())
      .collect();

    let end_event_classes = event_classes_count
      .keys()
      .filter(|c| match dfg_info.followed_events.get(*c) {
        None => true,
        Some(followers) => followers.is_empty(),
      })
      .map(|c| c.to_owned())
      .collect();

    Self {
      counts: None,
      event_classes_counts: event_classes_count.clone(),
      dfg_info,
      start_event_classes,
      end_event_classes,
    }
  }

  pub fn create_from<TLog: EventLog>(creation_dto: EventLogInfoCreationDto<TLog>) -> OfflineEventLogInfo {
    let EventLogInfoCreationDto {
      mut log,
      add_fake_start_end_events,
      ignored_events,
      thread_attribute,
    } = creation_dto;

    let mut new_log = None;
    if let Some(thread_attribute) = thread_attribute {
      new_log = Some(create_threads_log_by_attribute::<TLog>(log, thread_attribute.as_ref()));
      log = new_log.as_ref().unwrap();
    }

    let mut dfg_pairs: HashMap<Rc<str>, HashMap<Rc<str>, usize>> = HashMap::new();
    let mut events_count = 0;
    let mut events_counts = HashMap::new();
    let mut start_event_classes = HashSet::new();
    let mut end_event_classes = HashSet::new();

    let mut update_events_counts = |event_name: &Rc<str>| {
      increase_in_map(&mut events_counts, event_name);
    };

    let mut update_pairs_count = |first_name: &Rc<str>, second_name: &Rc<str>| {
      if let Some(values) = dfg_pairs.get_mut(first_name) {
        if let Some(count) = values.get_mut(second_name) {
          *count += 1;
        } else {
          values.insert(second_name.clone(), 1);
        }
      } else {
        let map = HashMap::from_iter(vec![(second_name.clone(), 1)]);
        dfg_pairs.insert(first_name.clone(), map);
      }
    };

    let fake_start_name = Rc::<str>::from(FAKE_EVENT_START_NAME);
    let fake_end_name = Rc::<str>::from(FAKE_EVENT_END_NAME);

    for trace in log.traces() {
      let trace = trace.borrow();
      let events = trace.events();
      events_count += events.len();
      let mut prev_event_name = None;

      if !events.is_empty() {
        start_event_classes.insert(events.first().unwrap().borrow().name_pointer().clone());
        end_event_classes.insert(events.last().unwrap().borrow().name_pointer().clone());
      }

      for event in events {
        let event = event.borrow();
        let current_name = event.name_pointer();

        if let Some(ignored_events) = ignored_events
          && ignored_events.contains(current_name)
        {
          continue;
        }

        update_events_counts(current_name);

        if prev_event_name.is_none() {
          prev_event_name = Some(current_name.to_owned());
          if add_fake_start_end_events {
            update_pairs_count(&fake_start_name, current_name);
          }

          continue;
        }

        let prev_name = prev_event_name.unwrap();
        update_pairs_count(&prev_name, current_name);
        prev_event_name = Some(event.name_pointer().clone());
      }

      if add_fake_start_end_events && let Some(prev_event_name) = prev_event_name {
        update_pairs_count(&prev_event_name, &fake_end_name);
      }
    }

    let mut precedes_events: HashMap<Rc<str>, HashMap<Rc<str>, usize>> = HashMap::new();
    let mut events_with_single_follower = HashSet::new();

    for (first, followers_map) in &dfg_pairs {
      if followers_map.len() == 1 {
        events_with_single_follower.insert(first.to_owned());
      }

      for (second, count) in followers_map {
        if precedes_events.contains_key(second) {
          precedes_events.get_mut(second).unwrap().insert(first.to_owned(), count.to_owned());
        } else {
          let map = HashMap::from_iter(vec![(first.to_owned(), count.to_owned())]);
          precedes_events.insert(second.to_owned(), map);
        }
      }
    }

    OfflineEventLogInfo {
      counts: Some(EventLogCountsImpl {
        events_count,
        traces_count: log.traces().len(),
      }),
      event_classes_counts: events_counts,
      dfg_info: OfflineDfgInfo {
        followed_events: dfg_pairs,
        precedes_events,
        events_with_single_follower,
      },
      start_event_classes,
      end_event_classes,
    }
  }
}

pub fn create_threads_log_by_attribute<TLog: EventLog>(log: &TLog, thread_attribute: &str) -> TLog {
  let mut thread_log = TLog::empty();

  for trace in log.traces() {
    let trace = trace.borrow();
    let mut threads_traces = HashMap::<Option<Rc<str>>, TLog::TTrace>::new();

    for event in trace.events() {
      let thread_id = extract_thread_id(event.borrow().deref(), thread_attribute);
      if let Some(thread_trace) = threads_traces.get_mut(&thread_id) {
        thread_trace.push(event.clone());
      } else {
        let mut new_trace = TLog::TTrace::empty();
        new_trace.push(event.clone());

        threads_traces.insert(thread_id, new_trace);
      }
    }

    for thread_trace in threads_traces.into_iter() {
      thread_log.push(Rc::new(RefCell::new(thread_trace.1)));
    }
  }

  thread_log
}

impl EventLogInfo for OfflineEventLogInfo {
  fn counts(&self) -> Option<&dyn EventLogCounts> {
    match self.counts.as_ref() {
      None => None,
      Some(counts) => Some(counts as &dyn EventLogCounts),
    }
  }

  fn event_classes_count(&self) -> u64 {
    self.event_classes_counts.len() as u64
  }

  fn event_count(&self, event_class: &str) -> u64 {
    match self.event_classes_counts.get(event_class) {
      Some(value) => value.to_owned(),
      None => 0,
    }
  }

  fn dfg_info(&self) -> &dyn DfgInfo {
    &self.dfg_info
  }

  fn all_event_classes(&self) -> Vec<&Rc<str>> {
    self.event_classes_counts.keys().collect()
  }

  fn start_event_classes(&self) -> &HashSet<Rc<str>> {
    &self.start_event_classes
  }

  fn end_event_classes(&self) -> &HashSet<Rc<str>> {
    &self.end_event_classes
  }
}

pub fn count_events(log: &impl EventLog) -> usize {
  log.traces().iter().map(|trace| trace.borrow().events().len()).sum()
}
