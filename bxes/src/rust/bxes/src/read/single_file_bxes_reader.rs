use std::fs;

use super::{errors::BxesReadError, read_utils::*};
use crate::{
    binary_rw::core::{BinaryReader, Endian},
    models::*,
};

pub fn read_bxes(path: &str) -> Result<BxesEventLog, BxesReadError> {
    let extracted_files_dir = try_extract_archive(path)?;
    let extracted_files_dir = extracted_files_dir.path();

    let files = fs::read_dir(extracted_files_dir)
        .unwrap()
        .into_iter()
        .map(|r| r.unwrap().path().to_str().unwrap().to_string())
        .collect::<Vec<String>>();

    if files.len() != 1 {
        return Err(BxesReadError::InvalidArchive(format!("Expected one file, got {:?}", files)));
    }

    let mut stream = try_open_file_stream(files[0].as_str())?;
    let mut reader = BinaryReader::new(&mut stream, Endian::Little);
    let version = try_read_u32(&mut reader)?;

    let values = try_read_values(&mut reader)?;
    let kv_pairs = try_read_key_values(&mut reader)?;
    let metadata = try_read_event_log_metadata(&mut reader, &values, &kv_pairs)?;
    let variants = try_read_traces_variants(&mut reader, &values, &kv_pairs)?;

    Ok(BxesEventLog {
        version,
        metadata,
        variants,
    })
}
