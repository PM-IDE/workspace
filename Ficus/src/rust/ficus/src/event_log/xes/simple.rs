use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::event_log::xes::xes_trace::XesTraceImpl;
use std::cell::RefCell;
use std::rc::Rc;

pub fn create_simple_event_log(raw_log: &Vec<Vec<&str>>) -> XesEventLogImpl {
    let mut log = XesEventLogImpl::empty();

    for raw_trace in raw_log {
        let mut trace = XesTraceImpl::empty();
        for raw_event in raw_trace {
            trace.push(Rc::new(RefCell::new(XesEventImpl::new_with_min_date(raw_event.to_string()))))
        }

        log.push(Rc::new(RefCell::new(trace)))
    }

    log
}
