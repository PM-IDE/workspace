use crate::models::domain::bxes_value::BxesValue;
use std::{fmt::Display, string::FromUtf8Error};

#[derive(Debug)]
pub enum BxesReadError {
  FailedToOpenFile(String),
  FailedToReadValue(FailedToReadValueError),
  FailedToReadPos(String),
  FailedToCreateUtf8String(FromUtf8Error),
  FailedToParseTypeId(u8),
  FailedToIndexValue(usize),
  FailedToIndexKeyValue(usize),
  LifecycleOfEventOutOfRange,
  EventAttributeKeyIsNotAString,
  VersionsMismatchError(VersionsMismatchError),
  FailedToExtractArchive,
  TooManyFilesInArchive,
  FailedToCreateTempDir,
  InvalidArchive(String),
  ExpectedString(BxesValue),
  Leb128ReadError(String),
  ValueAttributeNameIsNotAString,
}

impl Display for BxesReadError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        BxesReadError::FailedToOpenFile(value) => format!("Failed to open file {}", value),
        BxesReadError::FailedToReadValue(err) => {
          format!("Failed to read value: {}", err.to_string())
        }
        BxesReadError::FailedToReadPos(error_message) => {
          format!("Failed to read pos from stream: {}", error_message)
        }
        BxesReadError::FailedToCreateUtf8String(err) => {
          format!("Failed to create string: {}", err)
        }
        BxesReadError::FailedToParseTypeId(type_id) => {
          format!("Failed to parse type id: {}", type_id)
        }
        BxesReadError::FailedToIndexValue(index) => {
          format!("Failed to find bxes value for index: {}", index)
        }
        BxesReadError::FailedToIndexKeyValue(index) => {
          format!("Failed to find kv pair for index: {}", index)
        }
        BxesReadError::LifecycleOfEventOutOfRange => "LifecycleOfEventOutOfRange".to_string(),
        BxesReadError::EventAttributeKeyIsNotAString => "EventAttributeKeyIsNotAString".to_string(),
        BxesReadError::VersionsMismatchError(err) => err.to_string(),
        BxesReadError::FailedToExtractArchive => "FailedToExtractArchive".to_string(),
        BxesReadError::TooManyFilesInArchive => "TooManyFilesInArchive".to_string(),
        BxesReadError::FailedToCreateTempDir => "FailedToCreateTempDir".to_string(),
        BxesReadError::InvalidArchive(message) => format!("Invalid bxes archive: {}", message),
        BxesReadError::ExpectedString(value) => {
          format!("Expected string value, found: {:?}", value)
        }
        BxesReadError::Leb128ReadError(message) => {
          format!("Failed to read LEB128 encoded value: {}", message)
        }
        BxesReadError::ValueAttributeNameIsNotAString => "Value attribute name was not a string".to_string(),
      }
    )
  }
}

#[derive(Debug)]
pub struct FailedToReadValueError {
  pub offset: usize,
  pub message: String,
}

impl Display for FailedToReadValueError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Failed to read value at offset {}, error: {}", self.offset, self.message)
  }
}

impl FailedToReadValueError {
  pub fn new(offset: usize, message: String) -> Self {
    Self { offset, message }
  }
}

#[derive(Debug)]
pub struct VersionsMismatchError {
  previous_version: u32,
  current_version: u32,
}

impl Display for VersionsMismatchError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Versions mismatch: previous version: {}, current version: {}",
      self.previous_version, self.current_version
    )
  }
}

impl VersionsMismatchError {
  pub fn new(previous_version: u32, current_version: u32) -> Self {
    Self {
      previous_version,
      current_version,
    }
  }
}
