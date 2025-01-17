use crate::test_core::simple_events_logs_provider::{
    create_alpha_plus_miner_replay_test_log, create_heuristic_miner_replay_test_log, create_simple_event_log,
};
use ficus::event_log::core::event_log::EventLog;
use ficus::features::analysis::event_log_info::{OfflineEventLogInfo, EventLogInfoCreationDto};
use ficus::features::discovery::alpha::alpha::{discover_petri_net_alpha, discover_petri_net_alpha_plus, find_transitions_one_length_loop};
use ficus::features::discovery::alpha::providers::alpha_plus_provider::AlphaPlusRelationsProviderImpl;
use ficus::features::discovery::alpha::providers::alpha_provider::DefaultAlphaRelationsProvider;
use ficus::features::discovery::heuristic::heuristic_miner::discover_petri_net_heuristic;
use ficus::features::discovery::petri_net::marking::ensure_initial_marking;
use ficus::features::discovery::petri_net::petri_net::DefaultPetriNet;
use ficus::features::discovery::petri_net::replay::replay_petri_net;
use ficus::vecs;

#[test]
pub fn test_simple_replay() {
    let log = create_simple_event_log();
    let log_info = OfflineEventLogInfo::create_from(EventLogInfoCreationDto::default(&log));
    let petri_net = discover_petri_net_alpha(&DefaultAlphaRelationsProvider::new(&log_info));

    let expected_transitions = vec![Some(vecs!["A", "B", "C"]), Some(vecs!["A", "B", "C"])];

    execute_test_with_replay(&petri_net, &log, expected_transitions);
}

#[test]
pub fn test_silent_transitions_replay() {
    let log = create_heuristic_miner_replay_test_log();
    let mut petri_net = discover_petri_net_heuristic(&log, 0.0, 0, 1.0, 0.1, 0.5);
    ensure_initial_marking(&log, &mut petri_net);

    let expected_transitions = vec![
        Some(vecs!["A", "silent_start_A", "B", "C", "D"]),
        Some(vecs!["A", "silent_start_A", "C", "B", "D"]),
    ];

    execute_test_with_replay(&petri_net, &log, expected_transitions);
}

#[test]
pub fn test_alpha_plus_log_replay() {
    let log = create_alpha_plus_miner_replay_test_log();

    let one_length_loop_transitions = find_transitions_one_length_loop(&log);
    let event_log_info = OfflineEventLogInfo::create_from(EventLogInfoCreationDto::default_ignore(&log, &one_length_loop_transitions));
    let provider = AlphaPlusRelationsProviderImpl::new(&event_log_info, &log, &one_length_loop_transitions);

    let petri_net = discover_petri_net_alpha_plus(&provider, false);

    let expected_transitions = vec![
        Some(vecs!["A", "B", "C", "D"]),
        Some(vecs!["A", "C", "B", "D"]),
        Some(vecs!["E", "F"]),
    ];

    execute_test_with_replay(&petri_net, &log, expected_transitions);
}

fn execute_test_with_replay(net: &DefaultPetriNet, log: &impl EventLog, expected_transitions: Vec<Option<Vec<String>>>) {
    let replay_states = replay_petri_net(log, net).unwrap();
    if replay_states.len() != expected_transitions.len() {
        panic!();
    }

    for (replay_state, expected_transitions) in replay_states.iter().zip(expected_transitions.iter()) {
        if replay_state.is_none() && expected_transitions.is_none() {
            continue;
        }

        if !(replay_state.is_some() && expected_transitions.is_some()) {
            panic!();
        }

        let expected = expected_transitions.as_ref().unwrap();
        let state = replay_state.as_ref().unwrap();

        let replayed_transitions: Vec<String> = state
            .fired_transitions()
            .iter()
            .map(|id| net.transition(id).name().to_owned())
            .collect();

        assert_eq!(&replayed_transitions, expected);
    }
}
