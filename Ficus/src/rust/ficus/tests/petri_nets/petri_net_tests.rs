use crate::test_core::gold_based_test::execute_test_with_gold;
use crate::test_core::simple_events_logs_provider::create_simple_event_log3;
use crate::test_core::test_paths::get_serialized_petri_nets_gold_path;
use ficus::features::analysis::log_info::event_log_info::OfflineEventLogInfo;
use ficus::features::analysis::log_info::log_info_creation_dto::EventLogInfoCreationDto;
use ficus::features::discovery::alpha::alpha::discover_petri_net_alpha;
use ficus::features::discovery::alpha::providers::alpha_provider::DefaultAlphaRelationsProvider;
use ficus::features::discovery::petri_net::pnml_serialization::serialize_to_pnml;

#[test]
#[rustfmt::skip]
pub fn test_serialization_1() {
    execute_test_with_gold(
        get_serialized_petri_nets_gold_path(stringify!(test_serialization_1)), || {
            let log = create_simple_event_log3();
            let info = OfflineEventLogInfo::create_from(EventLogInfoCreationDto::default(&log));
            let provider = DefaultAlphaRelationsProvider::new(&info);

            let petri_net = discover_petri_net_alpha(&provider);
            serialize_to_pnml(&petri_net, true).ok().unwrap()
        },
    )
}
