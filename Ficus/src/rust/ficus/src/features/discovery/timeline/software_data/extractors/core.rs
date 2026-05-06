use crate::{
  event_log::xes::xes_event::XesEventImpl,
  features::discovery::timeline::{events_groups::EventGroup, software_data::models::SoftwareData},
};
use std::{
  cell::RefCell,
  error::Error,
  fmt::{Debug, Display, Formatter},
  rc::Rc,
  str::FromStr,
  sync::Arc,
};

#[derive(Debug, Clone)]
pub enum SoftwareDataExtractionError {
  FailedToParseRegex(Arc<str>),
  FailedToParseValue(Arc<str>),
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
      .cloned()
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
    Err(_) => Err(SoftwareDataExtractionError::FailedToParseValue(Arc::from(format!(
      "Failed to parse value: {}",
      value
    )))),
  }
}
