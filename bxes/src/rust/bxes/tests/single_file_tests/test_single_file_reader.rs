use std::path::Path;
use bxes::{read::single_file_bxes_reader::read_bxes, writer::single_file_bxes_writer::write_bxes};
use tempfile::TempDir;

use crate::test_core::random_log::generate_random_bxes_write_data;

#[test]
pub fn test_single_file_read_write() {
    let temp_dir = TempDir::new().unwrap();
    let log_file_name = "log.bxes";
    let temp_dir = temp_dir.path().as_os_str().to_str().unwrap();
    let log_save_path = Path::new(temp_dir).join(log_file_name);

    let data = generate_random_bxes_write_data();
    println!("{:?}", write_bxes(log_save_path.to_str().unwrap(), &data));

    let read_result = read_bxes(log_save_path.to_str().unwrap()).unwrap();
    assert!(read_result.log.eq(&data.log));
    assert!(read_result.system_metadata.eq(&data.system_metadata));
}
