use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::software_data::models::SoftwareData;
use std::cell::RefCell;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

#[derive(Debug)]
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

pub trait SoftwareDataExtractor {
  fn extract(&self, software_data: &mut SoftwareData, event_group: &Vec<Rc<RefCell<XesEventImpl>>>) -> Result<(), SoftwareDataExtractionError>;
}