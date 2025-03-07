use crate::event_log::core::event::event::{Event, EventPayloadValue};
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::analysis::threads_diagram::discovery::LogThreadsDiagramError;

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

pub fn get_stamp(event: &XesEventImpl, attribute: Option<&String>) -> Result<u64, LogThreadsDiagramError> {
  if let Some(attribute) = attribute {
    if let Some(map) = event.payload_map() {
      if let Some(value) = map.get(attribute) {
        match value {
          EventPayloadValue::Int32(v) => return Ok(*v as u64),
          EventPayloadValue::Int64(v) => return Ok(*v as u64),
          EventPayloadValue::Uint32(v) => return Ok(*v as u64),
          EventPayloadValue::Uint64(v) => return Ok(*v),
          _ => {}
        };
      }
    }
  }

  let utc_stamp = event.timestamp().timestamp_nanos_opt();
  if utc_stamp.is_none() || utc_stamp.unwrap() < 0 {
    Err(LogThreadsDiagramError::NotSupportedEventStamp)
  } else {
    Ok(utc_stamp.unwrap() as u64)
  }
}
