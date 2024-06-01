use std::{cell::RefCell, rc::Rc};

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
    let raw_log_path = match NamedTempFile::new() {
        Ok(file) => file,
        Err(_) => return Err(BxesWriteError::FailedToCreateTempFile),
    };

    let raw_log_path = raw_log_path.path().to_str().unwrap();
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

    compress_to_archive(raw_log_path, path)?;

    Ok(())
}
