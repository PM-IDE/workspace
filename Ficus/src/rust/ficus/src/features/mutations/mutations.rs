use crate::event_log::core::trace::trace::Trace;
use crate::event_log::core::{event::event::Event, event_log::EventLog};
use std::cell::RefCell;
use std::rc::Rc;

pub fn rename_events<TLog, TFilter>(log: &mut TLog, new_name: &str, filter: TFilter)
where
    TLog: EventLog,
    TFilter: Fn(&TLog::TEvent) -> bool,
{
    log.mutate_events(|event| {
        if filter(event) {
            event.set_name(new_name.to_owned())
        }
    })
}

pub const ARTIFICIAL_START_EVENT_NAME: &'static str = "ARTIFICIAL_START";
pub const ARTIFICIAL_END_EVENT_NAME: &'static str = "ARTIFICIAL_END";

pub fn add_artificial_start_end_activities<TLog>(log: &mut TLog, add_start_events: bool, add_end_events: bool)
where
    TLog: EventLog,
{
    for trace in log.traces() {
        let mut trace = trace.borrow_mut();
        let events = trace.events_mut();

        if add_start_events {
            let name = ARTIFICIAL_START_EVENT_NAME.to_string();
            let artificial_start_event = if events.is_empty() {
                TLog::TEvent::new_with_min_date(name)
            } else {
                TLog::TEvent::new(name, events.first().expect("!events.is_empty()").borrow().timestamp().clone())
            };

            events.insert(0, Rc::new(RefCell::new(artificial_start_event)));
        }

        if add_end_events {
            let name = ARTIFICIAL_END_EVENT_NAME.to_string();
            let artificial_end_event = if events.is_empty() {
                TLog::TEvent::new_with_max_date(name)
            } else {
                TLog::TEvent::new(name, events.last().expect("!events.is_empty()").borrow().timestamp().clone())
            };

            events.push(Rc::new(RefCell::new(artificial_end_event)));
        }
    }
}

pub fn append_attributes_to_name<TLog: EventLog>(log: &mut TLog, attributes: &Vec<String>) {
    log.mutate_events(|event| {
        let mut new_name = event.name().to_owned();
        let payload = event.payload_map();

        for attribute in attributes {
            let value = match payload {
                None => None,
                Some(payload) => match payload.get(attribute) {
                    None => None,
                    Some(value) => Some(value.to_string_repr()),
                },
            };

            let attribute_value_string = match value {
                None => "None".to_string(),
                Some(value) => value.as_str().to_owned(),
            };

            new_name += format!("_{}", attribute_value_string).as_str();
        }

        event.set_name(new_name);
    })
}
