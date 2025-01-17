use std::collections::{HashMap, HashSet};

use ficus::features::analysis::{
    entropy::dfg_entropy::{calculate_default_dfg_entropy, calculate_laplace_dfg_entropy},
    entropy::{pos_entropy::calculate_pos_entropies, pos_entropy_fast::calculate_pos_entropies_fast},
    event_log_info::{OfflineEventLogInfo, EventLogInfoCreationDto},
};
use ficus::features::analysis::event_log_info::EventLogInfo;
use crate::test_core::simple_events_logs_provider::{
    create_log_from_filter_out_chaotic_events, create_log_from_filter_out_chaotic_events_with_noise, create_simple_event_log,
};

#[test]
fn test_dfg_info() {
    let log = create_simple_event_log();
    let creation_dto = EventLogInfoCreationDto::default(&log);
    let log_info = OfflineEventLogInfo::create_from(creation_dto);
    let dfg = log_info.dfg_info();

    assert_eq!(dfg.get_directly_follows_count(&"A".to_string(), &"B".to_string()), 2);
    assert_eq!(dfg.get_directly_follows_count(&"B".to_string(), &"C".to_string()), 2);
    assert_eq!(dfg.get_directly_follows_count(&"A".to_string(), &"C".to_string()), 0);
    assert_eq!(dfg.get_directly_follows_count(&"C".to_string(), &"B".to_string()), 0);
    assert_eq!(dfg.get_directly_follows_count(&"B".to_string(), &"A".to_string()), 0);

    assert!(dfg.is_event_with_single_follower(&"A".to_string()));
    assert!(dfg.is_event_with_single_follower(&"B".to_string()));
    assert!(!dfg.is_event_with_single_follower(&"C".to_string()));

    let followers = dfg.get_followed_events(&"A".to_string()).unwrap();
    assert_eq!(followers.get(&"B".to_string()).unwrap(), &2usize);

    let followers = dfg.get_followed_events(&"B".to_string()).unwrap();
    assert_eq!(followers.get(&"C".to_string()).unwrap(), &2usize);

    assert_eq!(dfg.get_followed_events(&"C".to_string()), None);
}

#[test]
fn test_dfg_entropy() {
    let log = create_log_from_filter_out_chaotic_events();
    let entropies = calculate_default_dfg_entropy(&log, None);
    let expected = HashMap::from_iter(vec![
        ("c".to_string(), 1.8365916681089791),
        ("b".to_string(), 1.8365916681089791),
        ("x".to_string(), 3.169925001442312),
        ("a".to_string(), 0.9182958340544896),
    ]);

    assert_eq!(entropies, expected);
}

#[test]
fn test_dfg_entropy_with_noise() {
    let log = create_log_from_filter_out_chaotic_events_with_noise();
    let ignored_events = HashSet::from_iter(vec!["d".to_string(), "v".to_string()]);

    let entropies = calculate_default_dfg_entropy(&log, Some(&ignored_events));
    let expected = HashMap::from_iter(vec![
        ("c".to_string(), 1.8365916681089791),
        ("b".to_string(), 1.8365916681089791),
        ("x".to_string(), 3.169925001442312),
        ("a".to_string(), 0.9182958340544896),
    ]);

    assert_eq!(entropies, expected);
}

#[test]
fn test_dfg_laplace_entropy() {
    let log = create_log_from_filter_out_chaotic_events();
    let entropies = calculate_laplace_dfg_entropy(&log, None);
    let expected = HashMap::from_iter(vec![
        ("c".to_string(), 1.905904975406124),
        ("b".to_string(), 1.905904975406124),
        ("x".to_string(), 3.2127002996007796),
        ("a".to_string(), 1.002726083454847),
    ]);

    assert_eq!(entropies, expected);
}

#[test]
fn test_dfg_laplace_entropy_with_noise() {
    let log = create_log_from_filter_out_chaotic_events_with_noise();
    let ignored_events = HashSet::from_iter(vec!["d".to_string(), "v".to_string()]);

    let entropies = calculate_laplace_dfg_entropy(&log, Some(&ignored_events));

    let expected = HashMap::from_iter(vec![
        ("c".to_string(), 1.905904975406124),
        ("b".to_string(), 1.905904975406124),
        ("x".to_string(), 3.2127002996007796),
        ("a".to_string(), 1.002726083454847),
    ]);

    assert_eq!(entropies, expected);
}

#[test]
fn test_pos_entropy() {
    let log = create_log_from_filter_out_chaotic_events();
    let entropies = calculate_pos_entropies(&log, &None);
    let expected = HashMap::from_iter(vec![
        ("c".to_string(), 0.2211099839259014),
        ("b".to_string(), 0.2211099839259014),
        ("x".to_string(), 0.3230075074711545),
        ("a".to_string(), 0.0),
    ]);

    assert_eq!(entropies, expected);
}

#[test]
fn test_pos_entropy_with_noise() {
    let log = create_log_from_filter_out_chaotic_events_with_noise();
    let ignored_events = HashSet::from_iter(vec!["d".to_string(), "v".to_string()]);

    let entropies = calculate_pos_entropies(&log, &Some(ignored_events));
    let expected = HashMap::from_iter(vec![
        ("c".to_string(), 0.2211099839259014),
        ("b".to_string(), 0.2211099839259014),
        ("x".to_string(), 0.3230075074711545),
        ("a".to_string(), 0.0),
    ]);

    assert_eq!(entropies, expected);
}

#[test]
fn test_pos_entropy_fast() {
    let log = create_log_from_filter_out_chaotic_events();
    let entropies = calculate_pos_entropies_fast(&log, None);
    let expected = HashMap::from_iter(vec![
        ("c".to_string(), 0.2211099839259014),
        ("b".to_string(), 0.2211099839259014),
        ("x".to_string(), 0.3230075074711545),
        ("a".to_string(), 0.0),
    ]);

    assert_eq!(entropies, expected);
}

#[test]
fn test_pos_entropy_fast_with_noise() {
    let log = create_log_from_filter_out_chaotic_events_with_noise();
    let ignored_events = HashSet::from_iter(vec!["d".to_string(), "v".to_string()]);

    let entropies = calculate_pos_entropies_fast(&log, Some(&ignored_events));
    let expected = HashMap::from_iter(vec![
        ("c".to_string(), 0.2211099839259014),
        ("b".to_string(), 0.2211099839259014),
        ("x".to_string(), 0.3230075074711545),
        ("a".to_string(), 0.0),
    ]);

    assert_eq!(entropies, expected);
}
