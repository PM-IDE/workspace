use crate::event_log::core::event_log::EventLog;
use std::collections::HashSet;

pub struct EventLogInfoCreationDto<'a, TLog>
where
  TLog: EventLog,
{
  pub(super) log: &'a TLog,
  pub(super) add_fake_start_end_events: bool,
  pub(super) ignored_events: Option<&'a HashSet<String>>,
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
    }
  }

  pub fn default_fake_events(log: &'a TLog) -> Self {
    Self {
      log,
      add_fake_start_end_events: true,
      ignored_events: None,
    }
  }

  pub fn default_fake_ignored(log: &'a TLog, ignored_events: Option<&'a HashSet<String>>) -> Self {
    Self {
      log,
      add_fake_start_end_events: true,
      ignored_events,
    }
  }

  pub fn default_ignore(log: &'a TLog, ignored_events: &'a HashSet<String>) -> Self {
    Self {
      log,
      add_fake_start_end_events: false,
      ignored_events: Some(ignored_events),
    }
  }
}
