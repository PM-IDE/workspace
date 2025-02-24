use std::collections::{HashMap, HashSet};

use crate::event_log::core::{
  event_log::EventLog,
  trace::trace::{Trace, TraceInfo},
};
use crate::features::analysis::log_info::event_log_info::EventLogInfo;
use crate::features::analysis::log_info::event_log_info::OfflineEventLogInfo;
use crate::features::analysis::log_info::log_info_creation_dto::EventLogInfoCreationDto;

pub fn calculate_max_vector_length<TLog>(log: &TLog, ignored_events: Option<&HashSet<String>>) -> usize
where
  TLog: EventLog,
{
  match ignored_events {
    Some(ignored_events) => calculate_vector_length_with_ignored_events(log, ignored_events),
    None => calculate_vector_length(log),
  }
}

fn calculate_vector_length_with_ignored_events<TLog>(log: &TLog, ignored_events: &HashSet<String>) -> usize
where
  TLog: EventLog,
{
  let mut max = 0;

  for trace in log.traces() {
    let mut trace = trace.borrow_mut();
    let counts = trace.get_or_create_trace_info().events_counts();
    let mut num_of_ignored_events = 0;
    for ignored_event in ignored_events {
      if let Some(count) = counts.get(ignored_event) {
        num_of_ignored_events += *count;
      }
    }

    max = max.max(trace.events().len() - num_of_ignored_events);
  }

  max
}

fn calculate_vector_length<TLog>(log: &TLog) -> usize
where
  TLog: EventLog,
{
  log.traces().into_iter().map(|trace| trace.borrow().events().len()).max().unwrap()
}

pub fn calculate_pos_entropy(probabilities: &mut Vec<f64>, traces_count: f64) -> f64 {
  for i in 0..probabilities.len() {
    probabilities[i] = probabilities[i] / traces_count;
  }

  let log = traces_count.log2();
  let mut non_zero_count = 0;

  let sum: f64 = probabilities
    .iter()
    .filter(|p| {
      if **p != 0f64 {
        non_zero_count += 1;
        return true;
      }

      false
    })
    .map(|p| -p.log2() / log)
    .sum();

  sum / non_zero_count as f64
}

pub fn calculate_entropies<TLog, TEntropyCalculator>(
  log: &TLog,
  ignored_events: Option<&HashSet<String>>,
  entropy_calculator: TEntropyCalculator,
) -> HashMap<String, f64>
where
  TLog: EventLog,
  TEntropyCalculator: Fn(&TLog, &String, Option<&HashSet<String>>) -> f64,
{
  let log_info = OfflineEventLogInfo::create_from(EventLogInfoCreationDto::default(log));
  let mut entropies = HashMap::new();
  for event_name in log_info.all_event_classes() {
    if let Some(ignored_events) = ignored_events {
      if ignored_events.contains(event_name.as_str()) {
        continue;
      }
    }

    let entropy = entropy_calculator(log, &event_name, ignored_events);
    entropies.insert(event_name.to_owned(), entropy);
  }

  entropies
}
