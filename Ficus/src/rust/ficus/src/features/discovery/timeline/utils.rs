use crate::{
  event_log::{
    core::event::event::{Event, EventPayloadValue},
    xes::xes_event::XesEventImpl,
  },
  features::discovery::timeline::discovery::LogThreadsDiagramError,
};
use chrono::{DateTime, Utc};
use std::sync::Arc;

pub fn extract_thread_id<TEvent: Event>(event: &TEvent, thread_attribute: &str) -> Option<Arc<str>> {
  let value = event.payload_map()?.get(thread_attribute)?;
  Some(value.to_string_repr().clone())
}

pub fn get_stamp(event: &XesEventImpl, attribute: Option<&str>) -> Result<i64, LogThreadsDiagramError> {
  if let Some(attribute) = attribute
    && let Some(map) = event.payload_map()
    && let Some(value) = map.get(attribute)
  {
    match value {
      EventPayloadValue::Int32(v) => return Ok(*v as i64),
      EventPayloadValue::Int64(v) => return Ok(*v),
      EventPayloadValue::Uint32(v) => return Ok(*v as i64),
      EventPayloadValue::Date(date) => return get_utc_date_stamp(date),
      _ => {}
    };
  }

  get_utc_date_stamp(event.timestamp())
}

fn get_utc_date_stamp(date: &DateTime<Utc>) -> Result<i64, LogThreadsDiagramError> {
  date.timestamp_nanos_opt().ok_or(LogThreadsDiagramError::NotSupportedEventStamp)
}
