use crate::event_log::{
  core::{event::event::Event, event_log::EventLog, trace::trace::Trace},
  xes::{xes_event::XesEventImpl, xes_event_log::XesEventLogImpl, xes_trace::XesTraceImpl},
};
use chrono::Utc;
use std::{cell::RefCell, rc::Rc};

pub fn create_simple_event_log(raw_log: &Vec<Vec<&str>>) -> XesEventLogImpl {
  let mut log = XesEventLogImpl::default();

  for raw_trace in raw_log {
    let mut trace = XesTraceImpl::default();
    for raw_event in raw_trace {
      trace.push(Rc::new(RefCell::new(XesEventImpl::new(
        Rc::from(raw_event.to_string()),
        Utc::now(),
      ))))
    }

    log.push(Rc::new(RefCell::new(trace)))
  }

  log
}
