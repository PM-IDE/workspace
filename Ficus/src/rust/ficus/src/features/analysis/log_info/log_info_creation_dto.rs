use crate::event_log::core::event_log::EventLog;
use std::{collections::HashSet, rc::Rc};

pub struct EventLogInfoCreationDto<'a, TLog>
where
  TLog: EventLog,
{
  pub(super) log: &'a TLog,
  pub(super) add_fake_start_end_events: bool,
  pub(super) ignored_events: Option<&'a HashSet<Rc<str>>>,
  pub(super) thread_attribute: Option<Rc<str>>,
}

impl<'a, TLog> EventLogInfoCreationDto<'a, TLog>
where
  TLog: EventLog,
{
  pub fn default(log: &'a TLog) -> Self {
    EventLogInfoCreationDto {
      log,
      add_fake_start_end_events: false,
      ignored_events: None,
      thread_attribute: None,
    }
  }

  pub fn default_fake_events(log: &'a TLog) -> Self {
    Self {
      log,
      add_fake_start_end_events: true,
      ignored_events: None,
      thread_attribute: None,
    }
  }

  pub fn default_fake_ignored(log: &'a TLog, ignored_events: Option<&'a HashSet<Rc<str>>>) -> Self {
    Self {
      log,
      add_fake_start_end_events: true,
      ignored_events,
      thread_attribute: None,
    }
  }

  pub fn default_ignore(log: &'a TLog, ignored_events: &'a HashSet<Rc<str>>) -> Self {
    Self {
      log,
      add_fake_start_end_events: false,
      ignored_events: Some(ignored_events),
      thread_attribute: None,
    }
  }

  pub fn default_thread(log: &'a TLog, thread_attribute: Rc<str>) -> Self {
    Self {
      log,
      add_fake_start_end_events: false,
      ignored_events: None,
      thread_attribute: Some(thread_attribute),
    }
  }
}
