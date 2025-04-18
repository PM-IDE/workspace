use std::rc::Rc;

use chrono::{DateTime, Utc};

use crate::utils::user_data::user_data::UserDataImpl;

#[derive(Debug)]
pub struct EventBase {
  pub name: Rc<Box<String>>,
  pub timestamp: DateTime<Utc>,
  pub user_data: UserDataImpl,
}

impl EventBase {
  pub fn new(name: Rc<Box<String>>, timestamp: DateTime<Utc>) -> Self {
    Self {
      name,
      timestamp,
      user_data: UserDataImpl::new(),
    }
  }
}

impl Clone for EventBase {
  fn clone(&self) -> Self {
    Self {
      name: self.name.clone(),
      timestamp: self.timestamp.clone(),
      user_data: self.user_data.clone(),
    }
  }
}
