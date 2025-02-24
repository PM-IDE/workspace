use crate::test_core::gold_based_test::execute_test_with_gold;
use crate::test_core::simple_events_logs_provider::{create_simple_event_log, create_simple_event_log2, create_simple_event_log3};
use crate::test_core::test_paths::get_serialized_petri_nets_gold_path;
use ficus::event_log::xes::xes_event_log::XesEventLogImpl;
use ficus::features::analysis::log_info::event_log_info::OfflineEventLogInfo;
use ficus::features::analysis::log_info::log_info_creation_dto::EventLogInfoCreationDto;
use ficus::features::discovery::alpha::alpha::discover_petri_net_alpha;
use ficus::features::discovery::alpha::providers::alpha_provider::DefaultAlphaRelationsProvider;
use ficus::features::discovery::petri_net::pnml_serialization::serialize_to_pnml;

#[test]
pub fn alpha_simple_test_1() {
  execute_alpha_discovery_test("alpha_simple_test_1", || create_simple_event_log());
}

#[test]
pub fn alpha_simple_test_2() {
  execute_alpha_discovery_test("alpha_simple_test_2", || create_simple_event_log2());
}

#[test]
pub fn alpha_simple_test_3() {
  execute_alpha_discovery_test("alpha_simple_test_3", || create_simple_event_log3());
}

fn execute_alpha_discovery_test(test_name: &str, log_creator: impl Fn() -> XesEventLogImpl) {
  execute_test_with_gold(get_serialized_petri_nets_gold_path(test_name), || {
    let log = log_creator();
    let info = OfflineEventLogInfo::create_from(EventLogInfoCreationDto::default(&log));
    let provider = DefaultAlphaRelationsProvider::new(&info);

    serialize_to_pnml(&discover_petri_net_alpha(&provider), true).ok().unwrap()
  })
}
