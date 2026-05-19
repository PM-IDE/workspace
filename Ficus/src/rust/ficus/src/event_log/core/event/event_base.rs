use chrono::{DateTime, Utc};
use std::sync::Arc;

use crate::utils::user_data::user_data::UserDataImpl;

#[derive(Debug)]
pub struct EventBase {
  pub name: Arc<str>,
  pub timestamp: DateTime<Utc>,
  pub user_data: UserDataImpl,
}

impl EventBase {
  pub fn new(name: Arc<str>, timestamp: DateTime<Utc>) -> Self {
    Self {
      name,
      timestamp,
      user_data: Default::default(),
    }
  }
}

impl Clone for EventBase {
  fn clone(&self) -> Self {
    Self {
      name: self.name.clone(),
      timestamp: self.timestamp,
      user_data: self.user_data.clone(),
    }
  }
}
