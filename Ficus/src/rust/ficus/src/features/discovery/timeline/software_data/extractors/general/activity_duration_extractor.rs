use crate::event_log::core::event::event::Event;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::events_groups::EventGroup;
use crate::features::discovery::timeline::software_data::extraction_config::{ActivityDurationExtractionConfig, GenericExtractionConfigBase, SoftwareDataExtractionConfig};
use crate::features::discovery::timeline::software_data::extractors::core::{EventGroupTraceSoftwareDataExtractor, SoftwareDataExtractionError};
use crate::features::discovery::timeline::software_data::extractors::general::utils::RegexParingResult;
use crate::features::discovery::timeline::software_data::models::{ActivityDurationData, GenericEnhancementBase, SoftwareData};
use crate::features::discovery::timeline::utils::get_stamp;
use derive_new::new;
use fancy_regex::Regex;
use getset::Getters;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use log::error;
use crate::utils::vec_utils::VectorOptionExtensions;

#[derive(Clone, Debug, new)]
pub struct ActivityDurationExtractor<'a> {
  config: &'a SoftwareDataExtractionConfig,
}

type Configs<'a> = Vec<(RegexParingResult, RegexParingResult, &'a ActivityDurationExtractionConfig, Vec<StackActivityStartEntry>, Vec<Option<DurationMapInfo>>)>;

impl<'a> EventGroupTraceSoftwareDataExtractor for ActivityDurationExtractor<'a> {
  fn extract(&self, trace: &Vec<EventGroup>, software_data: &mut Vec<(SoftwareData, SoftwareData)>) -> Result<(), SoftwareDataExtractionError> {
    if self.config.activities_duration_configs().len() == 0 {
      return Ok(());
    }

    let mut configs = create_configs(self.config);
    process_events_groups(trace, &mut configs)?;
    add_durations_to_software_data(&configs, software_data);

    Ok(())
  }
}

fn create_configs(config: &SoftwareDataExtractionConfig) -> Configs {
  config
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
    .collect::<Configs>()
}

fn process_events_groups(trace: &Vec<EventGroup>, configs: &mut Configs) -> Result<(), SoftwareDataExtractionError> {
  for (index, group) in trace.iter().enumerate() {
    let events = group.statistic_events().iter().map(|e| (*e).clone()).collect::<Vec<Rc<RefCell<XesEventImpl>>>>();

    for (start_regex, end_regex, info, global_state, data) in configs.iter_mut() {
      let time_attr = info.time_attribute().as_ref();

      let (start_time, end_time) = get_event_group_node_start_end_stamps(index, trace, time_attr)?;
      let node_durations = process_events(events.as_slice(), start_time, end_time, start_regex, end_regex, info, global_state, data)?;

      data.push(Some(DurationMapInfo {
        map: node_durations,
        start_time,
        end_time,
      }));

      let edges_durations = if let Some(edge_events) = group.after_group_events().is_non_empty() {
        let (start_time, end_time) = get_event_group_edge_start_end_stamps(index, trace, time_attr)?;
        Some(process_events(edge_events.as_slice(), start_time, end_time, start_regex, end_regex, info, global_state, data)?)
      } else {
        None
      };

      data.push(if let Some(map) = edges_durations {
        let (start_time, end_time) = get_event_group_edge_start_end_stamps(index, trace, time_attr)?;
        Some(DurationMapInfo {
          map,
          start_time,
          end_time,
        })
      } else {
        None
      });
    }
  }

  Ok(())
}

fn add_durations_to_software_data(configs: &Configs, software_data: &mut Vec<(SoftwareData, SoftwareData)>) {
  for (_, _, _, _, data) in configs {
    if data.len() != software_data.len() * 2 {
      error!("data.len() != result.len() * 2");
      continue;
    }

    let mut index = 0;
    for (node_data, edge_data) in software_data.iter_mut() {
      if let Some(data) = data[index].as_ref() {
        add_software_activities_durations(node_data, data);
      }

      if let Some(data) = data[index + 1].as_ref() {
        add_software_activities_durations(edge_data, data);
      }

      index += 2;
    }
  }
}

fn get_event_group_node_start_end_stamps(
  index: usize,
  groups: &Vec<EventGroup>,
  time_attr: Option<&String>,
) -> Result<(u64, u64), SoftwareDataExtractionError> {
  Ok((
    get_stamp_or_err(groups[index].control_flow_events().first().as_ref().unwrap(), time_attr)?,
    get_stamp_or_err(groups[index].control_flow_events().last().as_ref().unwrap(), time_attr)?,
  ))
}

fn get_event_group_edge_start_end_stamps(
  index: usize,
  groups: &Vec<EventGroup>,
  time_attr: Option<&String>,
) -> Result<(u64, u64), SoftwareDataExtractionError> {
  Ok((
    get_stamp_or_err(groups[index].control_flow_events().last().as_ref().unwrap(), time_attr)?,
    if index + 1 < groups.len() {
      get_stamp_or_err(groups[index + 1].control_flow_events().first().as_ref().unwrap(), time_attr)?
    } else {
      get_stamp_or_err(groups[index].after_group_events().as_ref().expect("Must be set").last().as_ref().unwrap(), time_attr)?
    }
  ))
}

fn add_software_activities_durations(software_data: &mut SoftwareData, data: &DurationMapInfo) {
  software_data.activities_durations_mut().extend(
    data.map
      .iter()
      .map(|(_, (value, base))| ActivityDurationData::new(
        GenericEnhancementBase::new(base.name().to_string(), base.units().to_string(), base.group().clone()),
        *value as f64,
      ))
  );
}

type DurationsMap = HashMap<String, (u64, GenericExtractionConfigBase)>;

struct DurationMapInfo {
  start_time: u64,
  end_time: u64,
  map: DurationsMap,
}

trait DurationsMapExtensions {
  fn add_raw_duration(&mut self, duration: u64, info: &ActivityDurationExtractionConfig);

  fn add_duration(
    &mut self,
    start_event: &Rc<RefCell<XesEventImpl>>,
    end_event: &Rc<RefCell<XesEventImpl>>,
    info: &ActivityDurationExtractionConfig,
  ) -> Result<(), SoftwareDataExtractionError> {
    let duration = get_duration(start_event, end_event, info.time_attribute().as_ref())?;
    self.add_raw_duration(duration, info);

    Ok(())
  }
}

impl DurationsMapExtensions for DurationsMap {
  fn add_raw_duration(&mut self, duration: u64, info: &ActivityDurationExtractionConfig) {
    if duration == 0 {
      return;
    }

    (*self.entry(info.base().name().to_string()).or_insert((0u64, info.base().clone()))).0 += duration;
  }
}

#[derive(Getters, new)]
struct StackActivityStartEntry {
  #[getset(get = "pub")] id: Option<String>,
  #[getset(get = "pub")] event: Rc<RefCell<XesEventImpl>>,
}

fn process_events(
  events: &[Rc<RefCell<XesEventImpl>>],
  start_time: u64,
  end_time: u64,
  start_regex: &RegexParingResult,
  end_regex: &RegexParingResult,
  info: &ActivityDurationExtractionConfig,
  global_state: &mut Vec<StackActivityStartEntry>,
  previous_data: &mut Vec<Option<DurationMapInfo>>,
) -> Result<DurationsMap, SoftwareDataExtractionError> {
  let mut durations: DurationsMap = HashMap::new();

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
        durations.add_duration(start_event, event, info)?;

        local_state.remove(pos);
      } else {
        if let Some(pos) = global_state.iter().rposition(|e| e.id().eq(&id)) {
          global_state.remove(pos);
        } else {
          for prev_data in previous_data.iter_mut() {
            if let Some(prev_data) = prev_data.as_mut() {
              let duration = prev_data.end_time - prev_data.start_time;
              (*prev_data.map.entry(info.base().name().to_string()).or_insert((0u64, info.base().clone()))).0 += duration;
            }
          }
        }

        match previous_data.last() {
          None => durations.add_duration(events.first().unwrap(), event, info)?,
          Some(Some(last_data)) => {
            let stamp = get_stamp_or_err(event, info.time_attribute().as_ref())?;
            durations.add_raw_duration(stamp - last_data.end_time, info);
          }
          _ => {}
        };
      }
    }
  }

  for state in local_state.iter() {
    durations.add_raw_duration(end_time - get_stamp_or_err(state.event(), info.time_attribute().as_ref())?, info);
  }

  for _ in global_state.iter() {
    durations.add_raw_duration(end_time - start_time, info);
  }

  global_state.extend(local_state);

  Ok(durations)
}

fn get_duration(first: &Rc<RefCell<XesEventImpl>>, second: &Rc<RefCell<XesEventImpl>>, attribute: Option<&String>) -> Result<u64, SoftwareDataExtractionError> {
  Ok(get_stamp_or_err(second, attribute)? - get_stamp_or_err(first, attribute)?)
}

fn get_stamp_or_err(event: &Rc<RefCell<XesEventImpl>>, attribute: Option<&String>) -> Result<u64, SoftwareDataExtractionError> {
  get_stamp(&event.borrow(), attribute).map_err(|_| SoftwareDataExtractionError::FailedToGetStamp)
}