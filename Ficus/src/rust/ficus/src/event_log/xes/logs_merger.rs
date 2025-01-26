use log::error;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::xes::reader::file_xes_log_reader::read_event_log;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;

pub fn merge_xes_logs(paths: &Vec<String>) -> XesEventLogImpl {
    let mut merged_log = XesEventLogImpl::empty();

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
