use crate::test_core::simple_events_logs_provider::create_cases_discovery_test_log;
use ficus::event_log::core::event_log::EventLog;
use ficus::features::cases::cases_discovery::discover_cases;

#[test]
fn test_simple_cases_discovery() {
    let log = create_cases_discovery_test_log();
    let cases_log = discover_cases(&log, "S", "E", false);

    assert_eq!(
        cases_log.to_raw_vector(),
        vec![
            vec!["S", "b", "a", "d", "E"],
            vec!["S", "E"],
            vec!["S"],
            vec!["S", "E"],
            vec!["S", "a", "b", "a", "E"],
            vec!["S"]
        ]
    );
}

#[test]
fn test_simple_cases_discovery_inline() {
    let log = create_cases_discovery_test_log();
    let cases_log = discover_cases(&log, "S", "E", true);

    assert_eq!(
        cases_log.to_raw_vector(),
        vec![
            vec!["S", "b", "a", "d", "E"],
            vec!["S", "E"],
            vec!["S"],
            vec!["S", "a", "b", "S", "E", "a", "E"],
            vec!["S"]
        ]
    );
}
