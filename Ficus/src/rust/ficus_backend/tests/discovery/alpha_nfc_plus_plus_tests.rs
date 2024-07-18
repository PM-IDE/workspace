use crate::test_core::{
    gold_based_test::execute_test_with_gold, simple_events_logs_provider::*, test_paths::get_serialized_petri_nets_gold_path,
};
use ficus_backend::event_log::xes::xes_event_log::XesEventLogImpl;
use ficus_backend::features::discovery::{
    alpha::alpha_plus_plus_nfc::alpha_plus_plus_nfc::discover_petri_net_alpha_plus_plus_nfc,
    petri_net::pnml_serialization::serialize_to_pnml,
};

#[test]
pub fn alpha_plus_plus_nfc_test_1() {
    execute_alpha_plus_plus_nfc_discovery_test("alpha_plus_plus_nfc_test_1", || create_alpha_plus_plus_nfc_test_log());
}

#[test]
pub fn alpha_plus_plus_nfc_test_2() {
    execute_alpha_plus_plus_nfc_discovery_test("alpha_plus_plus_nfc_test_2", || create_alpha_plus_plus_nfc_test_log2());
}

#[test]
pub fn alpha_plus_plus_nfc_test_3() {
    execute_alpha_plus_plus_nfc_discovery_test("alpha_plus_plus_nfc_test_3", || create_alpha_plus_plus_nfc_test_log3());
}

#[test]
pub fn alpha_plus_plus_nfc_test_4() {
    execute_alpha_plus_plus_nfc_discovery_test("alpha_plus_plus_nfc_test_4", || create_alpha_plus_plus_nfc_test_log4());
}

#[test]
pub fn alpha_plus_plus_nfc_test_5() {
    execute_alpha_plus_plus_nfc_discovery_test("alpha_plus_plus_nfc_test_5", || create_alpha_plus_plus_nfc_test_log5());
}

#[test]
pub fn alpha_plus_plus_nfc_test_6() {
    execute_alpha_plus_plus_nfc_discovery_test("alpha_plus_plus_nfc_test_6", || create_alpha_plus_plus_nfc_test_log6());
}

#[test]
pub fn alpha_plus_plus_nfc_test_7() {
    execute_alpha_plus_plus_nfc_discovery_test("alpha_plus_plus_nfc_test_7", || create_alpha_plus_plus_nfc_test_log7());
}

#[test]
pub fn alpha_plus_plus_nfc_test_8() {
    execute_alpha_plus_plus_nfc_discovery_test("alpha_plus_plus_nfc_test_8", || create_alpha_plus_plus_nfc_test_log8());
}

#[test]
pub fn alpha_plus_plus_nfc_test_9() {
    execute_alpha_plus_plus_nfc_discovery_test("alpha_plus_plus_nfc_test_9", || create_alpha_plus_plus_nfc_test_log9());
}

fn execute_alpha_plus_plus_nfc_discovery_test(test_name: &str, log_creator: impl Fn() -> XesEventLogImpl) {
    execute_test_with_gold(get_serialized_petri_nets_gold_path(test_name), || {
        let log = log_creator();
        serialize_to_pnml(&discover_petri_net_alpha_plus_plus_nfc(&log), true).ok().unwrap()
    })
}
