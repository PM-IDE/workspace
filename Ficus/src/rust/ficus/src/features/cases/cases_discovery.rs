use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::event_log::xes::xes_trace::XesTraceImpl;
use fancy_regex::Regex;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn discover_cases(log: &XesEventLogImpl, start_regex_str: &str, end_regex_str: &str) -> XesEventLogImpl {
    let mut new_log = XesEventLogImpl::empty();

    let start_regex = Regex::new(start_regex_str).expect("Must create regex");
    let end_regex = Regex::new(end_regex_str).expect("Must create regex");

    for trace in log.traces() {
        let trace = trace.borrow();
        let mut stack = VecDeque::new();

        for event in trace.events() {
            let event = event.borrow();
            let event_name = event.name().as_str();
            if start_regex.is_match(event_name).expect("Regex") {
                let mut sub_trace = XesTraceImpl::empty();
                sub_trace.push(Rc::new(RefCell::new(event.clone())));

                stack.push_back(sub_trace);

                continue;
            }

            if end_regex.is_match(event_name).expect("Regex") {
                match stack.pop_back() {
                    None => {}
                    Some(mut sub_trace) => {
                        sub_trace.push(Rc::new(RefCell::new(event.clone())));
                        new_log.push(Rc::new(RefCell::new(sub_trace)));
                    }
                }

                continue;
            }

            if let Some(last_trace) = stack.back_mut() {
                last_trace.push(Rc::new(RefCell::new(event.clone())));
            }
        }

        loop {
            if stack.is_empty() {
                break;
            }

            new_log.push(Rc::new(RefCell::new(stack.pop_back().expect("Can not be empty"))));
        }
    }

    new_log
}