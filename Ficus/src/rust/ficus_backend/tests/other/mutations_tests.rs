use std::{collections::HashSet, vec};

use ficus_backend::{
    event_log::core::{event::event::Event, event_log::EventLog},
    features::mutations::{
        filtering::{filter_log_by_name, filter_log_by_names},
        mutations::rename_events,
    },
};

use crate::test_core::simple_events_logs_provider::{
    create_raw_event_log2, create_simple_event_log, create_simple_event_log2, create_simple_event_log3,
};

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
