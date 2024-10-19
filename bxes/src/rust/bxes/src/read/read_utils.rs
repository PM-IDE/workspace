use num_traits::{FromPrimitive, ToPrimitive};
use std::io::{Cursor, Seek};
use std::{fs::File, io::Read, rc::Rc};
use tempfile::TempDir;
use uuid::Uuid;
use zip::ZipArchive;

use super::errors::*;
use crate::models::domain::bxes_artifact::{BxesArtifact, BxesArtifactItem};
use crate::models::domain::bxes_driver::{BxesDriver, BxesDrivers};
use crate::models::domain::bxes_event_log::{BxesEvent, BxesEventLog, BxesTraceVariant};
use crate::models::domain::bxes_lifecycle::{BrafLifecycle, StandardLifecycle};
use crate::models::domain::bxes_log_metadata::{
    BxesClassifier, BxesEventLogMetadata, BxesExtension, BxesGlobal, BxesGlobalKind,
};
use crate::models::domain::bxes_value::BxesValue;
use crate::models::domain::software_event_type::SoftwareEventType;
use crate::models::domain::type_ids::{get_type_id, TypeIds};
use crate::models::system_models::{SystemMetadata, ValueAttributeDescriptor};
use crate::read::errors::BxesReadError::ValueAttributeNameIsNotAString;
use crate::read::read_context::ReadContext;
use crate::{
    binary_rw::{
        core::{BinaryReader, SeekStream},
        file_stream::FileStream,
    },
    utils::buffered_stream::BufferedReadFileStream,
};

#[derive(Debug)]
pub struct BxesEventLogReadResult {
    pub log: BxesEventLog,
    pub system_metadata: SystemMetadata,
}

pub fn try_read_event_log_metadata(
    context: &mut ReadContext,
) -> Result<BxesEventLogMetadata, BxesReadError> {
    let properties = try_read_attributes(context, false)?;
    let extensions = try_read_extensions(context)?;
    let globals = try_read_globals(context)?;
    let classifiers = try_read_classifiers(context)?;

    Ok(BxesEventLogMetadata {
        extensions,
        classifiers,
        properties,
        globals,
    })
}

pub fn try_read_system_metadata(context: &mut ReadContext) -> Result<(), BxesReadError> {
    context.metadata.system_metadata = Some(SystemMetadata {
        values_attrs: try_read_value_attributes(context)?,
    });

    Ok(())
}

fn try_read_value_attributes(
    context: &mut ReadContext,
) -> Result<Option<Vec<ValueAttributeDescriptor>>, BxesReadError> {
    let reader = context.reader.as_mut().unwrap();
    let count = try_read_u32(reader)?;
    if count == 0 {
        Ok(None)
    } else {
        let mut values_attributes = vec![];
        for _ in 0..count {
            let type_id = try_read_type_id(reader)?;
            let name_type_id = try_read_type_id(reader)?;

            if name_type_id == TypeIds::String {
                let name = try_read_string(reader)?;
                values_attributes.push(ValueAttributeDescriptor::new(type_id, name));
            } else {
                return Err(ValueAttributeNameIsNotAString);
            }
        }

        Ok(Some(values_attributes))
    }
}

pub fn try_read_classifiers(
    context: &mut ReadContext,
) -> Result<Option<Vec<BxesClassifier>>, BxesReadError> {
    let count = try_read_u32(context.reader.as_mut().unwrap())?;
    if count == 0 {
        Ok(None)
    } else {
        let mut classifiers = vec![];

        for _ in 0..count {
            let index = try_read_u32(context.reader.as_mut().unwrap())? as usize;
            let name = context
                .metadata
                .values
                .as_ref()
                .unwrap()
                .get(index)
                .unwrap()
                .clone();

            let keys_count = try_read_u32(context.reader.as_mut().unwrap())?;
            let mut keys = vec![];
            for _ in 0..keys_count {
                let index = try_read_u32(context.reader.as_mut().unwrap())? as usize;
                let key_value = context
                    .metadata
                    .values
                    .as_ref()
                    .unwrap()
                    .get(index)
                    .unwrap();
                keys.push(key_value.clone());
            }

            classifiers.push(BxesClassifier { name, keys });
        }

        Ok(Some(classifiers))
    }
}

pub fn try_read_globals(
    context: &mut ReadContext,
) -> Result<Option<Vec<BxesGlobal>>, BxesReadError> {
    let count = try_read_u32(context.reader.as_mut().unwrap())?;
    if count == 0 {
        Ok(None)
    } else {
        let mut globals = vec![];

        for _ in 0..count {
            let entity_kind =
                BxesGlobalKind::from_u8(try_read_u8(context.reader.as_mut().unwrap())?).unwrap();
            let globals_count = try_read_u32(context.reader.as_mut().unwrap())?;
            let mut entity_globals = vec![];

            for _ in 0..globals_count {
                entity_globals.push(try_read_kv_pair(context, false)?);
            }

            globals.push(BxesGlobal {
                entity_kind,
                globals: entity_globals,
            });
        }

        Ok(Some(globals))
    }
}

pub fn try_read_extensions(
    context: &mut ReadContext,
) -> Result<Option<Vec<BxesExtension>>, BxesReadError> {
    let count = try_read_u32(context.reader.as_mut().unwrap())?;
    if count == 0 {
        Ok(None)
    } else {
        let mut extensions = vec![];

        for _ in 0..count {
            let index = try_read_u32(context.reader.as_mut().unwrap())? as usize;
            let name = context
                .metadata
                .values
                .as_ref()
                .unwrap()
                .get(index)
                .unwrap()
                .clone();

            let index = try_read_u32(context.reader.as_mut().unwrap())? as usize;
            let prefix = context
                .metadata
                .values
                .as_ref()
                .unwrap()
                .get(index)
                .unwrap()
                .clone();

            let index = try_read_u32(context.reader.as_mut().unwrap())? as usize;
            let uri = context
                .metadata
                .values
                .as_ref()
                .unwrap()
                .get(index)
                .unwrap()
                .clone();

            extensions.push(BxesExtension { name, prefix, uri })
        }

        Ok(Some(extensions))
    }
}

pub fn try_read_leb128(reader: &mut BinaryReader) -> Result<u32, BxesReadError> {
    match leb128::read::unsigned(reader) {
        Ok(value) => Ok(value as u32),
        Err(err) => Err(BxesReadError::Leb128ReadError(err.to_string())),
    }
}

pub fn string_or_err(value: &BxesValue) -> Result<Rc<Box<String>>, BxesReadError> {
    if let BxesValue::String(string) = value {
        Ok(string.clone())
    } else {
        Err(BxesReadError::ExpectedString(value.clone()))
    }
}

pub fn owned_string_or_err(value: &BxesValue) -> Result<String, BxesReadError> {
    if let BxesValue::String(string) = value {
        Ok(string.as_ref().as_ref().to_owned())
    } else {
        Err(BxesReadError::ExpectedString(value.clone()))
    }
}

pub fn try_read_traces_variants(
    context: &mut ReadContext,
) -> Result<Vec<BxesTraceVariant>, BxesReadError> {
    let mut variants = vec![];
    let variant_count = try_read_u32(context.reader.as_mut().unwrap())?;

    for _ in 0..variant_count {
        variants.push(try_read_trace_variant(context)?);
    }

    Ok(variants)
}

pub fn try_read_trace_variant(
    context: &mut ReadContext,
) -> Result<BxesTraceVariant, BxesReadError> {
    let traces_count = try_read_u32(context.reader.as_mut().unwrap())?;

    let variant_metadata = try_read_trace_variant_metadata(context)?;
    let events = try_read_trace_variant_events(context)?;

    Ok(BxesTraceVariant {
        traces_count,
        metadata: variant_metadata,
        events,
    })
}

pub fn try_read_trace_variant_metadata(
    context: &mut ReadContext,
) -> Result<Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)>, BxesReadError> {
    let mut variant_metadata = vec![];
    let metadata_count = try_read_u32(context.reader.as_mut().unwrap())?;
    for _ in 0..metadata_count {
        variant_metadata.push(try_read_kv_pair(context, false)?);
    }

    Ok(variant_metadata)
}

pub fn try_read_trace_variant_events(
    context: &mut ReadContext,
) -> Result<Vec<BxesEvent>, BxesReadError> {
    let events_count = try_read_u32(context.reader.as_mut().unwrap())?;
    let mut events = vec![];

    for _ in 0..events_count {
        events.push(try_read_event(context)?);
    }

    Ok(events)
}

fn try_read_event(context: &mut ReadContext) -> Result<BxesEvent, BxesReadError> {
    let name_index = try_read_leb128(context.reader.as_mut().unwrap())? as usize;
    let name = context.metadata.values.as_ref().unwrap().get(name_index);

    if name.is_none() {
        return Err(BxesReadError::FailedToIndexValue(name_index));
    }

    let timestamp = try_read_i64(context.reader.as_mut().unwrap())?;

    Ok(BxesEvent {
        name: name.unwrap().clone(),
        timestamp,
        attributes: try_read_event_attributes(context)?,
    })
}

fn try_read_event_attributes(
    context: &mut ReadContext,
) -> Result<Option<Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)>>, BxesReadError> {
    let mut attributes = None;
    let value_attrs_len = if let Some(attrs) = context
        .metadata
        .system_metadata
        .as_ref()
        .unwrap()
        .values_attrs
        .as_ref()
    {
        attrs.len()
    } else {
        0
    };

    if value_attrs_len > 0 {
        for i in 0..value_attrs_len {
            let value = try_read_bxes_value(context)?;
            let metadata = context.metadata.system_metadata.as_ref().unwrap();
            let value_attrs = metadata.values_attrs.as_ref().unwrap();
            let descriptor = value_attrs.get(i).unwrap();

            let key = BxesValue::String(Rc::new(Box::new(descriptor.name.clone())));
            let key = Rc::new(Box::new(key));

            let value_type_id = get_type_id(&value);
            let null_type_id = TypeIds::Null;

            if value_type_id != null_type_id || descriptor.type_id == null_type_id {
                if attributes.is_none() {
                    attributes = Some(vec![]);
                }

                attributes
                    .as_mut()
                    .unwrap()
                    .push((key, Rc::new(Box::new(value))));
            }
        }
    }

    try_fill_attributes(context, true, &mut attributes)?;

    Ok(attributes)
}

pub fn try_fill_attributes(
    context: &mut ReadContext,
    leb_128: bool,
    attributes: &mut Option<Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)>>,
) -> Result<(), BxesReadError> {
    let attributes_count = try_read_count(context, leb_128)?;
    if attributes_count > 0 && attributes.is_none() {
        *attributes = Some(vec![]);
    }

    for _ in 0..attributes_count {
        attributes
            .as_mut()
            .unwrap()
            .push(try_read_kv_pair(context, leb_128)?);
    }

    Ok(())
}

fn try_read_attributes(
    context: &mut ReadContext,
    leb_128: bool,
) -> Result<Option<Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)>>, BxesReadError> {
    let attributes_count = try_read_count(context, leb_128)?;
    if attributes_count == 0 {
        Ok(None)
    } else {
        let mut attributes = vec![];
        for _ in 0..attributes_count {
            let pair = try_read_kv_pair(context, leb_128)?;
            attributes.push(pair);
        }

        Ok(Some(attributes))
    }
}

fn try_read_count(context: &mut ReadContext, leb_128: bool) -> Result<u32, BxesReadError> {
    if leb_128 {
        try_read_leb128(context.reader.as_mut().unwrap())
    } else {
        try_read_u32(context.reader.as_mut().unwrap())
    }
}

fn try_read_kv_pair(
    context: &mut ReadContext,
    leb_128: bool,
) -> Result<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>), BxesReadError> {
    let kv_index = try_read_count(context, leb_128)? as usize;

    let kv_pair = match context.metadata.kv_pairs.as_ref().unwrap().get(kv_index) {
        None => return Err(BxesReadError::FailedToIndexKeyValue(kv_index)),
        Some(pair) => pair,
    };

    let key_index = kv_pair.0 as usize;
    let key = match context.metadata.values.as_ref().unwrap().get(key_index) {
        None => return Err(BxesReadError::FailedToIndexValue(key_index)),
        Some(value) => value,
    };

    let value_index = kv_pair.1 as usize;
    let value = match context.metadata.values.as_ref().unwrap().get(value_index) {
        None => return Err(BxesReadError::FailedToIndexValue(value_index)),
        Some(value) => value,
    };

    Ok((key.clone(), value.clone()))
}

pub fn try_read_key_values(context: &mut ReadContext) -> Result<(), BxesReadError> {
    let reader = context.reader.as_mut().unwrap();
    if context.metadata.kv_pairs.is_none() {
        context.metadata.kv_pairs = Some(vec![]);
    }

    let key_values_count = try_read_u32(reader)?;
    for _ in 0..key_values_count {
        context
            .metadata
            .kv_pairs
            .as_mut()
            .unwrap()
            .push((try_read_leb128(reader)?, try_read_leb128(reader)?));
    }

    Ok(())
}

pub fn try_read_values(context: &mut ReadContext) -> Result<(), BxesReadError> {
    let reader = context.reader.as_mut().unwrap();

    if context.metadata.values.is_none() {
        context.metadata.values = Some(vec![]);
    }

    let values_count = try_read_u32(reader)?;
    for _ in 0..values_count {
        let value = try_read_bxes_value(context)?;
        context
            .metadata
            .values
            .as_mut()
            .unwrap()
            .push(Rc::new(Box::new(value)));
    }

    Ok(())
}

fn try_read_bxes_value(context: &mut ReadContext) -> Result<BxesValue, BxesReadError> {
    let reader = context.reader.as_mut().unwrap();

    match try_read_type_id(reader)? {
        TypeIds::Null => Ok(BxesValue::Null),
        TypeIds::I32 => Ok(BxesValue::Int32(try_read_i32(reader)?)),
        TypeIds::I64 => Ok(BxesValue::Int64(try_read_i64(reader)?)),
        TypeIds::U32 => Ok(BxesValue::Uint32(try_read_u32(reader)?)),
        TypeIds::U64 => Ok(BxesValue::Uint64(try_read_u64(reader)?)),
        TypeIds::F32 => Ok(BxesValue::Float32(try_read_f32(reader)?)),
        TypeIds::F64 => Ok(BxesValue::Float64(try_read_f64(reader)?)),
        TypeIds::Bool => Ok(BxesValue::Bool(try_read_bool(reader)?)),
        TypeIds::String => Ok(BxesValue::String(Rc::new(Box::new(try_read_string(
            reader,
        )?)))),
        TypeIds::Timestamp => Ok(BxesValue::Timestamp(try_read_i64(reader)?)),
        TypeIds::BrafLifecycle => Ok(BxesValue::BrafLifecycle(try_read_braf_lifecycle(reader)?)),
        TypeIds::StandardLifecycle => Ok(BxesValue::StandardLifecycle(
            try_read_standard_lifecycle(reader)?,
        )),
        TypeIds::Guid => Ok(BxesValue::Guid(try_read_guid(reader)?)),
        TypeIds::Artifact => Ok(BxesValue::Artifact(try_read_artifact(context)?)),
        TypeIds::Drivers => Ok(BxesValue::Drivers(try_read_drivers(context)?)),
        TypeIds::SoftwareEventType => Ok(BxesValue::SoftwareEventType(
            try_read_software_event_type(reader)?,
        )),
    }
}

fn try_read_type_id(reader: &mut BinaryReader) -> Result<TypeIds, BxesReadError> {
    let type_id_byte = try_read_u8(reader)?;
    match TypeIds::from_u8(type_id_byte) {
        None => Err(BxesReadError::FailedToParseTypeId(type_id_byte)),
        Some(id) => Ok(id),
    }
}

pub fn try_read_drivers(context: &mut ReadContext) -> Result<BxesDrivers, BxesReadError> {
    let reader = context.reader.as_mut().unwrap();
    let drivers_count = try_read_u32(reader)?;
    let mut drivers = vec![];

    for _ in 0..drivers_count {
        drivers.push(try_read_driver(context)?);
    }

    Ok(BxesDrivers { drivers })
}

pub fn try_read_driver(context: &mut ReadContext) -> Result<BxesDriver, BxesReadError> {
    let amount = try_read_f64(context.reader.as_mut().unwrap())?;
    let name_index = try_read_u32(context.reader.as_mut().unwrap())? as usize;
    let driver_type_index = try_read_u32(context.reader.as_mut().unwrap())? as usize;

    Ok(BxesDriver {
        amount: BxesValue::Float64(amount),
        name: context.metadata.values.as_ref().unwrap()[name_index].clone(),
        driver_type: context.metadata.values.as_ref().unwrap()[driver_type_index].clone(),
    })
}

pub fn try_read_artifact(context: &mut ReadContext) -> Result<BxesArtifact, BxesReadError> {
    let artifacts_count = try_read_u32(context.reader.as_mut().unwrap())?;
    let mut artifacts = vec![];

    for _ in 0..artifacts_count {
        artifacts.push(try_read_artifact_item(context)?);
    }

    Ok(BxesArtifact { items: artifacts })
}

pub fn try_read_artifact_item(
    context: &mut ReadContext,
) -> Result<BxesArtifactItem, BxesReadError> {
    let model_index = try_read_u32(context.reader.as_mut().unwrap())? as usize;
    let instance_index = try_read_u32(context.reader.as_mut().unwrap())? as usize;
    let transition_index = try_read_u32(context.reader.as_mut().unwrap())? as usize;

    Ok(BxesArtifactItem {
        model: context.metadata.values.as_ref().unwrap()[model_index].clone(),
        instance: context.metadata.values.as_ref().unwrap()[instance_index].clone(),
        transition: context.metadata.values.as_ref().unwrap()[transition_index].clone(),
    })
}

pub fn try_read_guid(reader: &mut BinaryReader) -> Result<Uuid, BxesReadError> {
    try_read(try_tell_pos(reader)?, || {
        let mut buf = [0; 16];
        reader.read(&mut buf)?;

        Ok(Uuid::from_slice_le(&buf).unwrap())
    })
}

pub fn try_read_software_event_type(
    reader: &mut BinaryReader,
) -> Result<SoftwareEventType, BxesReadError> {
    try_read_enum::<SoftwareEventType>(reader)
}

pub fn try_read_i32(reader: &mut BinaryReader) -> Result<i32, BxesReadError> {
    try_read(try_tell_pos(reader)?, || reader.read_i32())
}

pub fn try_read_i64(reader: &mut BinaryReader) -> Result<i64, BxesReadError> {
    try_read(try_tell_pos(reader)?, || reader.read_i64())
}

pub fn try_read_u32(reader: &mut BinaryReader) -> Result<u32, BxesReadError> {
    try_read(try_tell_pos(reader)?, || reader.read_u32())
}

pub fn try_read_u64(reader: &mut BinaryReader) -> Result<u64, BxesReadError> {
    try_read(try_tell_pos(reader)?, || reader.read_u64())
}

pub fn try_read_f32(reader: &mut BinaryReader) -> Result<f32, BxesReadError> {
    try_read(try_tell_pos(reader)?, || reader.read_f32())
}

pub fn try_read_f64(reader: &mut BinaryReader) -> Result<f64, BxesReadError> {
    try_read(try_tell_pos(reader)?, || reader.read_f64())
}

pub fn try_read_bool(reader: &mut BinaryReader) -> Result<bool, BxesReadError> {
    try_read(try_tell_pos(reader)?, || reader.read_bool())
}

pub fn try_read_u8(reader: &mut BinaryReader) -> Result<u8, BxesReadError> {
    try_read(try_tell_pos(reader)?, || reader.read_u8())
}

fn try_read_string(reader: &mut BinaryReader) -> Result<String, BxesReadError> {
    let string_length = try_read_u64(reader)?;
    let bytes = try_read_bytes(reader, string_length as usize)?;

    match String::from_utf8(bytes) {
        Ok(string) => Ok(string),
        Err(err) => Err(BxesReadError::FailedToCreateUtf8String(err)),
    }
}

fn try_read_bytes(reader: &mut BinaryReader, length: usize) -> Result<Vec<u8>, BxesReadError> {
    let offset = try_tell_pos(reader)?;
    let mut buf = vec![0; length];
    match reader.read(&mut buf) {
        Ok(_) => Ok(buf),
        Err(err) => Err(BxesReadError::FailedToReadValue(
            FailedToReadValueError::new(offset, err.to_string()),
        )),
    }
}

fn try_read_braf_lifecycle(reader: &mut BinaryReader) -> Result<BrafLifecycle, BxesReadError> {
    try_read_enum::<BrafLifecycle>(reader)
}

fn try_read_enum<T: FromPrimitive>(reader: &mut BinaryReader) -> Result<T, BxesReadError> {
    let offset = try_tell_pos(reader)?;
    match reader.read_u8() {
        Ok(byte) => Ok(T::from_u8(byte).unwrap()),
        Err(err) => Err(BxesReadError::FailedToReadValue(
            FailedToReadValueError::new(offset, err.to_string()),
        )),
    }
}

fn try_read_standard_lifecycle(
    reader: &mut BinaryReader,
) -> Result<StandardLifecycle, BxesReadError> {
    try_read_enum::<StandardLifecycle>(reader)
}

pub fn try_extract_archive_bytes(bytes: &[u8]) -> Result<TempDir, BxesReadError> {
    try_extract_archive_internal(Cursor::new(bytes))
}

pub fn try_extract_archive(path: &str) -> Result<TempDir, BxesReadError> {
    let fs = match File::open(path) {
        Ok(fs) => fs,
        Err(err) => return Err(BxesReadError::FailedToOpenFile(err.to_string())),
    };

    try_extract_archive_internal(fs)
}

fn try_extract_archive_internal(stream: impl Read + Seek) -> Result<TempDir, BxesReadError> {
    let mut archive = match ZipArchive::new(stream) {
        Ok(archive) => archive,
        Err(err) => return Err(BxesReadError::FailedToOpenFile(err.to_string())),
    };

    let temp_dir = match TempDir::new() {
        Ok(temp_dir) => temp_dir,
        Err(_) => return Err(BxesReadError::FailedToCreateTempDir),
    };

    match archive.extract(temp_dir.path()) {
        Ok(_) => {}
        Err(_) => return Err(BxesReadError::FailedToExtractArchive),
    };

    return Ok(temp_dir);
}

pub fn try_open_file_stream(path: &str) -> Result<BufferedReadFileStream, BxesReadError> {
    match FileStream::open(path) {
        Ok(fs) => Ok(BufferedReadFileStream::new(fs, 1024 * 8)),
        Err(err) => Err(BxesReadError::FailedToOpenFile(err.to_string())),
    }
}

fn try_read<T>(
    reader_pos: usize,
    mut read_func: impl FnMut() -> crate::binary_rw::core::Result<T>,
) -> Result<T, BxesReadError> {
    match read_func() {
        Ok(value) => Ok(value),
        Err(err) => Err(BxesReadError::FailedToReadValue(
            FailedToReadValueError::new(reader_pos, err.to_string()),
        )),
    }
}

fn try_tell_pos(reader: &mut BinaryReader) -> Result<usize, BxesReadError> {
    match reader.tell() {
        Ok(pos) => Ok(pos),
        Err(err) => Err(BxesReadError::FailedToReadPos(err.to_string())),
    }
}
