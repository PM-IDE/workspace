use std::{cell::RefCell, rc::Rc};
use std::fs::File;
use std::io::Read;
use tempfile::NamedTempFile;

use crate::binary_rw::core::{BinaryWriter, Endian};
use crate::writer::writer_utils::{try_write_system_metadata, BxesLogWriteData};

use super::{
    errors::BxesWriteError,
    write_context::BxesWriteContext,
    writer_utils::{
        compress_to_archive, try_open_write, try_write_key_values, try_write_log_metadata,
        try_write_values, try_write_variants, try_write_version,
    },
};

pub fn write_bxes(path: &str, data: &BxesLogWriteData) -> Result<(), BxesWriteError> {
    let temp_file = create_temp_file()?;
    let raw_log_path = temp_file.path().to_str().unwrap();

    write_bxes_to_temp_file(data, raw_log_path)?;
    compress_to_archive(raw_log_path, path)?;

    Ok(())
}

pub fn write_bxes_to_bytes(data: &BxesLogWriteData) -> Result<Vec<u8>, BxesWriteError> {
    let temp_file = create_temp_file()?;
    let raw_log_path = temp_file.path().to_str().unwrap();

    write_bxes_to_temp_file(data, raw_log_path)?;
    let archive_temp_file = create_temp_file()?;
    let archive_path = archive_temp_file.path().to_str().unwrap();

    compress_to_archive(raw_log_path, archive_path)?;

    let mut bytes = vec![];

    match File::open(archive_path) {
        Ok(mut file) => match file.read_to_end(&mut bytes) {
            Ok(_) => Ok(bytes),
            Err(err) => Err(BxesWriteError::Default(err.to_string()))
        },
        Err(err) => Err(BxesWriteError::Default(err.to_string()))
    }
}

fn create_temp_file() -> Result<NamedTempFile, BxesWriteError> {
    match NamedTempFile::new() {
        Ok(file) => Ok(file),
        Err(_) => Err(BxesWriteError::FailedToCreateTempFile),
    }
}

fn write_bxes_to_temp_file(data: &BxesLogWriteData, raw_log_path: &str) -> Result<(), BxesWriteError> {
    let mut stream = try_open_write(raw_log_path)?;
    let mut writer = BinaryWriter::new(&mut stream, Endian::Little);

    let context = BxesWriteContext::new(&mut writer, data.system_metadata.values_attrs.clone());
    let context = Rc::new(RefCell::new(context));

    let log = &data.log;
    try_write_version(context.borrow_mut().writer.as_mut().unwrap(), log.version)?;
    try_write_system_metadata(&data.system_metadata, context.clone())?;
    try_write_values(log, context.clone())?;
    try_write_key_values(log, context.clone())?;
    try_write_log_metadata(log, context.clone())?;
    try_write_variants(log, context.clone())?;

    Ok(())
}