use crate::{
  event_log::{core::event::event::Event, xes::xes_event::XesEventImpl},
  features::discovery::timeline::software_data::{
    extraction_config::{SimpleCountExtractionConfig, SoftwareDataExtractionConfig},
    extractors::{
      core::{EventGroupSoftwareDataExtractor, SoftwareDataExtractionError, parse_or_err},
      utils::RegexParingResult,
    },
    models::{GenericEnhancementBase, SimpleCounterData, SoftwareData},
  },
};
use derive_new::new;
use fancy_regex::Regex;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, Debug, new)]
pub struct SimpleCounterExtractor<'a> {
  config: &'a SoftwareDataExtractionConfig,
}

impl<'a> EventGroupSoftwareDataExtractor for SimpleCounterExtractor<'a> {
  fn extract_from_events(
    &self,
    software_data: &mut SoftwareData,
    events: &[Rc<RefCell<XesEventImpl>>],
  ) -> Result<(), SoftwareDataExtractionError> {
    if self.config.simple_counter_configs().is_empty() {
      return Ok(());
    }

    let regexes = self
      .config
      .simple_counter_configs()
      .iter()
      .map(|c| {
        (
          Regex::new(c.event_class_regex()).map_err(|_| SoftwareDataExtractionError::FailedToParseRegex(c.event_class_regex().to_string())),
          c.info(),
        )
      })
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

              result.entry(config.base().name().to_string()).or_insert((config.base(), 0.)).1 += count;
            }
          }
          Err(err) => return Err(err.clone()),
        }
      }
    }

    for (_, (base, count)) in result {
      software_data.simple_counters_mut().push(SimpleCounterData::new(
        GenericEnhancementBase::new(base.name().clone(), base.units().clone(), base.group().clone()),
        count,
      ));
    }

    Ok(())
  }
}
