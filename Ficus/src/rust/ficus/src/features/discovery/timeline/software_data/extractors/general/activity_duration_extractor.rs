use crate::event_log::core::event::event::Event;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::software_data::extraction_config::{ActivityDurationExtractionConfig, SoftwareDataExtractionConfig};
use crate::features::discovery::timeline::software_data::extractors::core::{SoftwareDataExtractionError, SoftwareDataExtractor};
use crate::features::discovery::timeline::software_data::extractors::general::utils::RegexParingResult;
use crate::features::discovery::timeline::software_data::models::{ActivityDurationData, SoftwareData};
use crate::features::discovery::timeline::utils::get_stamp;
use derive_new::new;
use fancy_regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, new)]
pub struct ActivityDurationExtractor<'a> {
  config: &'a SoftwareDataExtractionConfig,
}

pub enum LastSeenEvent {
  Start(Rc<RefCell<XesEventImpl>>),
  End(Rc<RefCell<XesEventImpl>>),
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
          Regex::new(c.info().start_event_regex()).map_err(|_| SoftwareDataExtractionError::FailedToParseRegex(c.event_class_regex().to_string())),
          Regex::new(c.info().end_event_regex()).map_err(|_| SoftwareDataExtractionError::FailedToParseRegex(c.event_class_regex().to_string())),
          c.info(),
          None
        )
      )
      .collect::<Vec<(RegexParingResult, RegexParingResult, &ActivityDurationExtractionConfig, Option<LastSeenEvent>)>>();

    let mut durations: HashMap<String, (u64, String)> = HashMap::new();
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

        if start_regex.is_match(event.borrow().name()).unwrap_or(false) {
          match state {
            None => {
              let duration = get_duration(&events.first().unwrap().borrow(), &event.borrow(), info.time_attribute().as_ref())?;
              (*durations.entry(info.name().to_string()).or_insert((0u64, info.units().to_string()))).0 += duration;
            }
            Some(_) => {}
          };

          *state = Some(LastSeenEvent::Start(event.clone()));
        }

        if end_regex.is_match(event.borrow().name()).unwrap_or(false) {
          match state {
            None => {}
            Some(state) => {
              if let LastSeenEvent::Start(start_event) = state {
                let duration = get_duration(&start_event.borrow(), &event.borrow(), info.time_attribute().as_ref())?;
                (*durations.entry(info.name().to_string()).or_insert((0u64, info.units().to_string()))).0 += duration;
              }
            }
          };

          *state = Some(LastSeenEvent::End(event.clone()));
        }
      }
    }

    for (name, (value, units)) in durations {
      software_data.activities_duration_mut().push(ActivityDurationData::new(name, value as f64, units));
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