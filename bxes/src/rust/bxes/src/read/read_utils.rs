use std::{fs::File, io::Read, rc::Rc};

use num_traits::FromPrimitive;
use tempfile::TempDir;
use uuid::Uuid;
use zip::ZipArchive;

use crate::models::domain::bxes_artifact::{BxesArtifact, BxesArtifactItem};
use crate::models::domain::bxes_driver::{BxesDriver, BxesDrivers};
use crate::models::domain::bxes_event_log::{BxesEvent, BxesEventLog, BxesTraceVariant};
use crate::models::domain::bxes_lifecycle::{BrafLifecycle, StandardLifecycle};
use crate::models::domain::bxes_log_metadata::{
    BxesClassifier, BxesEventLogMetadata, BxesExtension, BxesGlobal, BxesGlobalKind,
};
use crate::models::domain::bxes_value::BxesValue;
use crate::models::domain::software_event_type::SoftwareEventType;
use crate::models::system_models::{SystemMetadata, ValueAttributeDescriptor};
use crate::{
    binary_rw::{
        core::{BinaryReader, SeekStream},
        file_stream::FileStream,
    },
    type_ids::TypeIds,
    utils::buffered_stream::BufferedReadFileStream,
};

use super::errors::*;

pub struct BxesEventLogReadResult {
    pub log: BxesEventLog,
    pub system_metadata: SystemMetadata,
}

pub fn try_read_event_log_metadata(
    reader: &mut BinaryReader,
    values: &Vec<Rc<Box<BxesValue>>>,
    kv_pairs: &Vec<(u32, u32)>,
) -> Result<BxesEventLogMetadata, BxesReadError> {
    let properties = try_read_attributes(reader, values, kv_pairs, false)?;
    let extensions = try_read_extensions(reader, values, kv_pairs)?;
    let globals = try_read_globals(reader, values, kv_pairs)?;
    let classifiers = try_read_classifiers(reader, values, kv_pairs)?;

    Ok(BxesEventLogMetadata {
        extensions,
        classifiers,
        properties,
        globals,
    })
}

pub fn try_read_system_metadata(
    reader: &mut BinaryReader,
) -> Result<SystemMetadata, BxesReadError> {
    let values_attributes = try_read_value_attributes(reader)?;

    Ok(SystemMetadata {
        values_attrs: values_attributes,
    })
}

fn try_read_value_attributes(
    reader: &mut BinaryReader,
) -> Result<Option<Vec<ValueAttributeDescriptor>>, BxesReadError> {
    let count = try_read_u32(reader)?;
    if count == 0 {
        Ok(None)
    } else {
        let mut values_attributes = vec![];
        for _ in 0..count {
            let type_id = try_read_type_id(reader)?;

            let name_type_id = try_read_type_id(reader)?;
            if name_type_id != TypeIds::String {
                return Err(BxesReadError::ValueAttributeNameIsNotAString);
            }

            let name = try_read_string(reader)?;
            values_attributes.push(ValueAttributeDescriptor::new(type_id, name));
        }

        Ok(Some(values_attributes))
    }
}

pub fn try_read_classifiers(
    reader: &mut BinaryReader,
    values: &Vec<Rc<Box<BxesValue>>>,
    kv_pairs: &Vec<(u32, u32)>,
) -> Result<Option<Vec<BxesClassifier>>, BxesReadError> {
    let count = try_read_u32(reader)?;
    if count == 0 {
        Ok(None)
    } else {
        let mut classifiers = vec![];

        for _ in 0..count {
            let name = values.get(try_read_u32(reader)? as usize).unwrap().clone();

            let keys_count = try_read_u32(reader)?;
            let mut keys = vec![];
            for _ in 0..keys_count {
                let key_value = values.get(try_read_u32(reader)? as usize).unwrap();
                keys.push(key_value.clone());
            }

            classifiers.push(BxesClassifier { name, keys });
        }

        Ok(Some(classifiers))
    }
}

pub fn try_read_globals(
    reader: &mut BinaryReader,
    values: &Vec<Rc<Box<BxesValue>>>,
    kv_pairs: &Vec<(u32, u32)>,
) -> Result<Option<Vec<BxesGlobal>>, BxesReadError> {
    let count = try_read_u32(reader)?;
    if count == 0 {
        Ok(None)
    } else {
        let mut globals = vec![];

        for _ in 0..count {
            let entity_kind = BxesGlobalKind::from_u8(try_read_u8(reader)?).unwrap();
            let globals_count = try_read_u32(reader)?;
            let mut entity_globals = vec![];

            for _ in 0..globals_count {
                entity_globals.push(try_read_kv_pair(reader, values, kv_pairs, false)?);
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
    reader: &mut BinaryReader,
    values: &Vec<Rc<Box<BxesValue>>>,
    kv_pairs: &Vec<(u32, u32)>,
) -> Result<Option<Vec<BxesExtension>>, BxesReadError> {
    let count = try_read_u32(reader)?;
    if count == 0 {
        Ok(None)
    } else {
        let mut extensions = vec![];

        for _ in 0..count {
            let name = values.get(try_read_u32(reader)? as usize).unwrap().clone();
            let prefix = values.get(try_read_u32(reader)? as usize).unwrap().clone();
            let uri = values.get(try_read_u32(reader)? as usize).unwrap().clone();

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

fn string_or_err(value: &BxesValue) -> Result<Rc<Box<String>>, BxesReadError> {
    if let BxesValue::String(string) = value {
        Ok(string.clone())
    } else {
        Err(BxesReadError::ExpectedString(value.clone()))
    }
}

pub fn try_read_traces_variants(
    reader: &mut BinaryReader,
    values: &Vec<Rc<Box<BxesValue>>>,
    kv_pairs: &Vec<(u32, u32)>,
) -> Result<Vec<BxesTraceVariant>, BxesReadError> {
    let mut variants = vec![];
    let variant_count = try_read_u32(reader)?;

    for _ in 0..variant_count {
        variants.push(try_read_trace_variant(reader, values, kv_pairs)?);
    }

    Ok(variants)
}

fn try_read_trace_variant(
    reader: &mut BinaryReader,
    values: &Vec<Rc<Box<BxesValue>>>,
    kv_pairs: &Vec<(u32, u32)>,
) -> Result<BxesTraceVariant, BxesReadError> {
    let traces_count = try_read_u32(reader)?;

    let mut variant_metadata = vec![];
    let metadata_count = try_read_u32(reader)?;
    for _ in 0..metadata_count {
        variant_metadata.push(try_read_kv_pair(reader, values, kv_pairs, false)?);
    }

    let events_count = try_read_u32(reader)?;
    let mut events = vec![];

    for _ in 0..events_count {
        events.push(try_read_event(reader, values, kv_pairs)?);
    }

    Ok(BxesTraceVariant {
        traces_count,
        metadata: variant_metadata,
        events,
    })
}

fn try_read_event(
    reader: &mut BinaryReader,
    values: &Vec<Rc<Box<BxesValue>>>,
    kv_pairs: &Vec<(u32, u32)>,
) -> Result<BxesEvent, BxesReadError> {
    let name_index = try_read_leb128(reader)? as usize;
    let name = values.get(name_index);

    if name.is_none() {
        return Err(BxesReadError::FailedToIndexValue(name_index));
    }

    let timestamp = try_read_i64(reader)?;

    Ok(BxesEvent {
        name: name.unwrap().clone(),
        timestamp,
        attributes: try_read_attributes(reader, values, kv_pairs, true)?,
    })
}

fn try_read_attributes(
    reader: &mut BinaryReader,
    values: &Vec<Rc<Box<BxesValue>>>,
    kv_pairs: &Vec<(u32, u32)>,
    leb_128: bool,
) -> Result<Option<Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)>>, BxesReadError> {
    let attributes_count = if leb_128 {
        try_read_leb128(reader)?
    } else {
        try_read_u32(reader)?
    };

    if attributes_count == 0 {
        Ok(None)
    } else {
        let mut attributes = vec![];
        for _ in 0..attributes_count {
            let pair = try_read_kv_pair(reader, values, kv_pairs, leb_128)?;
            attributes.push(pair);
        }

        Ok(Some(attributes))
    }
}

fn try_read_kv_pair(
    reader: &mut BinaryReader,
    values: &Vec<Rc<Box<BxesValue>>>,
    kv_pairs: &Vec<(u32, u32)>,
    leb_128: bool,
) -> Result<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>), BxesReadError> {
    let kv_index = if leb_128 {
        try_read_leb128(reader)?
    } else {
        try_read_u32(reader)?
    } as usize;

    let kv_pair = match kv_pairs.get(kv_index) {
        None => return Err(BxesReadError::FailedToIndexKeyValue(kv_index)),
        Some(pair) => pair,
    };

    let key_index = kv_pair.0 as usize;
    let key = match values.get(key_index) {
        None => return Err(BxesReadError::FailedToIndexValue(key_index)),
        Some(value) => value,
    };

    let value_index = kv_pair.1 as usize;
    let value = match values.get(value_index) {
        None => return Err(BxesReadError::FailedToIndexValue(value_index)),
        Some(value) => value,
    };

    Ok((key.clone(), value.clone()))
}

pub fn try_read_key_values(reader: &mut BinaryReader) -> Result<Vec<(u32, u32)>, BxesReadError> {
    let mut key_values = vec![];

    let key_values_count = try_read_u32(reader)?;
    for _ in 0..key_values_count {
        key_values.push((try_read_leb128(reader)?, try_read_leb128(reader)?));
    }

    Ok(key_values)
}

pub fn try_read_values(
    reader: &mut BinaryReader,
) -> Result<Vec<Rc<Box<BxesValue>>>, BxesReadError> {
    let mut values = vec![];

    let values_count = try_read_u32(reader)?;
    for _ in 0..values_count {
        values.push(Rc::new(Box::new(try_read_bxes_value(reader, &values)?)));
    }

    Ok(values)
}

fn try_read_bxes_value(
    reader: &mut BinaryReader,
    values: &Vec<Rc<Box<BxesValue>>>,
) -> Result<BxesValue, BxesReadError> {
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
        TypeIds::Artifact => Ok(BxesValue::Artifact(try_read_artifact(reader, values)?)),
        TypeIds::Drivers => Ok(BxesValue::Drivers(try_read_drivers(reader, values)?)),
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

pub fn try_read_drivers(
    reader: &mut BinaryReader,
    values: &Vec<Rc<Box<BxesValue>>>,
) -> Result<BxesDrivers, BxesReadError> {
    let drivers_count = try_read_u32(reader)?;
    let mut drivers = vec![];

    for _ in 0..drivers_count {
        drivers.push(try_read_driver(reader, values)?);
    }

    Ok(BxesDrivers { drivers })
}

pub fn try_read_driver(
    reader: &mut BinaryReader,
    values: &Vec<Rc<Box<BxesValue>>>,
) -> Result<BxesDriver, BxesReadError> {
    let amount = try_read_f64(reader)?;
    let name_index = try_read_u32(reader)? as usize;
    let driver_type_index = try_read_u32(reader)? as usize;

    Ok(BxesDriver {
        amount: BxesValue::Float64(amount),
        name: values[name_index].clone(),
        driver_type: values[driver_type_index].clone(),
    })
}

pub fn try_read_artifact(
    reader: &mut BinaryReader,
    values: &Vec<Rc<Box<BxesValue>>>,
) -> Result<BxesArtifact, BxesReadError> {
    let artifacts_count = try_read_u32(reader)?;
    let mut artifacts = vec![];

    for _ in 0..artifacts_count {
        artifacts.push(try_read_artifact_item(reader, values)?);
    }

    Ok(BxesArtifact { items: artifacts })
}

pub fn try_read_artifact_item(
    reader: &mut BinaryReader,
    values: &Vec<Rc<Box<BxesValue>>>,
) -> Result<BxesArtifactItem, BxesReadError> {
    let model_index = try_read_u32(reader)? as usize;
    let instance_index = try_read_u32(reader)? as usize;
    let transition_index = try_read_u32(reader)? as usize;

    Ok(BxesArtifactItem {
        model: values[model_index].clone(),
        instance: values[instance_index].clone(),
        transition: values[transition_index].clone(),
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

pub fn try_extract_archive(path: &str) -> Result<TempDir, BxesReadError> {
    let fs = match File::open(path) {
        Ok(fs) => fs,
        Err(err) => return Err(BxesReadError::FailedToOpenFile(err.to_string())),
    };

    let mut archive = match ZipArchive::new(fs) {
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
