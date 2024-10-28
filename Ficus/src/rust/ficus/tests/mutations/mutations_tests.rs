use crate::test_core::simple_events_logs_provider::{
    create_raw_event_log2, create_simple_event_log, create_simple_event_log2, create_simple_event_log3,
};
use chrono::{DateTime, Utc};
use ficus::event_log::core::trace::trace::Trace;
use ficus::event_log::xes::xes_event_log::XesEventLogImpl;
use ficus::event_log::xes::xes_trace::XesTraceImpl;
use ficus::features::mutations::mutations::{add_artificial_start_end_activities, ARTIFICIAL_END_EVENT_NAME, ARTIFICIAL_START_EVENT_NAME};
use ficus::{
    event_log::core::{event::event::Event, event_log::EventLog},
    features::mutations::{
        filtering::{filter_log_by_name, filter_log_by_names},
        mutations::rename_events,
    },
};
use std::cell::RefCell;
use std::rc::Rc;
use std::{collections::HashSet, vec};

#[test]
fn test_removing_events() {
    let mut log = create_simple_event_log();
    log.filter_events_by(|event| event.name() == "A");

    assert_eq!(log.to_raw_vector(), vec![vec!["B", "C"], vec!["B", "C"]]);
}

#[test]
fn test_removing_events2() {
    let mut log = create_simple_event_log();
    log.filter_events_by(|event| event.name() == "B" || event.name() == "C");

    assert_eq!(log.to_raw_vector(), vec![vec!["A"], vec!["A"]]);
}

#[test]
fn test_removing_events3() {
    let mut log = create_simple_event_log();
    filter_log_by_name(&mut log, "A");

    assert_eq!(log.to_raw_vector(), vec![vec!["B", "C"], vec!["B", "C"]]);
}

#[test]
fn test_removing_events4() {
    let mut log = create_simple_event_log();
    let set = HashSet::from_iter(vec!["A".to_string(), "B".to_string()]);
    filter_log_by_names(&mut log, &set);

    assert_eq!(log.to_raw_vector(), vec![vec!["C"], vec!["C"]]);
}

#[test]
fn test_removing_events5() {
    let mut log = create_simple_event_log();
    let set = HashSet::from_iter(vec!["A".to_string(), "B".to_string(), "C".to_string()]);
    filter_log_by_names(&mut log, &set);

    assert!(log.to_raw_vector().is_empty());
}

#[test]
fn test_renaming() {
    let mut log = create_simple_event_log();
    rename_events(&mut log, "D", |event| event.name() == "A");

    assert_eq!(log.to_raw_vector(), vec![vec!["D", "B", "C"], vec!["D", "B", "C"]])
}

#[test]
fn test_renaming2() {
    let mut log = create_simple_event_log2();
    rename_events(&mut log, "D", |_| true);

    assert_eq!(
        log.to_raw_vector(),
        vec![
            vec!["D", "D", "D", "D", "D"],
            vec!["D", "D", "D", "D", "D", "D"],
            vec!["D", "D", "D", "D", "D", "D", "D", "D"],
            vec!["D", "D", "D", "D", "D"],
            vec!["D", "D", "D", "D", "D", "D"],
        ]
    );
}

#[test]
fn test_renaming2_no_change() {
    let mut log = create_simple_event_log2();
    rename_events(&mut log, "D", |_| false);

    assert_eq!(log.to_raw_vector(), create_raw_event_log2());
}

#[test]
fn test_renaming3() {
    let mut log = create_simple_event_log3();
    rename_events(&mut log, "D", |event| event.name() == "E");

    assert_eq!(
        log.to_raw_vector(),
        vec![
            vec!["A", "B", "C", "D", "D"],
            vec!["B", "C", "D", "A", "A", "A"],
            vec!["A", "D", "C", "B", "B", "B", "D", "A"],
            vec!["A", "B", "C", "C", "A"],
            vec!["B", "C", "D", "A", "A", "A"],
            vec!["A", "B", "C", "D", "D"],
            vec!["A", "B", "C", "C", "A"],
            vec!["A", "B", "C", "C", "A"],
            vec!["A", "D", "C", "B", "B", "B", "D", "A"],
        ]
    );
}

#[test]
fn test_add_artificial_start_event() {
    let mut log = create_simple_event_log3();
    add_artificial_start_end_activities(&mut log, true, false);

    assert_start_artificial_events(&log);
}

fn assert_start_artificial_events(log: &XesEventLogImpl) {
    for trace in log.traces() {
        let trace = trace.borrow();

        let first_event = trace.events().first().expect("Trace must be non empty");
        let second_event = trace.events().get(1).expect("Trace must contain at least two elements");

        assert_eq!(first_event.borrow().name(), ARTIFICIAL_START_EVENT_NAME);
        assert_eq!(first_event.borrow().timestamp(), second_event.borrow().timestamp());
    }
}

fn assert_end_artificial_events(log: &XesEventLogImpl) {
    for trace in log.traces() {
        let trace = trace.borrow();

        let first_event = trace.events().last().expect("Trace must be non empty");
        let second_event = trace
            .events()
            .get(trace.events().len() - 2)
            .expect("Trace must contain at least two elements");

        assert_eq!(first_event.borrow().name(), ARTIFICIAL_END_EVENT_NAME);
        assert_eq!(first_event.borrow().timestamp(), second_event.borrow().timestamp());
    }
}

#[test]
fn test_add_artificial_end_event() {
    let mut log = create_simple_event_log3();
    add_artificial_start_end_activities(&mut log, false, true);

    assert_end_artificial_events(&log);
}

#[test]
fn test_add_artificial_start_end_events() {
    let mut log = create_simple_event_log3();
    add_artificial_start_end_activities(&mut log, true, true);

    assert_start_artificial_events(&log);
    assert_end_artificial_events(&log);
}

#[test]
fn test_add_artificial_start_events_empty_trace() {
    let mut log = create_empty_log_with_empty_trace();

    add_artificial_start_end_activities(&mut log, true, false);

    let first_trace = log.traces().first().expect("Event log must contain log").borrow();
    let event = first_trace.events().first().expect("Trace must contain event");

    assert_eq!(event.borrow().name(), ARTIFICIAL_START_EVENT_NAME);
    assert_eq!(*event.borrow().timestamp(), DateTime::<Utc>::MIN_UTC);
}

fn create_empty_log_with_empty_trace() -> XesEventLogImpl {
    let mut log = XesEventLogImpl::empty();
    log.push(Rc::new(RefCell::new(XesTraceImpl::empty())));

    log
}

#[test]
fn test_add_artificial_end_events_empty_trace() {
    let mut log = create_empty_log_with_empty_trace();

    add_artificial_start_end_activities(&mut log, false, true);

    let first_trace = log.traces().first().expect("Event log must contain log").borrow();
    let event = first_trace.events().first().expect("Trace must contain event");

    assert_eq!(event.borrow().name(), ARTIFICIAL_END_EVENT_NAME);
    assert_eq!(*event.borrow().timestamp(), DateTime::<Utc>::MAX_UTC);
}

#[test]
fn test_add_artificial_start_end_events_empty_trace() {
    let mut log = create_empty_log_with_empty_trace();

    add_artificial_start_end_activities(&mut log, true, true);

    assert_start_artificial_events(&log);
    assert_end_artificial_events(&log);
}
