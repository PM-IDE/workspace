use ficus::features::analysis::event_log_info::{EventLogInfo, EventLogInfoCreationDto, OfflineEventLogInfo};

use crate::test_core::simple_events_logs_provider::create_simple_event_log;

#[test]
fn test_event_log_info() {
    let log = create_simple_event_log();
    let creation_dto = EventLogInfoCreationDto::default(&log);
    let log_info = OfflineEventLogInfo::create_from(creation_dto);
    assert_eq!(log_info.events_count(), 6);

    assert_eq!(log_info.event_count(&"A".to_string()), 2usize);
    assert_eq!(log_info.event_count(&"B".to_string()), 2usize);
    assert_eq!(log_info.event_count(&"C".to_string()), 2usize);
}
