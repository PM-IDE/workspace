use chrono::Utc;
use core::fmt::Debug;
use ficus_backend::event_log::{
    core::{event::event::Event, event_log::EventLog, trace::trace::Trace},
};
use ficus_backend::event_log::xes::xes_event::XesEventImpl;
use ficus_backend::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::test_core::simple_events_logs_provider::{create_raw_event_log, create_simple_event_log};

#[test]
fn test_simple_event_log_creation() {
    let raw_log = create_raw_event_log();
    let simple_event_log = ficus_backend::event_log::xes::simple::create_simple_event_log(&raw_log);
    assert_eq!(raw_log, simple_event_log.to_raw_vector())
}

#[test]
fn test_set_name() {
    let log = create_simple_event_log();
    let value = String::from_utf8("ASDASD".into()).ok().unwrap();

    execute_test_set_test(&log, &value, |event| event.name(), |event, value| event.set_name(value.to_owned()))
}

#[test]
fn test_set_date() {
    let log = create_simple_event_log();
    let value = Utc::now();

    execute_test_set_test(&log, &value, |event| event.timestamp(), |event, value| event.set_timestamp(*value))
}

fn execute_test_set_test<TValue, TGet, TSet>(log: &XesEventLogImpl, value: &TValue, get_property: TGet, set_property: TSet)
where
    TGet: Fn(&XesEventImpl) -> &TValue,
    TSet: Fn(&mut XesEventImpl, &TValue) -> (),
    TValue: PartialEq + Debug,
{
    for trace in log.traces() {
        for event in trace.borrow().events() {
            set_property(&mut event.borrow_mut(), &value);
        }
    }

    for trace in log.traces() {
        for event in trace.borrow().events() {
            let event = &event.borrow();
            assert_eq!(get_property(event), value);
        }
    }
}
