use crate::test_core::simple_events_logs_provider::{annotate_log_with_real_time, create_simple_event_log2};
use chrono::{Duration, Utc};
use ficus::event_log::core::event_log::EventLog;
use ficus::grpc::kafka::streaming::t1::configs::{EventsTimeoutConfiguration, TracesTimeoutConfiguration};
use ficus::grpc::kafka::streaming::t1::filterers::{EventsTimeoutFiltererImpl, TracesTimeoutFiltererImpl};
use std::ops::Sub;

#[test]
pub fn events_filterer_test() {
    let filterer = EventsTimeoutFiltererImpl::new(EventsTimeoutConfiguration::new(500 * 1000));
    let mut log = create_simple_event_log2();
    annotate_log_with_real_time(&mut log, Utc::now().sub(Duration::seconds(1000)), Duration::seconds(100));
    filterer.filter(&mut log);

    assert_eq!(log.to_raw_vector(), vec![vec!["A"], vec!["B", "E", "A"], vec!["A"]]);
}

#[test]
pub fn traces_filterer_test() {
    let filterer = TracesTimeoutFiltererImpl::new(TracesTimeoutConfiguration::new(500 * 1000));
    let mut log = create_simple_event_log2();
    annotate_log_with_real_time(&mut log, Utc::now().sub(Duration::seconds(1000)), Duration::seconds(100));
    filterer.filter(&mut log);

    assert_eq!(
        log.to_raw_vector(),
        vec![
            vec!["B", "C", "E", "A", "A", "A"],
            vec!["A", "E", "C", "B", "B", "B", "E", "A"],
            vec!["B", "C", "E", "A", "A", "A"]
        ]
    );
}
