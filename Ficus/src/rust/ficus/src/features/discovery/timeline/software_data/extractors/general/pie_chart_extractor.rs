use crate::event_log::core::event::event::Event;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::software_data::extraction_config::{PieChartExtractionConfig, SoftwareDataExtractionConfig};
use crate::features::discovery::timeline::software_data::extractors::core::{parse_or_err, SoftwareDataExtractionError, EventGroupSoftwareDataExtractor};
use crate::features::discovery::timeline::software_data::extractors::general::utils::RegexParingResult;
use crate::features::discovery::timeline::software_data::models::{GenericEnhancementBase, HistogramData, HistogramEntry, SoftwareData};
use derive_new::new;
use fancy_regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, new)]
pub struct PieChartExtractor<'a> {
  config: &'a SoftwareDataExtractionConfig,
}

impl<'a> EventGroupSoftwareDataExtractor for PieChartExtractor<'a> {
  fn extract_from_events(&self, software_data: &mut SoftwareData, events: &[Rc<RefCell<XesEventImpl>>]) -> Result<(), SoftwareDataExtractionError> {
    if self.config.pie_chart_extraction_configs().is_empty() {
      return Ok(());
    }

    let regexes = self.config
      .pie_chart_extraction_configs()
      .iter()
      .map(|c|
        (
          Regex::new(c.event_class_regex()).map_err(|_| SoftwareDataExtractionError::FailedToParseRegex(c.event_class_regex().to_string())),
          c.info()
        )
      )
      .collect::<Vec<(RegexParingResult, &PieChartExtractionConfig)>>();

    let mut result = HashMap::new();
    for event in events {
      if let Some(payload) = event.borrow().payload_map() {
        for (regex, config) in &regexes {
          match regex {
            Ok(regex) => {
              if regex.is_match(event.borrow().name()).unwrap_or(false) {
                let count = if let Some(count_attr) = config.count_attr() {
                  if let Some(count) = payload.get(count_attr) {
                    parse_or_err::<f64>(count.to_string_repr().as_str())?
                  } else {
                    continue;
                  }
                } else {
                  1.
                };

                let grouping_value = if let Some(strategy) = config.grouping_attr() {
                  strategy.create(&event.borrow())
                } else {
                  event.borrow().name().to_string()
                };

                *result.entry(config.base().name()).or_insert((config.base(), HashMap::new())).1.entry(grouping_value).or_insert(0.) += count;
              }
            }
            Err(err) => return Err(err.clone())
          }
        }
      }
    }

    for (_, (base, counts)) in result {
      software_data.histograms_mut().push(HistogramData::new(
        GenericEnhancementBase::new(base.name().to_string(), base.units().to_string(), base.group().clone()),
        counts.into_iter().map(|(k, v)| HistogramEntry::new(k, v)).collect(),
      ))
    }

    Ok(())
  }
}