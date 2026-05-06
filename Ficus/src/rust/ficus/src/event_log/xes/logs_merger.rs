use crate::event_log::{
  core::event_log::EventLog,
  xes::{reader::file_xes_log_reader::read_event_log, xes_event_log::XesEventLogImpl},
};
use log::error;
use std::rc::Rc;
use std::sync::Arc;

pub fn merge_xes_logs(paths: &Vec<Arc<str>>) -> XesEventLogImpl {
  let mut merged_log = XesEventLogImpl::default();

  for path in paths {
    if let Some(log) = read_event_log(path) {
      for trace in log.traces() {
        merged_log.push(trace.clone());
      }
    } else {
      error!("Failed to read event log from {}", path);
    }
  }

  merged_log
}
