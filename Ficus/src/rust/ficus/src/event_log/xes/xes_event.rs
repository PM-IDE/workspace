use chrono::{DateTime, Utc};
use std::{
  collections::HashMap,
  fmt::{Debug, Formatter},
  rc::Rc,
};

use crate::{
  event_log::core::event::{
    event::{Event, EventPayloadValue},
    event_base::EventBase,
  },
  utils::{
    user_data::user_data::{UserDataImpl, UserDataOwner},
    vec_utils,
  },
};

pub struct XesEventImpl {
  event_base: EventBase,
  payload: Option<HashMap<Rc<str>, EventPayloadValue>>,
}

impl Debug for XesEventImpl {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.name_pointer())
  }
}

impl XesEventImpl {
  pub fn new_all_fields(name: Rc<str>, timestamp: DateTime<Utc>, payload: Option<HashMap<Rc<str>, EventPayloadValue>>) -> Self {
    Self {
      event_base: EventBase::new(name, timestamp),
      payload,
    }
  }
}

impl PartialEq<Self> for XesEventImpl {
  fn eq(&self, other: &Self) -> bool {
    self.name().eq(other.name())
  }
}

impl UserDataOwner for XesEventImpl {
  fn user_data(&self) -> &UserDataImpl {
    &self.event_base.user_data
  }

  fn user_data_mut(&mut self) -> &mut UserDataImpl {
    &mut self.event_base.user_data
  }
}

impl Event for XesEventImpl {
  fn new(name: Rc<str>, timestamp: DateTime<Utc>) -> Self {
    Self {
      event_base: EventBase::new(name, timestamp),
      payload: None,
    }
  }

  fn new_with_min_date(name: Rc<str>) -> Self {
    Self::new(name, DateTime::<Utc>::MIN_UTC)
  }

  fn new_with_max_date(name: Rc<str>) -> Self {
    Self::new(name, DateTime::<Utc>::MAX_UTC)
  }

  fn name(&self) -> &str {
    &self.event_base.name
  }

  fn name_pointer(&self) -> &Rc<str> {
    &self.event_base.name
  }

  fn timestamp(&self) -> &DateTime<Utc> {
    &self.event_base.timestamp
  }

  fn payload_map(&self) -> Option<&HashMap<Rc<str>, EventPayloadValue>> {
    self.payload.as_ref()
  }

  fn payload_map_mut(&mut self) -> Option<&mut HashMap<Rc<str>, EventPayloadValue>> {
    self.payload.as_mut()
  }

  fn ordered_payload(&self) -> Vec<(&Rc<str>, &EventPayloadValue)> {
    let mut payload = Vec::new();
    if let Some(payload_map) = self.payload_map() {
      for (key, value) in payload_map {
        payload.push((key, value));
      }

      vec_utils::sort_by_first(&mut payload);
      payload
    } else {
      payload
    }
  }

  fn set_name(&mut self, new_name: Rc<str>) {
    self.event_base.name = new_name;
  }

  fn set_timestamp(&mut self, new_timestamp: DateTime<Utc>) {
    self.event_base.timestamp = new_timestamp;
  }

  fn add_or_update_payload(&mut self, key: Rc<str>, value: EventPayloadValue) {
    if self.payload.is_none() {
      self.payload = Some(HashMap::new());
    }

    self.payload.as_mut().unwrap().insert(key, value);
  }
}

impl Clone for XesEventImpl {
  fn clone(&self) -> Self {
    Self {
      event_base: self.event_base.clone(),
      payload: self.payload.clone(),
    }
  }
}
