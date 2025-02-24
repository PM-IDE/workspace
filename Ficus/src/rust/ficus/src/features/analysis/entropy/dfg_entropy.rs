use std::collections::{HashMap, HashSet};

use crate::features::analysis::log_info::event_log_info::EventLogInfo;
use crate::features::analysis::log_info::event_log_info::OfflineEventLogInfo;
use crate::features::analysis::log_info::log_info_creation_dto::EventLogInfoCreationDto;
use crate::{
  event_log::core::event_log::EventLog,
  features::analysis::constants::{FAKE_EVENT_END_NAME, FAKE_EVENT_START_NAME},
};

pub fn calculate_laplace_dfg_entropy<TLog>(log: &TLog, ignored_events: Option<&HashSet<String>>) -> HashMap<String, f64>
where
  TLog: EventLog,
{
  let dfr_or_dpr_calculator = |pair_count, events_count, event_count| {
    let alpha = 1 as f64 / event_count as f64;
    let x = alpha + pair_count as f64;
    let y = alpha * ((events_count + 1) as f64) + event_count as f64;
    x / y
  };

  let dfr_calculator = |first: &String, second: &String, log_info: &dyn EventLogInfo| {
    let pair_count = log_info.dfg_info().get_directly_follows_count(first, second);
    let first_count = log_info.event_count(first);
    dfr_or_dpr_calculator(pair_count, log_info.event_classes_count(), first_count)
  };

  let dpr_calculator = |first: &String, second: &String, log_info: &dyn EventLogInfo| {
    let pair_count = log_info.dfg_info().get_directly_follows_count(second, first);
    let first_count = log_info.event_count(first);
    dfr_or_dpr_calculator(pair_count, log_info.event_classes_count(), first_count)
  };

  let creation_dto = EventLogInfoCreationDto::default_fake_ignored(log, ignored_events);
  let log_info = OfflineEventLogInfo::create_from(creation_dto);
  calculate_dfg_entropy(&log_info, dfr_calculator, dpr_calculator)
}

pub fn calculate_default_dfg_entropy<TLog>(log: &TLog, ignored_events: Option<&HashSet<String>>) -> HashMap<String, f64>
where
  TLog: EventLog,
{
  let dfr_calculator = |first: &String, second: &String, log_info: &dyn EventLogInfo| {
    let dfg = log_info.dfg_info();
    let dfr = dfg.get_directly_follows_count(first, second);
    let first_count = log_info.event_count(first);
    dfr as f64 / first_count as f64
  };

  let dpr_calculator = |first: &String, second: &String, log_info: &dyn EventLogInfo| {
    let dfg = log_info.dfg_info();
    let dfr = dfg.get_directly_follows_count(second, first);
    let first_count = log_info.event_count(first);
    dfr as f64 / first_count as f64
  };

  let creation_dto = EventLogInfoCreationDto::default_fake_ignored(log, ignored_events);
  let log_info = OfflineEventLogInfo::create_from(creation_dto);
  calculate_dfg_entropy(&log_info, dfr_calculator, dpr_calculator)
}

fn calculate_dfg_entropy<TDfrEntropyCalculator, TDprEntropyCalculator>(
  log_info: &dyn EventLogInfo,
  dfr_calculator: TDfrEntropyCalculator,
  dpr_calculator: TDprEntropyCalculator,
) -> HashMap<String, f64>
where
  TDfrEntropyCalculator: Fn(&String, &String, &dyn EventLogInfo) -> f64,
  TDprEntropyCalculator: Fn(&String, &String, &dyn EventLogInfo) -> f64,
{
  let mut entropy = HashMap::new();
  let events_names = &log_info.all_event_classes();

  let mut dfr_events_names = events_names.clone();
  let fake_end = FAKE_EVENT_END_NAME.to_string();
  dfr_events_names.push(&fake_end);

  let mut dpr_events_names = events_names.clone();
  let fake_start = FAKE_EVENT_START_NAME.to_string();
  dpr_events_names.push(&fake_start);

  for event_name in events_names {
    let dfr_vector: Vec<f64> = dfr_events_names
      .iter()
      .map(|current_name| dfr_calculator(event_name, current_name, log_info))
      .collect();

    let dpr_vector: Vec<f64> = dpr_events_names
      .iter()
      .map(|current_name| dpr_calculator(event_name, current_name, log_info))
      .collect();

    let event_entropy = calculate_entropy(&dfr_vector) + calculate_entropy(&dpr_vector);
    entropy.insert(event_name.to_string(), event_entropy);
  }

  entropy
}

fn calculate_entropy(values: &Vec<f64>) -> f64 {
  let mut entropy = 0f64;
  for value in values {
    if *value != 0f64 {
      entropy -= value * value.log2();
    }
  }

  entropy
}
