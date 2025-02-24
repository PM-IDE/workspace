use ficus::event_log::xes::reader::file_xes_log_reader::read_event_log;
use ficus::event_log::xes::writer::xes_event_log_writer::serialize_event_log;

use crate::test_core::gold_based_test::execute_test_with_gold;
use crate::test_core::test_paths::{create_example_log_gold_file_path, get_paths_to_example_logs};

#[test]
fn test_read_write_xes() {
  for log_path in get_paths_to_example_logs() {
    let log_name = log_path.file_name().unwrap().to_str().unwrap();
    execute_test_with_gold(create_example_log_gold_file_path(log_name), || {
      let event_log = read_event_log(log_path.to_str().unwrap()).unwrap();
      serialize_event_log(&event_log).ok().unwrap()
    });
  }
}
