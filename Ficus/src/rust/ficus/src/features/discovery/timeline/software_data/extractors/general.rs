use crate::event_log::core::event::event::Event;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::software_data::extraction_config::SoftwareDataExtractionConfig;
use crate::features::discovery::timeline::software_data::extractors::core::{parse_or_err, SoftwareDataExtractionError, SoftwareDataExtractor};
use crate::features::discovery::timeline::software_data::models::{HistogramData, HistogramEntry, SimpleCounterData, SoftwareData};
use derive_new::new;
use fancy_regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, new)]
pub struct GeneralHistogramExtractor<'a> {
  config: &'a SoftwareDataExtractionConfig,
}

impl<'a> SoftwareDataExtractor for GeneralHistogramExtractor<'a> {
  fn extract_from_events(&self, software_data: &mut SoftwareData, events: &[Rc<RefCell<XesEventImpl>>]) -> Result<(), SoftwareDataExtractionError> {
    if self.config.histogram_extraction_configs().is_empty() {
      return Ok(());
    }

    let regexes = self.config
      .histogram_extraction_configs()
      .iter()
      .map(|c|
        (
          Regex::new(c.event_class_regex()).map_err(|_| SoftwareDataExtractionError::FailedToParseRegex(c.event_class_regex().to_string())),
          c.info().name(),
          c.info().count_attr(),
          c.info().grouping_attr()
        )
      )
      .collect::<Vec<(Result<Regex, SoftwareDataExtractionError>, &String, &String, &String)>>();

    let mut result = HashMap::new();
    for event in events {
      if let Some(payload) = event.borrow().payload_map() {
        for (regex, name, count_attr, grouping_attr) in &regexes {
          match regex {
            Ok(regex) => {
              if regex.is_match(event.borrow().name()).unwrap_or(false) {
                let count = if let Some(count) = payload.get(*count_attr) {
                  parse_or_err::<f64>(count.to_string_repr().as_str())?
                } else {
                  continue
                };

                let grouping_value = if let Some(grouping_value) = payload.get(*grouping_attr) {
                  grouping_value.to_string_repr()
                } else {
                  continue
                };

                *result.entry(name).or_insert(HashMap::new()).entry(grouping_value.to_string()).or_insert(0.) += count;
              }
            }
            Err(err) => return Err(err.clone())
          }
        }
      }
    }

    for (name, counts) in result {
      software_data.histograms_mut().push(HistogramData::new(
        name.to_string(),
        counts.into_iter().map(|(k, v)| HistogramEntry::new(k, v)).collect(),
      ))
    }

    Ok(())
  }
}

#[derive(Clone, Debug, new)]
pub struct SimpleCounterExtractor<'a> {
  config: &'a SoftwareDataExtractionConfig,
}

impl<'a> SoftwareDataExtractor for SimpleCounterExtractor<'a> {
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
          c.info().name()
        )
      )
      .collect::<Vec<(Result<Regex, SoftwareDataExtractionError>, &String)>>();

    let mut result = HashMap::new();
    for event in events {
      for (regex, name) in &regexes {
        match regex {
          Ok(regex) => {
            if regex.is_match(event.borrow().name()).unwrap_or(false) {
              *result.entry(name.to_string()).or_insert(0.) += 1.;
            }
          }
          Err(err) => return Err(err.clone())
        }
      }
    }

    for (name, count) in result {
      software_data.simple_counters_mut().push(SimpleCounterData::new(name, count));
    }

    Ok(())
  }
}