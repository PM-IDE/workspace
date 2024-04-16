use std::collections::{HashMap, HashSet};

use crate::event_log::core::{
    event_log::EventLog,
    trace::trace::{Trace, TraceEventsPositions},
};

use super::shared::{calculate_entropies, calculate_max_vector_length, calculate_pos_entropy};

pub fn calculate_pos_entropies_fast<TLog>(log: &TLog, ignored_events: Option<&HashSet<String>>) -> HashMap<String, f64>
where
    TLog: EventLog,
{
    calculate_entropies(log, ignored_events, calculate_pos_entropy_for_event_fast)
}

pub fn calculate_pos_entropy_for_event_fast<TLog>(log: &TLog, name: &String, ignored_events: Option<&HashSet<String>>) -> f64
where
    TLog: EventLog,
{
    let vector_length = calculate_max_vector_length(log, ignored_events);
    let mut probabilities = vec![0f64; vector_length];

    let mut non_empty_traces_count = 0;
    let mut ordered_positions_of_ignored_events: Vec<usize> = Vec::new();

    for trace in log.traces() {
        let trace_length = trace.borrow().events().len();
        let mut trace = trace.borrow_mut();

        let positions = trace.get_or_create_events_positions();
        ordered_positions_of_ignored_events.clear();

        if let Some(ignored_events_set) = ignored_events {
            for ignored_event in ignored_events_set {
                let positions = positions.event_positions(ignored_event);
                if let Some(positions) = positions {
                    ordered_positions_of_ignored_events.extend(positions);
                }
            }
        }

        if ordered_positions_of_ignored_events.len() == trace_length {
            continue;
        }

        non_empty_traces_count += 1;

        let positions_of_our_event = positions.event_positions(name);
        if positions_of_our_event.is_none() {
            continue;
        }

        let positions_of_our_event = positions_of_our_event.unwrap();
        ordered_positions_of_ignored_events.sort();

        let mut our_idx = 0;
        let mut ignored_idx = 0;

        while our_idx != positions_of_our_event.len() || ignored_idx != ordered_positions_of_ignored_events.len() {
            if our_idx >= positions_of_our_event.len() {
                break;
            }

            while ignored_idx < ordered_positions_of_ignored_events.len()
                && positions_of_our_event[our_idx] > ordered_positions_of_ignored_events[ignored_idx]
            {
                ignored_idx += 1
            }

            probabilities[positions_of_our_event[our_idx] - ignored_idx] += 1f64;
            our_idx += 1;
        }
    }

    calculate_pos_entropy(&mut probabilities, non_empty_traces_count as f64)
}
