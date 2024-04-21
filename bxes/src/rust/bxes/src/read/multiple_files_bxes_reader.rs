use std::path::Path;

use crate::models::domain::bxes_event_log::BxesEventLog;
use crate::{
    binary_rw::core::{BinaryReader, Endian},
    constants::*,
};

use super::{errors::*, read_utils::*};

pub fn read_bxes_multiple_files(
    directory_path: &str,
) -> Result<BxesEventLogReadResult, BxesReadError> {
    let mut version = 0u32;

    let system_metadata = read_file(directory_path, SYSTEM_METADATA_FILE_NAME, |reader| {
        version = try_read_u32(reader)?;
        try_read_system_metadata(reader)
    })?;

    let values = read_file(directory_path, VALUES_FILE_NAME, |reader| {
        if let Some(error) = read_version(&mut version, reader) {
            return Err(error);
        }

        try_read_values(reader)
    })?;

    let kv_pairs = read_file(directory_path, KEY_VALUES_FILE_NAME, |reader| {
        if let Some(error) = read_version(&mut version, reader) {
            return Err(error);
        }

        try_read_key_values(reader)
    })?;

    let metadata = read_file(directory_path, METADATA_FILE_NAME, |reader| {
        if let Some(error) = read_version(&mut version, reader) {
            return Err(error);
        }

        try_read_event_log_metadata(reader, &values, &kv_pairs)
    })?;

    let variants = read_file(directory_path, VARIANTS_FILE_NAME, |reader| {
        if let Some(error) = read_version(&mut version, reader) {
            return Err(error);
        }

        try_read_traces_variants(reader, &values, &kv_pairs)
    })?;

    let log = BxesEventLog {
        version,
        metadata,
        variants,
    };

    Ok(BxesEventLogReadResult {
        log,
        system_metadata,
    })
}

fn read_version(previous_version: &mut u32, reader: &mut BinaryReader) -> Option<BxesReadError> {
    let current_version = try_read_u32(reader);
    if current_version.is_err() {
        return Some(current_version.err().unwrap());
    }

    let current_version = current_version.ok().unwrap();
    if *previous_version != current_version {
        Some(BxesReadError::VersionsMismatchError(
            VersionsMismatchError::new(*previous_version, current_version),
        ))
    } else {
        *previous_version = current_version;
        None
    }
}

fn read_file<T>(
    directory_path: &str,
    file_name: &str,
    reader_func: impl FnMut(&mut BinaryReader) -> Result<T, BxesReadError>,
) -> Result<T, BxesReadError> {
    let directory_path = Path::new(directory_path);
    let file_path = directory_path.join(file_name);
    let file_path = file_path.to_str().unwrap();

    execute_with_reader(file_path, reader_func)
}

fn execute_with_reader<T>(
    path: &str,
    mut reader_func: impl FnMut(&mut BinaryReader) -> Result<T, BxesReadError>,
) -> Result<T, BxesReadError> {
    let mut stream = try_open_file_stream(path)?;
    let mut reader = BinaryReader::new(&mut stream, Endian::Little);

    reader_func(&mut reader)
}
