use std::fs;

use tempfile::TempDir;

use crate::binary_rw::core::{BinaryReader, Endian};
use crate::models::domain::bxes_event_log::BxesEventLog;
use crate::read::read_context::ReadContext;

use super::{errors::BxesReadError, read_utils::*};

pub fn read_bxes_from_archive_bytes(
    bytes: Vec<u8>,
) -> Result<BxesEventLogReadResult, BxesReadError> {
    let extracted_files_dir = try_extract_archive_bytes(bytes)?;
    read_bxes_internal(extracted_files_dir)
}

pub fn read_bxes(path: &str) -> Result<BxesEventLogReadResult, BxesReadError> {
    let extracted_files_dir = try_extract_archive(path)?;
    read_bxes_internal(extracted_files_dir)
}

fn read_bxes_internal(
    extracted_files_dir: TempDir,
) -> Result<BxesEventLogReadResult, BxesReadError> {
    let extracted_files_dir = extracted_files_dir.path();

    let files = fs::read_dir(extracted_files_dir)
        .unwrap()
        .into_iter()
        .map(|r| r.unwrap().path().to_str().unwrap().to_string())
        .collect::<Vec<String>>();

    if files.len() != 1 {
        return Err(BxesReadError::InvalidArchive(format!(
            "Expected one file, got {:?}",
            files
        )));
    }

    let mut stream = try_open_file_stream(files[0].as_str())?;
    let mut reader = BinaryReader::new(&mut stream, Endian::Little);
    let version = try_read_u32(&mut reader)?;

    let mut context = ReadContext::new(&mut reader);
    try_read_system_metadata(&mut context)?;

    try_read_values(&mut context)?;
    try_read_key_values(&mut context)?;

    let metadata = try_read_event_log_metadata(&mut context)?;
    let variants = try_read_traces_variants(&mut context)?;

    let log = BxesEventLog {
        version,
        metadata,
        variants,
    };

    Ok(BxesEventLogReadResult {
        log,
        system_metadata: context.system_metadata.unwrap(),
    })
}
