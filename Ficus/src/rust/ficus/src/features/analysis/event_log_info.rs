use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::{event_log::core::event::event::Event, utils::hash_map_utils::increase_in_map};
use std::collections::{HashMap, HashSet};

use super::constants::{FAKE_EVENT_END_NAME, FAKE_EVENT_START_NAME};

pub struct EventLogInfo {
    events_count: usize,
    event_classes_counts: HashMap<String, usize>,
    dfg_info: DfgInfo,
    traces_count: usize,
    start_event_classes: HashSet<String>,
    end_event_classes: HashSet<String>,
}

pub struct EventLogInfoCreationDto<'a, TLog>
where
    TLog: EventLog,
{
    log: &'a TLog,
    add_fake_start_end_events: bool,
    ignored_events: Option<&'a HashSet<String>>,
}

impl<'a, TLog> EventLogInfoCreationDto<'a, TLog>
where
    TLog: EventLog,
{
    pub fn default(log: &'a TLog) -> Self {
        EventLogInfoCreationDto {
            log,
            add_fake_start_end_events: false,
            ignored_events: None,
        }
    }

    pub fn default_fake_events(log: &'a TLog) -> Self {
        Self {
            log,
            add_fake_start_end_events: true,
            ignored_events: None,
        }
    }

    pub fn default_fake_ignored(log: &'a TLog, ignored_events: Option<&'a HashSet<String>>) -> Self {
        Self {
            log,
            add_fake_start_end_events: true,
            ignored_events,
        }
    }

    pub fn default_ignore(log: &'a TLog, ignored_events: &'a HashSet<String>) -> Self {
        Self {
            log,
            add_fake_start_end_events: false,
            ignored_events: Some(ignored_events),
        }
    }
}

impl EventLogInfo {
    pub fn create_from<TLog>(creation_dto: EventLogInfoCreationDto<TLog>) -> EventLogInfo
    where
        TLog: EventLog,
    {
        let EventLogInfoCreationDto {
            log,
            add_fake_start_end_events,
            ignored_events,
        } = creation_dto;

        let mut dfg_pairs: HashMap<String, HashMap<String, usize>> = HashMap::new();
        let mut events_count = 0;
        let mut events_counts = HashMap::new();
        let mut start_event_classes = HashSet::new();
        let mut end_event_classes = HashSet::new();

        let mut update_events_counts = |event_name: &String| {
            increase_in_map(&mut events_counts, event_name);
        };

        let mut update_pairs_count = |first_name: &String, second_name: &String| {
            if let Some(values) = dfg_pairs.get_mut(first_name) {
                if let Some(count) = values.get_mut(second_name) {
                    *count += 1;
                } else {
                    values.insert(second_name.to_string(), 1);
                }
            } else {
                let map = HashMap::from_iter(vec![(second_name.to_owned(), 1)]);
                dfg_pairs.insert(first_name.to_owned(), map);
            }
        };

        for trace in log.traces() {
            let trace = trace.borrow();
            let events = trace.events();
            events_count += events.len();
            let mut prev_event_name = None;

            if events.len() > 0 {
                start_event_classes.insert(events.first().unwrap().borrow().name().to_owned());
                end_event_classes.insert(events.last().unwrap().borrow().name().to_owned());
            }

            for event in events {
                let event = event.borrow();
                let current_name = event.name().to_owned();

                if let Some(ignored_events) = ignored_events {
                    if ignored_events.contains(&current_name) {
                        continue;
                    }
                }

                update_events_counts(&current_name);

                if prev_event_name.is_none() {
                    prev_event_name = Some(current_name.to_owned());
                    if add_fake_start_end_events {
                        update_pairs_count(&FAKE_EVENT_START_NAME.to_string(), &current_name);
                    }

                    continue;
                }

                let prev_name = prev_event_name.unwrap();
                update_pairs_count(&prev_name, &current_name);
                prev_event_name = Some(event.name().to_owned());
            }

            if add_fake_start_end_events && prev_event_name.is_some() {
                update_pairs_count(&prev_event_name.unwrap(), &FAKE_EVENT_END_NAME.to_string());
            }
        }

        let mut followed_events: HashMap<String, HashMap<String, usize>> = HashMap::new();
        let mut precedes_events: HashMap<String, HashMap<String, usize>> = HashMap::new();
        let mut events_with_single_follower = HashSet::new();

        for (first, followers_map) in &dfg_pairs {
            for (second, count) in followers_map {
                if followed_events.contains_key(first) {
                    if events_with_single_follower.contains(first) {
                        events_with_single_follower.remove(first);
                    }

                    if !followed_events.get(first).unwrap().contains_key(second) {
                        let followers_map = followed_events.get_mut(first).unwrap();
                        followers_map.insert(second.to_owned(), count.to_owned());
                    }
                } else {
                    let map = HashMap::from_iter(vec![(second.to_owned(), count.to_owned())]);
                    followed_events.insert(first.to_owned(), map);
                    events_with_single_follower.insert(first.to_owned());
                }

                if precedes_events.contains_key(second) {
                    precedes_events.get_mut(second).unwrap().insert(first.to_owned(), count.to_owned());
                } else {
                    let map = HashMap::from_iter(vec![(first.to_owned(), count.to_owned())]);
                    precedes_events.insert(second.to_owned(), map);
                }
            }
        }

        EventLogInfo {
            events_count,
            event_classes_counts: events_counts,
            dfg_info: DfgInfo {
                dfg_pairs,
                followed_events,
                precedes_events,
                events_with_single_follower,
            },
            traces_count: log.traces().len(),
            start_event_classes,
            end_event_classes,
        }
    }

    pub fn traces_count(&self) -> usize {
        self.traces_count
    }

    pub fn events_count(&self) -> usize {
        self.events_count
    }

    pub fn event_classes_count(&self) -> usize {
        self.event_classes_counts.len()
    }

    pub fn event_count(&self, event_class: &String) -> usize {
        match self.event_classes_counts.get(event_class) {
            Some(value) => value.to_owned(),
            None => 0,
        }
    }

    pub fn dfg_info(&self) -> &DfgInfo {
        &self.dfg_info
    }

    pub fn all_event_classes(&self) -> Vec<&String> {
        self.event_classes_counts.keys().into_iter().collect()
    }

    pub fn start_event_classes(&self) -> &HashSet<String> {
        &self.start_event_classes
    }

    pub fn end_event_classes(&self) -> &HashSet<String> {
        &self.end_event_classes
    }
}

#[derive(Debug)]
pub struct DfgInfo {
    dfg_pairs: HashMap<String, HashMap<String, usize>>,
    followed_events: HashMap<String, HashMap<String, usize>>,
    precedes_events: HashMap<String, HashMap<String, usize>>,
    events_with_single_follower: HashSet<String>,
}

impl DfgInfo {
    pub fn get_directly_follows_count(&self, first: &String, second: &String) -> usize {
        if let Some(values) = self.dfg_pairs.get(first) {
            if let Some(dfg_count) = values.get(second) {
                return *dfg_count;
            }
        }

        0
    }

    pub fn is_in_directly_follows_relation(&self, left: &str, right: &str) -> bool {
        if let Some(values) = self.dfg_pairs.get(left) {
            values.contains_key(right)
        } else {
            false
        }
    }

    pub fn get_followed_events(&self, event_class: &String) -> Option<&HashMap<String, usize>> {
        match self.followed_events.get(event_class) {
            Some(followers_counts) => Some(followers_counts),
            None => None,
        }
    }

    pub fn get_precedes_events(&self, event_class: &String) -> Option<&HashMap<String, usize>> {
        match self.precedes_events.get(event_class) {
            Some(followers_counts) => Some(followers_counts),
            None => None,
        }
    }

    pub fn is_event_with_single_follower(&self, event_class: &String) -> bool {
        self.events_with_single_follower.contains(event_class)
    }
}

pub fn count_events(log: &impl EventLog) -> usize {
    log.traces().iter().map(|trace| trace.borrow().events().len()).sum()
}
