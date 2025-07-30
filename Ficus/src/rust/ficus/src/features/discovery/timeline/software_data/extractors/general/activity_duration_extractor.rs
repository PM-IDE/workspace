use crate::event_log::core::event::event::Event;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::software_data::extraction_config::{ActivityDurationExtractionConfig, SoftwareDataExtractionConfig};
use crate::features::discovery::timeline::software_data::extractors::core::{SoftwareDataExtractionError, SoftwareDataExtractor};
use crate::features::discovery::timeline::software_data::extractors::general::utils::RegexParingResult;
use crate::features::discovery::timeline::software_data::models::{ActivityDurationData, SoftwareData};
use crate::features::discovery::timeline::utils::get_stamp;
use derive_new::new;
use fancy_regex::Regex;
use getset::Getters;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, new)]
pub struct ActivityDurationExtractor<'a> {
  config: &'a SoftwareDataExtractionConfig,
}

#[derive(Getters, new)]
struct StackEntry {
  #[getset(get = "pub")] id: Option<String>,
  #[getset(get = "pub")] event: LastSeenEvent,
}

pub enum LastSeenEvent {
  Start(Rc<RefCell<XesEventImpl>>),
  End(Rc<RefCell<XesEventImpl>>),
}

impl LastSeenEvent {
  fn is_start(&self) -> bool {
    match self {
      LastSeenEvent::Start(_) => true,
      LastSeenEvent::End(_) => false
    }
  }

  fn event(&self) -> &Rc<RefCell<XesEventImpl>> {
    match self {
      LastSeenEvent::Start(e) => e,
      LastSeenEvent::End(e) => e
    }
  }
}

impl<'a> SoftwareDataExtractor for ActivityDurationExtractor<'a> {
  fn extract_from_events(&self, software_data: &mut SoftwareData, events: &[Rc<RefCell<XesEventImpl>>]) -> Result<(), SoftwareDataExtractionError> {
    if self.config.activities_duration_configs().len() == 0 {
      return Ok(());
    }

    let mut configs = self.config
      .activities_duration_configs()
      .iter()
      .map(|c|
        (
          Regex::new(c.start_event_regex()).map_err(|_| SoftwareDataExtractionError::FailedToParseRegex(c.start_event_regex().to_string())),
          Regex::new(c.end_event_regex()).map_err(|_| SoftwareDataExtractionError::FailedToParseRegex(c.end_event_regex().to_string())),
          c,
          Vec::new()
        )
      )
      .collect::<Vec<(RegexParingResult, RegexParingResult, &ActivityDurationExtractionConfig, Vec<StackEntry>)>>();

    let mut durations: HashMap<String, (u64, String)> = HashMap::new();

    let mut add_duration = |
      start_event: &Rc<RefCell<XesEventImpl>>,
      end_event: &Rc<RefCell<XesEventImpl>>,
      info: &ActivityDurationExtractionConfig
    | -> Result<(), SoftwareDataExtractionError> {
      let duration = get_duration(&start_event.borrow(), &end_event.borrow(), info.time_attribute().as_ref())?;
      (*durations.entry(info.name().to_string()).or_insert((0u64, info.units().to_string()))).0 += duration;

      Ok(())
    };

    for event in events {
      for (start_regex, end_regex, info, state) in configs.iter_mut() {
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
          state.push(StackEntry::new(id, LastSeenEvent::Start(event.clone())));
          continue;
        }

        if end_regex.is_match(event.borrow().name()).unwrap_or(false) {
          if let Some(pos) = state.iter().rposition(|e| e.event().is_start() && e.id().eq(&id)) {
            let start_event = state[pos].event().event();
            add_duration(start_event, event, info)?;

            state.remove(pos);
          } else {
            add_duration(events.first().unwrap(), event, info)?;
          }
        }
      }
    }

    for (name, (value, units)) in durations {
      software_data.activities_durations_mut().push(ActivityDurationData::new(name, value as f64, units));
    }

    Ok(())
  }
}

fn get_duration(first: &XesEventImpl, second: &XesEventImpl, attribute: Option<&String>) -> Result<u64, SoftwareDataExtractionError> {
  Ok(get_stamp_or_err(second, attribute)? - get_stamp_or_err(first, attribute)?)
}

fn get_stamp_or_err(event: &XesEventImpl, attribute: Option<&String>) -> Result<u64, SoftwareDataExtractionError> {
  get_stamp(event, attribute).map_err(|e| SoftwareDataExtractionError::FailedToGetStamp)
}