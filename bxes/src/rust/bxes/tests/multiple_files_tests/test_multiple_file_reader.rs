use bxes::{
    read::multiple_files_bxes_reader::read_bxes_multiple_files,
    writer::multiple_file_bxes_writer::write_bxes_multiple_files,
};
use tempfile::TempDir;

use crate::test_core::random_log::generate_random_log;

#[test]
pub fn test_multiple_file_reader() {
    let log = generate_random_log();
    let temp_dir = TempDir::new().unwrap();
    let temp_dir_path = temp_dir.path().to_str().unwrap();
    write_bxes_multiple_files(&log, temp_dir_path).ok();

    let read_log = read_bxes_multiple_files(temp_dir_path).ok().unwrap();

    assert!(log.eq(&read_log));
}
