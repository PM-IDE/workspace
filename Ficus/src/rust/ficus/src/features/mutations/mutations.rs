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
            events.insert(
                0,
                Rc::new(RefCell::new(TLog::TEvent::new_with_min_date(
                    ARTIFICIAL_START_EVENT_NAME.to_string(),
                ))),
            );
        }

        if add_end_events {
            events.push(Rc::new(RefCell::new(TLog::TEvent::new_with_max_date(
                ARTIFICIAL_END_EVENT_NAME.to_string(),
            ))));
        }
    }
}
