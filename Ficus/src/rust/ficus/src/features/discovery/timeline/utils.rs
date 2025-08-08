use chrono::{DateTime, Utc};
use crate::event_log::core::event::event::{Event, EventPayloadValue};
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::discovery::LogThreadsDiagramError;

pub fn extract_thread_id<TEvent: Event>(event: &TEvent, thread_attribute: &str) -> Option<String> {
  if let Some(map) = event.payload_map() {
    if let Some(value) = map.get(thread_attribute) {
      Some(value.to_string_repr().as_str().to_owned())
    } else {
      None
    }
  } else {
    None
  }
}

pub fn get_stamp(event: &XesEventImpl, attribute: Option<&String>) -> Result<i64, LogThreadsDiagramError> {
  if let Some(attribute) = attribute {
    if let Some(map) = event.payload_map() {
      if let Some(value) = map.get(attribute) {
        match value {
          EventPayloadValue::Int32(v) => return Ok(*v as i64),
          EventPayloadValue::Int64(v) => return Ok(*v),
          EventPayloadValue::Uint32(v) => return Ok(*v as i64),
          EventPayloadValue::Date(date) => return get_utc_date_stamp(date),
          _ => {}
        };
      }
    }
  }

  get_utc_date_stamp(event.timestamp())
}

fn get_utc_date_stamp(date: &DateTime<Utc>) -> Result<i64, LogThreadsDiagramError> {
  if let Some(utc_stamp) = date.timestamp_nanos_opt() {
    Ok(utc_stamp)
  } else {
    Err(LogThreadsDiagramError::NotSupportedEventStamp)
  }
}