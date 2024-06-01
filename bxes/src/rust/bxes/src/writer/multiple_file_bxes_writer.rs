use crate::writer::writer_utils::{try_write_system_metadata, BxesLogWriteData};
use crate::{
    binary_rw::core::{BinaryWriter, Endian},
    constants,
};
use std::{cell::RefCell, path::Path, rc::Rc};

use super::{
    errors::BxesWriteError,
    write_context::BxesWriteContext,
    writer_utils::{
        try_open_write, try_write_key_values, try_write_log_metadata, try_write_u32_no_type_id,
        try_write_values, try_write_variants,
    },
};

type WriterFunc =
    dyn Fn(&BxesLogWriteData, Rc<RefCell<BxesWriteContext>>) -> Result<(), BxesWriteError>;

pub fn write_bxes_multiple_files(
    data: &BxesLogWriteData,
    directory_path: &str,
) -> Result<(), BxesWriteError> {
    let context = BxesWriteContext::empty(data.system_metadata.values_attrs.clone());

    let writer = |file_path: &'static str, action: Box<WriterFunc>| {
        execute_with_writer(&data, directory_path, file_path, &context, action)
    };

    writer(
        constants::SYSTEM_METADATA_FILE_NAME,
        Box::new(|data, context| try_write_system_metadata(&data.system_metadata, context)),
    )?;

    writer(
        constants::VALUES_FILE_NAME,
        Box::new(|data, context| try_write_values(&data.log, context)),
    )?;

    writer(
        constants::KEY_VALUES_FILE_NAME,
        Box::new(|data, context| try_write_key_values(&data.log, context)),
    )?;

    writer(
        constants::METADATA_FILE_NAME,
        Box::new(|data, context| try_write_log_metadata(&data.log, context)),
    )?;

    writer(
        constants::VARIANTS_FILE_NAME,
        Box::new(|data, context| try_write_variants(&data.log, context)),
    )
}

fn execute_with_writer<'a, T>(
    data: &'a BxesLogWriteData,
    directory_path: &'a str,
    file_name: &'static str,
    context: &'a BxesWriteContext<'_>,
    action: T,
) -> Result<(), BxesWriteError>
where
    T: Fn(&BxesLogWriteData, Rc<RefCell<BxesWriteContext>>) -> Result<(), BxesWriteError>,
{
    let directory_path = Path::new(directory_path);
    let file_path = directory_path.join(file_name);
    let file_path = file_path.to_str().unwrap();

    let mut file_stream = try_open_write(file_path)?;
    let mut writer = BinaryWriter::new(&mut file_stream, Endian::Little);

    try_write_u32_no_type_id(&mut writer, data.log.version)?;
    action(
        data,
        Rc::new(RefCell::new(context.with_writer(&mut writer))),
    )
}
