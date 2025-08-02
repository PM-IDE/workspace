use crate::event_log::core::event::event::Event;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::software_data::extraction_config::{SimpleCountExtractionConfig, SoftwareDataExtractionConfig};
use crate::features::discovery::timeline::software_data::extractors::core::{parse_or_err, SoftwareDataExtractionError, EventGroupSoftwareDataExtractor};
use crate::features::discovery::timeline::software_data::extractors::general::utils::RegexParingResult;
use crate::features::discovery::timeline::software_data::models::{SimpleCounterData, SoftwareData};
use derive_new::new;
use fancy_regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, new)]
pub struct SimpleCounterExtractor<'a> {
  config: &'a SoftwareDataExtractionConfig,
}

impl<'a> EventGroupSoftwareDataExtractor for SimpleCounterExtractor<'a> {
  fn extract_from_events(&self, software_data: &mut SoftwareData, events: &[Rc<RefCell<XesEventImpl>>]) -> Result<(), SoftwareDataExtractionError> {
    if self.config.simple_counter_configs().is_empty() {
      return Ok(());
    }

    let regexes = self.config
      .simple_counter_configs()
      .iter()
      .map(|c|
        (
          Regex::new(c.event_class_regex()).map_err(|_| SoftwareDataExtractionError::FailedToParseRegex(c.event_class_regex().to_string())),
          c.info()
        )
      )
      .collect::<Vec<(RegexParingResult, &SimpleCountExtractionConfig)>>();

    let mut result = HashMap::new();
    for event in events {
      for (regex, config) in &regexes {
        match regex {
          Ok(regex) => {
            if regex.is_match(event.borrow().name()).unwrap_or(false) {
              let count = if let Some(count_attribute) = config.count_attr().as_ref() {
                if let Some(payload) = event.borrow().payload_map() {
                  if let Some(count_value) = payload.get(count_attribute) {
                    parse_or_err::<f64>(count_value.to_string_repr().as_str())?
                  } else {
                    continue;
                  }
                } else {
                  continue;
                }
              } else {
                1.
              };

              (*result.entry(config.base().name().to_string()).or_insert((config.base().units(), 0.))).1 += count;
            }
          }
          Err(err) => return Err(err.clone())
        }
      }
    }

    for (name, (units, count)) in result {
      software_data.simple_counters_mut().push(SimpleCounterData::new(name, count, units.to_owned()));
    }

    Ok(())
  }
}