use crate::event_log::core::event::event::EventPayloadValue;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::events_groups::EventGroup;
use crate::features::discovery::timeline::software_data::extraction_config::ExtractionConfig;
use crate::features::discovery::timeline::software_data::models::SoftwareData;
use fancy_regex::Regex;
use log::warn;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum SoftwareDataExtractionError {
  FailedToParseRegex(String),
  FailedToParseValue(String),
  FailedToGetStamp,
}

impl Display for SoftwareDataExtractionError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      SoftwareDataExtractionError::FailedToParseRegex(regex) => f.write_str(format!("Failed to parse regex {}", regex).as_str()),
      SoftwareDataExtractionError::FailedToParseValue(value) => f.write_str(format!("Failed to parse value {}", value).as_str()),
      SoftwareDataExtractionError::FailedToGetStamp => f.write_str("Failed to get stamp"),
    }
  }
}

impl Error for SoftwareDataExtractionError {}

pub trait EventGroupSoftwareDataExtractor {
  fn extract(&self, software_data: &mut SoftwareData, event_group: &EventGroup) -> Result<(), SoftwareDataExtractionError> {
    let events = event_group
      .all_events()
      .into_iter()
      .map(|c| c.clone())
      .collect::<Vec<Rc<RefCell<XesEventImpl>>>>();

    self.extract_from_events(software_data, events.as_slice())
  }

  fn extract_from_events(
    &self,
    software_data: &mut SoftwareData,
    events: &[Rc<RefCell<XesEventImpl>>],
  ) -> Result<(), SoftwareDataExtractionError>;
}

pub trait EventGroupTraceSoftwareDataExtractor {
  fn extract(&self, trace: &Vec<EventGroup>, data: &mut Vec<(SoftwareData, SoftwareData)>) -> Result<(), SoftwareDataExtractionError>;
}

pub(super) fn parse_or_err<ToType: FromStr>(value: &str) -> Result<ToType, SoftwareDataExtractionError> {
  match value.parse::<ToType>() {
    Ok(value) => Ok(value),
    Err(_) => Err(SoftwareDataExtractionError::FailedToParseValue(format!(
      "Failed to parse value: {}",
      value
    ))),
  }
}

pub(super) fn regex_or_err(regex_str: &str) -> Result<Regex, SoftwareDataExtractionError> {
  match Regex::new(regex_str) {
    Ok(regex) => Ok(regex),
    Err(_) => Err(SoftwareDataExtractionError::FailedToParseRegex(regex_str.to_owned())),
  }
}

pub(super) fn regex_option_or_err(regex_str: Option<&String>) -> Result<Option<Regex>, SoftwareDataExtractionError> {
  match regex_str {
    None => Ok(None),
    Some(regex_str) => match regex_or_err(regex_str.as_str()) {
      Ok(regex) => Ok(Some(regex)),
      Err(err) => Err(err),
    },
  }
}

pub(super) fn payload_value_or_none(payload: &HashMap<String, EventPayloadValue>, attribute_name: &str) -> Option<String> {
  if let Some(value) = payload.get(attribute_name) {
    Some(value.to_string_repr().to_string())
  } else {
    warn!("Failed to get value for attribute {}", attribute_name);
    None
  }
}

pub(super) fn prepare_configs<'a, TConfig: Clone + Debug, TEnum: Clone>(
  configs: &'a [(&Option<ExtractionConfig<TConfig>>, TEnum)],
) -> Result<Vec<(Regex, &'a TConfig, TEnum)>, SoftwareDataExtractionError> {
  let mut result = vec![];

  for config in configs {
    if let Some(extraction_config) = config.0 {
      result.push((
        regex_or_err(extraction_config.event_class_regex().as_str())?,
        extraction_config.info(),
        config.1.clone(),
      ))
    }
  }

  Ok(result)
}

pub(super) fn prepare_functional_configs<TData: Clone>(
  configs: &[(Option<&String>, TData)],
) -> Result<Vec<(Regex, TData)>, SoftwareDataExtractionError> {
  let mut result = vec![];

  for config in configs {
    if let Some(regex) = regex_option_or_err(config.0)? {
      result.push((regex, config.1.clone()))
    }
  }

  Ok(result)
}
