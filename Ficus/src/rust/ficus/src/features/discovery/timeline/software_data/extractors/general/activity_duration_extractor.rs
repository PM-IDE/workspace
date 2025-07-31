use crate::event_log::core::event::event::Event;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::events_groups::EventGroup;
use crate::features::discovery::timeline::software_data::extraction_config::{ActivityDurationExtractionConfig, SoftwareDataExtractionConfig};
use crate::features::discovery::timeline::software_data::extractors::core::{EventGroupTraceSoftwareDataExtractor, SoftwareDataExtractionError};
use crate::features::discovery::timeline::software_data::extractors::general::utils::RegexParingResult;
use crate::features::discovery::timeline::software_data::models::{ActivityDurationData, SoftwareData};
use crate::features::discovery::timeline::utils::get_stamp;
use derive_new::new;
use fancy_regex::Regex;
use getset::Getters;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use log::error;

#[derive(Clone, Debug, new)]
pub struct ActivityDurationExtractor<'a> {
  config: &'a SoftwareDataExtractionConfig,
}

#[derive(Getters, new)]
struct StackActivityStartEntry {
  #[getset(get = "pub")] id: Option<String>,
  #[getset(get = "pub")] event: Rc<RefCell<XesEventImpl>>,
}

impl<'a> EventGroupTraceSoftwareDataExtractor for ActivityDurationExtractor<'a> {
  fn extract(&self, trace: &Vec<EventGroup>, software_data: &mut Vec<(SoftwareData, SoftwareData)>) -> Result<(), SoftwareDataExtractionError> {
    if self.config.activities_duration_configs().len() == 0 {
      return Ok(());
    }

    let mut configs = self.config
      .activities_duration_configs()
      .iter()
      .map(|info|
        (
          Regex::new(info.start_event_regex()).map_err(|_| SoftwareDataExtractionError::FailedToParseRegex(info.start_event_regex().to_string())),
          Regex::new(info.end_event_regex()).map_err(|_| SoftwareDataExtractionError::FailedToParseRegex(info.end_event_regex().to_string())),
          info,
          Vec::new(),
          Vec::new()
        )
      )
      .collect::<Vec<(RegexParingResult, RegexParingResult, &ActivityDurationExtractionConfig, Vec<StackActivityStartEntry>, Vec<Option<DurationMapInfo>>)>>();


    for group in trace.iter() {
      let events = group.all_events().iter().map(|e| (*e).clone()).collect::<Vec<Rc<RefCell<XesEventImpl>>>>();

      for (start_regex, end_regex, info, global_state, data) in configs.iter_mut() {
        let node_durations = process_events(events.as_slice(), start_regex, end_regex, info, global_state, data)?;

        data.push(Some(DurationMapInfo {
          map: node_durations,
          start_time: get_stamp_or_err(&events.first().unwrap().borrow(), info.time_attribute().as_ref())?,
          end_time: get_stamp_or_err(&events.last().unwrap().borrow(), info.time_attribute().as_ref())?,
        }));

        let edge_events = group.after_group_events();
        let edges_durations = if let Some(edge_events) = edge_events.as_ref() {
          Some(process_events(edge_events.as_slice(), start_regex, end_regex, info, global_state, data)?)
        } else {
          None
        };

        data.push(if let Some(map) = edges_durations {
          Some(DurationMapInfo {
            map,
            start_time: get_stamp_or_err(&edge_events.as_ref().unwrap().first().unwrap().borrow(), info.time_attribute().as_ref())?,
            end_time: get_stamp_or_err(&edge_events.as_ref().unwrap().first().unwrap().borrow(), info.time_attribute().as_ref())?,
          })
        } else {
          None
        });
      }
    }

    for (_, _, _, _, data) in configs.iter_mut() {
      if data.len() != software_data.len() * 2 {
        error!("data.len() != result.len() * 2");
        continue;
      }

      let mut index = 0;
      for (node_data, edge_data) in software_data.iter_mut() {
        for (name, (value, units)) in data[index].as_ref().unwrap().map.iter() {
          node_data.activities_durations_mut().push(ActivityDurationData::new(name.to_string(), *value as f64, units.to_string()));
        }

        for (name, (value, units)) in data[index + 1].as_ref().unwrap().map.iter() {
          edge_data.activities_durations_mut().push(ActivityDurationData::new(name.to_string(), *value as f64, units.to_string()));
        }

        index += 2;
      }
    }

    Ok(())
  }
}

type DurationsMap = HashMap<String, (u64, String)>;

struct DurationMapInfo {
  start_time: u64,
  end_time: u64,
  map: DurationsMap,
}

fn process_events(
  events: &[Rc<RefCell<XesEventImpl>>],
  start_regex: &RegexParingResult,
  end_regex: &RegexParingResult,
  info: &ActivityDurationExtractionConfig,
  global_state: &mut Vec<StackActivityStartEntry>,
  previous_data: &mut Vec<Option<DurationMapInfo>>,
) -> Result<DurationsMap, SoftwareDataExtractionError> {
  let mut durations: DurationsMap = HashMap::new();

  let mut add_duration = |start_event: &Rc<RefCell<XesEventImpl>>,
                          end_event: &Rc<RefCell<XesEventImpl>>,
                          info: &ActivityDurationExtractionConfig| -> Result<(), SoftwareDataExtractionError> {
    let duration = get_duration(&start_event.borrow(), &end_event.borrow(), info.time_attribute().as_ref())?;
    (*durations.entry(info.name().to_string()).or_insert((0u64, info.units().to_string()))).0 += duration;

    Ok(())
  };

  let mut local_state = vec![];

  for event in events {
    let start_regex = match start_regex {
      Ok(regex) => regex,
      Err(err) => return Err(err.clone())
    };

    let end_regex = match end_regex {
      Ok(regex) => regex,
      Err(err) => return Err(err.clone())
    };

    let id = if let Some(strategy) = info.activity_id_attr() {
      Some(strategy.create(&event.borrow()))
    } else {
      None
    };

    if start_regex.is_match(event.borrow().name()).unwrap_or(false) {
      local_state.push(StackActivityStartEntry::new(id, (*event).clone()));
      continue;
    }

    if end_regex.is_match(event.borrow().name()).unwrap_or(false) {
      if let Some(pos) = local_state.iter().rposition(|e| e.id().eq(&id)) {
        let start_event = local_state[pos].event();
        add_duration(start_event, event, info)?;

        local_state.remove(pos);
      } else {
        if let Some(pos) = global_state.iter().rposition(|e| e.id().eq(&id)) {
          global_state.remove(pos);
        } else {
          for prev_data in previous_data.iter_mut() {
            if let Some(prev_data) = prev_data.as_mut() {
              let duration = prev_data.end_time - prev_data.start_time;
              (*prev_data.map.entry(info.name().to_string()).or_insert((0u64, info.units().to_string()))).0 += duration;
            }
          }
        }

        add_duration(events.first().unwrap(), event, info)?;
      }
    }
  }

  for _ in global_state.iter() {
    add_duration(events.first().unwrap(), events.last().unwrap(), info)?;
  }

  global_state.extend(local_state);

  Ok(durations)
}

fn get_duration(first: &XesEventImpl, second: &XesEventImpl, attribute: Option<&String>) -> Result<u64, SoftwareDataExtractionError> {
  Ok(get_stamp_or_err(second, attribute)? - get_stamp_or_err(first, attribute)?)
}

fn get_stamp_or_err(event: &XesEventImpl, attribute: Option<&String>) -> Result<u64, SoftwareDataExtractionError> {
  get_stamp(event, attribute).map_err(|_| SoftwareDataExtractionError::FailedToGetStamp)
}