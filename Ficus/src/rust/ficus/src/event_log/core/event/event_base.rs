use std::rc::Rc;

use chrono::{DateTime, Utc};

use crate::utils::user_data::user_data::UserDataHolder;

#[derive(Debug)]
pub struct EventBase {
  pub name: Rc<Box<String>>,
  pub timestamp: DateTime<Utc>,
  pub user_data_holder: UserDataHolder,
}

impl EventBase {
  pub fn new(name: Rc<Box<String>>, timestamp: DateTime<Utc>) -> Self {
    Self {
      name,
      timestamp,
      user_data_holder: UserDataHolder::new(),
    }
  }
}

impl Clone for EventBase {
  fn clone(&self) -> Self {
    Self {
      name: self.name.clone(),
      timestamp: self.timestamp.clone(),
      user_data_holder: self.user_data_holder.clone(),
    }
  }
}
