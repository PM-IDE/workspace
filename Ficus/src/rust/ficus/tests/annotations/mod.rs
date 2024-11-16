mod annotation_tests;

use std::{collections::HashMap, fmt::Debug};

use ficus::{
    event_log::core::event_log::EventLog,
    features::{
        analysis::event_log_info::{EventLogInfo, EventLogInfoCreationDto},
        discovery::{
            alpha::{
                alpha::{discover_petri_net_alpha, discover_petri_net_alpha_plus, find_transitions_one_length_loop},
                providers::{alpha_plus_provider::AlphaPlusRelationsProviderImpl, alpha_provider::DefaultAlphaRelationsProvider},
            },
            heuristic::heuristic_miner::discover_petri_net_heuristic,
            petri_net::{
                annotations::{annotate_with_counts, annotate_with_frequencies, annotate_with_trace_frequency},
                marking::ensure_initial_marking,
                petri_net::DefaultPetriNet,
                replay::replay_petri_net,
            },
        },
    },
    vecs,
};

use crate::test_core::simple_events_logs_provider::{
    create_alpha_plus_miner_replay_test_log, create_heuristic_miner_replay_test_log, create_simple_event_log,
};

#[test]
pub fn test_simple_count_annotation() {
    let log = create_simple_event_log();
    let log_info = EventLogInfo::create_from(EventLogInfoCreationDto::default(&log));
    let petri_net = discover_petri_net_alpha(&DefaultAlphaRelationsProvider::new(&log_info));
    let annotation = annotate_with_counts(&log, &petri_net, true).unwrap();

    execute_test_with_annotation(
        &petri_net,
        annotation,
        vec![
            ("({A}, {B})--A".to_owned(), 2),
            ("({A}, {B})--B".to_owned(), 2),
            ("({B}, {C})--B".to_owned(), 2),
            ("({B}, {C})--C".to_owned(), 2),
            ("EndPlace--C".to_owned(), 2),
            ("StartPlace--A".to_owned(), 2),
        ],
    );
}

pub fn execute_test_with_annotation<T>(net: &DefaultPetriNet, annotation: HashMap<u64, T>, mut expected: Vec<(String, T)>)
where
    T: ToString + PartialEq + Debug + Copy,
{
    let mut processed_annotations: Vec<(String, T)> = annotation
        .iter()
        .map(|pair| {
            if let Some((arc, transition)) = net.arc(pair.0) {
                let place = net.place(&arc.place_id());
                let name = format!("{}--{}", place.name(), transition.name());
                return (name, *pair.1);
            }

            panic!();
        })
        .collect();

    processed_annotations.sort_by(|first, second| first.0.cmp(&second.0));
    expected.sort_by(|first, second| first.0.cmp(&second.0));

    assert_eq!(processed_annotations, expected);
}

#[test]
pub fn test_simple_frequency_annotation() {
    let log = create_simple_event_log();
    let log_info = EventLogInfo::create_from(EventLogInfoCreationDto::default(&log));
    let petri_net = discover_petri_net_alpha(&DefaultAlphaRelationsProvider::new(&log_info));
    let annotation = annotate_with_frequencies(&log, &petri_net, true).unwrap();

    execute_test_with_annotation(
        &petri_net,
        annotation,
        vec![
            ("({A}, {B})--A".to_owned(), 0.16666666666666666),
            ("({A}, {B})--B".to_owned(), 0.16666666666666666),
            ("({B}, {C})--B".to_owned(), 0.16666666666666666),
            ("({B}, {C})--C".to_owned(), 0.16666666666666666),
            ("EndPlace--C".to_owned(), 0.16666666666666666),
            ("StartPlace--A".to_owned(), 0.16666666666666666),
        ],
    );
}

#[test]
pub fn test_simple_trace_frequency_annotation() {
    let log = create_simple_event_log();
    let log_info = EventLogInfo::create_from(EventLogInfoCreationDto::default(&log));
    let petri_net = discover_petri_net_alpha(&DefaultAlphaRelationsProvider::new(&log_info));
    let annotation = annotate_with_trace_frequency(&log, &petri_net, true).unwrap();

    execute_test_with_annotation(
        &petri_net,
        annotation,
        vec![
            ("({A}, {B})--A".to_owned(), 1.0),
            ("({A}, {B})--B".to_owned(), 1.0),
            ("({B}, {C})--B".to_owned(), 1.0),
            ("({B}, {C})--C".to_owned(), 1.0),
            ("EndPlace--C".to_owned(), 1.0),
            ("StartPlace--A".to_owned(), 1.0),
        ],
    );
}
