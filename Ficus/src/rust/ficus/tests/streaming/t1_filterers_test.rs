use crate::test_core::simple_events_logs_provider::{annotate_log_with_real_time, create_simple_event_log2};
use chrono::{Duration, Utc};
use ficus::event_log::core::event_log::EventLog;
use ficus::event_log::xes::xes_event_log::XesEventLogImpl;
use ficus::grpc::kafka::streaming::t1::configs::{EventsTimeoutConfiguration, TracesQueueConfiguration, TracesTimeoutConfiguration};
use ficus::grpc::kafka::streaming::t1::filterers::{EventsTimeoutFiltererImpl, TracesQueueFiltererImpl, TracesTimeoutFiltererImpl};
use ficus::vecs;
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

#[test]
pub fn traces_queue_test() {
    execute_traces_queue_test(
        4,
        create_simple_event_log2(),
        vec![
            vecs!["B", "C", "E", "A", "A", "A"],
            vecs!["A", "E", "C", "B", "B", "B", "E", "A"],
            vecs!["A", "B", "C", "C", "A"],
            vecs!["B", "C", "E", "A", "A", "A"],
        ],
    );
}

fn execute_traces_queue_test(queue_capacity: u64, mut original_log: XesEventLogImpl, expected_log: Vec<Vec<String>>) {
    let filterer = TracesQueueFiltererImpl::new(TracesQueueConfiguration::new(queue_capacity));
    filterer.filter(&mut original_log);

    assert_eq!(original_log.to_raw_vector(), expected_log);
}

#[test]
pub fn test_() {
    execute_traces_queue_test(
        123123,
        create_simple_event_log2(),
        vec![
            vecs!["A", "B", "C", "D", "E"],
            vecs!["B", "C", "E", "A", "A", "A"],
            vecs!["A", "E", "C", "B", "B", "B", "E", "A"],
            vecs!["A", "B", "C", "C", "A"],
            vecs!["B", "C", "E", "A", "A", "A"],
        ],
    );
}
