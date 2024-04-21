use bxes::{
    read::multiple_files_bxes_reader::read_bxes_multiple_files,
    writer::multiple_file_bxes_writer::write_bxes_multiple_files,
};
use tempfile::TempDir;

use crate::test_core::random_log::generate_random_bxes_write_data;

#[test]
pub fn test_multiple_file_reader() {
    let temp_dir = TempDir::new().unwrap();
    let temp_dir_path = temp_dir.path().to_str().unwrap();
    let write_data = generate_random_bxes_write_data();
    write_bxes_multiple_files(&write_data, temp_dir_path).ok();

    let read_result = read_bxes_multiple_files(temp_dir_path).ok().unwrap();

    assert!(write_data.log.eq(&read_result.log));
    assert!(write_data.system_metadata.eq(&read_result.system_metadata));
}
