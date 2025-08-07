use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::events_groups::EventGroup;
use crate::features::discovery::timeline::software_data::models::SoftwareData;
use std::cell::RefCell;
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