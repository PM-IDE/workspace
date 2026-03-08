use crate::utils::user_data::user_data::UserDataOwner;

use super::lifecycle::xes_lifecycle::Lifecycle;
use chrono::{DateTime, Utc};
use std::{collections::HashMap, fmt::Debug, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub enum EventPayloadValue {
  Null,
  Date(DateTime<Utc>),
  String(Rc<str>),
  Boolean(bool),
  Int32(i32),
  Int64(i64),
  Float32(f32),
  Float64(f64),
  Uint32(u32),
  Uint64(u64),
  Guid(uuid::Uuid),
  Timestamp(i64),
  Lifecycle(Lifecycle),
  Artifact(EventPayloadArtifact),
  Drivers(EventPayloadDrivers),
  SoftwareEvent(EventPayloadSoftwareEventType),
}

#[derive(Debug, Clone, PartialEq)]
pub struct EventPayloadArtifact {
  pub items: Vec<EventPayloadArtifactItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EventPayloadArtifactItem {
  pub model: String,
  pub instance: String,
  pub transition: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EventPayloadDrivers {
  pub drivers: Vec<EventPayloadDriver>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EventPayloadDriver {
  pub amount: f64,
  pub name: String,
  pub driver_type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventPayloadSoftwareEventType {
  Unspecified = 0,
  Call = 1,
  Return = 2,
  Throws = 3,
  Handle = 4,
  Calling = 5,
  Returning = 6,
}

impl EventPayloadValue {
  pub fn to_string_repr(&self) -> Rc<str> {
    if let EventPayloadValue::String(string) = self {
      return string.clone();
    }

    Rc::from(match self {
      EventPayloadValue::Null => "NULL".to_string(),
      EventPayloadValue::Date(date) => date.to_rfc3339(),
      EventPayloadValue::Boolean(bool) => bool.to_string(),
      EventPayloadValue::Int32(int) => int.to_string(),
      EventPayloadValue::Float32(float) => float.to_string(),
      EventPayloadValue::Int64(value) => value.to_string(),
      EventPayloadValue::Float64(value) => value.to_string(),
      EventPayloadValue::Uint32(value) => value.to_string(),
      EventPayloadValue::Uint64(value) => value.to_string(),
      EventPayloadValue::Guid(value) => value.to_string(),
      EventPayloadValue::Timestamp(value) => value.to_string(),
      EventPayloadValue::Lifecycle(lifecycle) => lifecycle.to_string(),
      EventPayloadValue::Artifact(artifact) => format!("{:?}", artifact),
      EventPayloadValue::Drivers(drivers) => format!("{:?}", drivers),
      EventPayloadValue::SoftwareEvent(software_event) => format!("{:?}", software_event),
      EventPayloadValue::String { .. } => unreachable!(),
    })
  }
}

pub trait Event: Clone + Debug + UserDataOwner {
  fn new(name: Rc<str>, stamp: DateTime<Utc>) -> Self;
  fn new_with_min_date(name: Rc<str>) -> Self;
  fn new_with_max_date(name: Rc<str>) -> Self;

  fn name(&self) -> &str;
  fn name_pointer(&self) -> &Rc<str>;

  fn timestamp(&self) -> &DateTime<Utc>;
  fn payload_map(&self) -> Option<&HashMap<Rc<str>, EventPayloadValue>>;
  fn payload_map_mut(&mut self) -> Option<&mut HashMap<Rc<str>, EventPayloadValue>>;
  fn ordered_payload(&self) -> Vec<(&Rc<str>, &EventPayloadValue)>;

  fn set_name(&mut self, new_name: Rc<str>);
  fn set_timestamp(&mut self, new_timestamp: DateTime<Utc>);
  fn add_or_update_payload(&mut self, key: Rc<str>, value: EventPayloadValue);
}
