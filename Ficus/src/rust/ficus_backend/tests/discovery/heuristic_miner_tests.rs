use ficus_backend::{
    event_log::simple::simple_event_log::SimpleEventLog,
    features::discovery::{heuristic::heuristic_miner::discover_petri_net_heuristic, petri_net::pnml_serialization::serialize_to_pnml},
};

use crate::test_core::{
    gold_based_test::execute_test_with_gold, simple_events_logs_provider::*, test_paths::get_serialized_petri_nets_gold_path,
};

#[test]
pub fn heuristic_miner_test_2() {
    execute_heuristic_miner_discovery_test("heuristic_miner_test_2", || create_alpha_plus_plus_nfc_test_log2());
}

#[test]
pub fn heuristic_miner_test_3() {
    execute_heuristic_miner_discovery_test("heuristic_miner_test_3", || create_alpha_plus_plus_nfc_test_log3());
}

#[test]
pub fn heuristic_miner_test_4() {
    execute_heuristic_miner_discovery_test("heuristic_miner_test_4", || create_alpha_plus_plus_nfc_test_log4());
}

#[test]
pub fn heuristic_miner_test_5() {
    execute_heuristic_miner_discovery_test("heuristic_miner_test_5", || create_alpha_plus_plus_nfc_test_log5());
}

#[test]
pub fn heuristic_miner_test_6() {
    execute_heuristic_miner_discovery_test("heuristic_miner_test_6", || create_alpha_plus_plus_nfc_test_log6());
}

#[test]
pub fn heuristic_miner_test_7() {
    execute_heuristic_miner_discovery_test("heuristic_miner_test_7", || create_alpha_plus_plus_nfc_test_log7());
}

#[test]
pub fn heuristic_miner_test_8() {
    execute_heuristic_miner_discovery_test("heuristic_miner_test_8", || create_alpha_plus_plus_nfc_test_log8());
}

#[test]
pub fn heuristic_miner_test_9() {
    execute_heuristic_miner_discovery_test("heuristic_miner_test_9", || create_alpha_plus_plus_nfc_test_log9());
}

fn execute_heuristic_miner_discovery_test(test_name: &str, log_creator: impl Fn() -> SimpleEventLog) {
    execute_test_with_gold(get_serialized_petri_nets_gold_path(test_name), || {
        let log = log_creator();
        serialize_to_pnml(&discover_petri_net_heuristic(&log, 0.2, 1, 1.0, 0.1, 0.5), true)
            .ok()
            .unwrap()
    })
}
