use std::collections::{HashMap, HashSet};

use crate::event_log::core::{event::event::Event, event_log::EventLog, trace::trace::Trace};

use super::shared::{calculate_entropies, calculate_max_vector_length, calculate_pos_entropy};

pub fn calculate_pos_entropies<TLog>(log: &TLog, ignored_events: &Option<HashSet<String>>) -> HashMap<String, f64>
where
  TLog: EventLog,
{
  calculate_entropies(log, ignored_events.as_ref(), calculate_pos_entropy_for_event)
}

pub fn calculate_pos_entropy_for_event<TLog>(log: &TLog, event_name: &String, ignored_events: Option<&HashSet<String>>) -> f64
where
  TLog: EventLog,
{
  let vector_length = calculate_max_vector_length(log, ignored_events);
  let mut prob_vector = vec![0f64; vector_length];
  let mut non_empty_traces_count = 0;

  for trace in log.traces() {
    let mut index = 0;
    let mut empty_trace = true;
    for event in trace.borrow().events() {
      let event = event.borrow();
      let name = event.name();

      if let Some(ignored_events_set) = ignored_events {
        if ignored_events_set.contains(name) {
          continue;
        }
      }

      empty_trace = false;
      if name == event_name {
        prob_vector[index] += 1f64;
      }

      index += 1;
    }

    if !empty_trace {
      non_empty_traces_count += 1;
    }
  }

  calculate_pos_entropy(&mut prob_vector, non_empty_traces_count as f64)
}
