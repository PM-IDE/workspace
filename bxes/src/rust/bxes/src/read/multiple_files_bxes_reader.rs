use std::path::Path;

use crate::models::domain::bxes_event_log::{BxesEventLog, BxesTraceVariant};
use crate::{
    binary_rw::core::{BinaryReader, Endian},
    constants::*,
};
use crate::models::domain::bxes_log_metadata::BxesEventLogMetadata;
use crate::models::system_models::SystemMetadata;
use crate::read::read_context::ReadContext;
use crate::utils::buffered_stream::BufferedReadFileStream;

use super::{errors::*, read_utils::*};

pub fn read_bxes_multiple_files(
    directory_path: &str,
) -> Result<BxesEventLogReadResult, BxesReadError> {
    let mut context = ReadContext::new_without_reader();

    let mut stream = open_file(directory_path, SYSTEM_METADATA_FILE_NAME)?;
    let mut reader = BinaryReader::new(&mut stream, Endian::Little);
    context.set_reader(&mut reader);
    let (mut version, system_metadata) = read_system_metadata(&mut context)?;

    let mut stream = open_file(directory_path, VALUES_FILE_NAME)?;
    let mut reader = BinaryReader::new(&mut stream, Endian::Little);
    context.set_reader(&mut reader);
    read_values(&mut context, &mut version)?;

    let mut stream = open_file(directory_path, KEY_VALUES_FILE_NAME)?;
    let mut reader = BinaryReader::new(&mut stream, Endian::Little);
    context.set_reader(&mut reader);
    read_key_values(&mut context, &mut version)?;

    let mut stream = open_file(directory_path, METADATA_FILE_NAME)?;
    let mut reader = BinaryReader::new(&mut stream, Endian::Little);
    context.set_reader(&mut reader);
    let metadata = read_metadata_file(&mut context, &mut version)?;

    let mut stream = open_file(directory_path, VARIANTS_FILE_NAME)?;
    let mut reader = BinaryReader::new(&mut stream, Endian::Little);
    context.set_reader(&mut reader);
    let variants = read_variants(&mut context, &mut version)?;

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

fn read_system_metadata(context: &mut ReadContext) -> Result<(u32, SystemMetadata), BxesReadError> {
    let version = try_read_u32(context.reader.as_mut().unwrap())?;
    let metadata = try_read_system_metadata(context)?;

    Ok((version, metadata))
}

fn read_values(context: &mut ReadContext, version: &mut u32) -> Result<(), BxesReadError> {
    if let Some(error) = read_version(version, context.reader.as_mut().unwrap()) {
        return Err(error);
    }

    try_read_values(context)
}

fn read_key_values(context: &mut ReadContext, version: &mut u32) -> Result<(), BxesReadError> {
    if let Some(error) = read_version(version, context.reader.as_mut().unwrap()) {
        return Err(error);
    }

    try_read_key_values(context)
}

fn read_metadata_file(context: &mut ReadContext, version: &mut u32) -> Result<BxesEventLogMetadata, BxesReadError> {
    if let Some(error) = read_version(version, context.reader.as_mut().unwrap()) {
        return Err(error);
    }

    try_read_event_log_metadata(context)
}

fn read_variants(context: &mut ReadContext, version: &mut u32) -> Result<Vec<BxesTraceVariant>, BxesReadError> {
    if let Some(error) = read_version(version, context.reader.as_mut().unwrap()) {
        return Err(error);
    }

    try_read_traces_variants(context)
}

fn open_file(directory_path: &str, file_name: &str,) -> Result<BufferedReadFileStream, BxesReadError> {
    let directory_path = Path::new(directory_path);
    let file_path = directory_path.join(file_name);
    let file_path = file_path.to_str().unwrap();
    try_open_file_stream(file_path)
}