use std::{cell::RefCell, collections::HashMap, rc::Rc};

use chrono::{DateTime, Utc};
use quick_xml::Reader;

use crate::event_log::xes::constants::*;
use crate::event_log::xes::reader::file_xes_log_reader::XmlReader;
use crate::event_log::{core::event::event::EventPayloadValue, xes::xes_event::XesEventImpl};

use super::utils;

pub struct TraceXesEventLogIterator<'a> {
  buffer: Vec<u8>,
  reader: Rc<RefCell<Reader<XmlReader<'a>>>>,
  globals: Rc<RefCell<HashMap<String, HashMap<String, EventPayloadValue>>>>,
}

impl<'a> Iterator for TraceXesEventLogIterator<'a> {
  type Item = XesEventImpl;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      let event = self.reader.borrow_mut().read_event_into(&mut self.buffer);
      match event {
        Ok(quick_xml::events::Event::Start(e)) => match e.name().0 {
          EVENT_TAG_NAME => match self.try_parse_event_from() {
            None => continue,
            Some(parsed_event) => return Some(parsed_event),
          },
          _ => continue,
        },
        Ok(quick_xml::events::Event::End(e)) => match e.name().0 {
          TRACE_TAG_NAME => return None,
          _ => continue,
        },
        Err(_) => return None,
        _ => continue,
      }
    }
  }
}

impl<'a> TraceXesEventLogIterator<'a> {
  pub(crate) fn new(
    reader: Rc<RefCell<Reader<XmlReader>>>,
    seen_globals: Rc<RefCell<HashMap<String, HashMap<String, EventPayloadValue>>>>,
  ) -> TraceXesEventLogIterator {
    TraceXesEventLogIterator {
      reader,
      buffer: Vec::new(),
      globals: seen_globals,
    }
  }

  fn try_parse_event_from(&mut self) -> Option<XesEventImpl> {
    let mut name = None;
    let mut date = None;
    let mut payload = HashMap::new();

    self.set_defaults_value(&mut name, &mut date, &mut payload);

    loop {
      match self.reader.borrow_mut().read_event_into(&mut self.buffer) {
        Ok(quick_xml::events::Event::End(end)) => match end.name().0 {
          EVENT_TAG_NAME => {
            if !name.is_some() {
              return None;
            }
            if !date.is_some() {
              return None;
            }

            let event = XesEventImpl::new_all_fields(name.unwrap(), date.unwrap(), Some(payload));
            return Some(event);
          }
          _ => continue,
        },
        Ok(quick_xml::events::Event::Empty(empty)) => match utils::read_payload_like_tag(&empty) {
          Some(descriptor) => {
            let payload_type = descriptor.payload_type.as_str().as_bytes();
            let key = descriptor.key.as_str();
            let value = descriptor.value.as_str();

            Self::set_parsed_value(payload_type, key, value, &mut name, &mut date, &mut payload, &self.globals.borrow());
          }
          None => continue,
        },
        _ => continue,
      }
    }
  }

  fn set_defaults_value(
    &self,
    name: &mut Option<Rc<Box<String>>>,
    date: &mut Option<DateTime<Utc>>,
    payload: &mut HashMap<String, EventPayloadValue>,
  ) {
    let globals = self.globals.borrow_mut();
    if !globals.contains_key(EVENT_TAG_NAME_STR) {
      return;
    }

    for (key, value) in globals.get(EVENT_TAG_NAME_STR).unwrap() {
      Self::update_event_data(key, value.clone(), date, name, payload);
    }
  }

  fn set_parsed_value(
    payload_type: &[u8],
    key: &str,
    value: &str,
    name: &mut Option<Rc<Box<String>>>,
    date: &mut Option<DateTime<Utc>>,
    payload: &mut HashMap<String, EventPayloadValue>,
    globals: &HashMap<String, HashMap<String, EventPayloadValue>>,
  ) -> bool {
    let payload_value = utils::extract_payload_value(payload_type, key, value);
    if !payload_value.is_some() {
      return false;
    }

    if let Some(event_globals) = globals.get(EVENT_TAG_NAME_STR) {
      if let Some(default_value) = event_globals.get(key) {
        if default_value == payload_value.as_ref().unwrap() {
          return false;
        }
      }
    }

    Self::update_event_data(key, payload_value.unwrap(), date, name, payload);
    true
  }

  fn update_event_data(
    key: &str,
    payload_value: EventPayloadValue,
    date: &mut Option<DateTime<Utc>>,
    name: &mut Option<Rc<Box<String>>>,
    payload: &mut HashMap<String, EventPayloadValue>,
  ) {
    match key {
      TIME_TIMESTAMP_STR => {
        if let EventPayloadValue::Date(parsed_date) = payload_value {
          *date = Some(parsed_date);
        }
      }
      CONCEPT_NAME_STR => {
        if let EventPayloadValue::String(parsed_string) = payload_value {
          *name = Some(parsed_string.clone());
        }
      }
      _ => {
        if payload.contains_key(key) {
          *payload.get_mut(key).unwrap() = payload_value;
        } else {
          payload.insert(key.to_owned(), payload_value);
        }
      }
    }
  }
}
