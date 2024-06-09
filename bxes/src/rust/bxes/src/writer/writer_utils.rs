use std::{
    cell::RefCell,
    fs::{self, File},
    io::Write,
    path::Path,
    rc::Rc,
};
use std::collections::HashMap;

use num_traits::ToPrimitive;
use zip::{write::FileOptions, ZipWriter};

use crate::binary_rw::{
    core::{BinaryWriter, SeekStream},
    file_stream::FileStream,
};
use crate::models::domain::bxes_artifact::BxesArtifact;
use crate::models::domain::bxes_driver::BxesDrivers;
use crate::models::domain::bxes_event_log::{BxesEvent, BxesEventLog};
use crate::models::domain::bxes_lifecycle::{BrafLifecycle, Lifecycle, StandardLifecycle};
use crate::models::domain::bxes_log_metadata::{BxesClassifier, BxesExtension, BxesGlobal};
use crate::models::domain::bxes_value::BxesValue;
use crate::models::domain::software_event_type::SoftwareEventType;
use crate::models::domain::type_ids::{get_type_id, TypeIds};
use crate::models::system_models::{SystemMetadata, ValueAttributeDescriptor};
use crate::read::read_utils::string_or_err;

use super::{errors::BxesWriteError, write_context::BxesWriteContext};

pub struct BxesLogWriteData {
    pub log: BxesEventLog,
    pub system_metadata: SystemMetadata,
}

pub fn try_write_system_metadata(
    system_metadata: &SystemMetadata,
    context: Rc<RefCell<BxesWriteContext>>,
) -> Result<(), BxesWriteError> {
    try_write_values_attributes(system_metadata.values_attrs.as_ref(), context)
}

fn try_write_values_attributes(
    value_attributes: Option<&Vec<ValueAttributeDescriptor>>,
    context: Rc<RefCell<BxesWriteContext>>,
) -> Result<(), BxesWriteError> {
    if let Some(value_attributes) = value_attributes {
        write_collection_and_count(
            context.clone(),
            false,
            value_attributes.len() as u32,
            || {
                for attr in value_attributes {
                    try_write_u8_no_type_id(
                        context.borrow_mut().writer.as_mut().unwrap(),
                        attr.type_id.to_u8().unwrap(),
                    )?;
                    try_write_string(
                        context.borrow_mut().writer.as_mut().unwrap(),
                        attr.name.as_str(),
                    )?;
                }

                Ok(())
            },
        )
    } else {
        try_write_u32_no_type_id(context.borrow_mut().writer.as_mut().unwrap(), 0)
    }
}

pub fn try_write_variants(
    log: &BxesEventLog,
    context: Rc<RefCell<BxesWriteContext>>,
) -> Result<(), BxesWriteError> {
    write_collection_and_count(context.clone(), false, log.variants.len() as u32, || {
        for variant in &log.variants {
            try_write_u32_no_type_id(
                context.borrow_mut().writer.as_mut().unwrap(),
                variant.traces_count,
            )?;

            try_write_attributes(context.clone(), Some(&variant.metadata), false)?;

            write_collection_and_count(
                context.clone(),
                false,
                variant.events.len() as u32,
                || {
                    for event in &variant.events {
                        try_write_event(event, context.clone())?;
                    }

                    Ok(())
                },
            )?;
        }

        Ok(())
    })
}

pub fn try_write_event(
    event: &BxesEvent,
    context: Rc<RefCell<BxesWriteContext>>,
) -> Result<(), BxesWriteError> {
    let exists = context
        .borrow()
        .values_indices
        .borrow()
        .contains_key(&event.name);

    if exists {
        let index = *context
            .borrow()
            .values_indices
            .borrow()
            .get(&event.name)
            .unwrap();

        try_write_leb_128(context.borrow_mut().writer.as_mut().unwrap(), index as u32)?;
    } else {
        return Err(BxesWriteError::FailedToFindValueIndex(event.name.clone()));
    };

    try_write_i64_no_type_id(
        context.borrow_mut().writer.as_mut().unwrap(),
        event.timestamp,
    )?;

    try_write_event_attributes(event, context.clone())
}

fn try_write_event_attributes(
    event: &BxesEvent,
    context: Rc<RefCell<BxesWriteContext>>,
) -> Result<(), BxesWriteError> {
    let value_attrs_count = try_write_event_value_attributes(event, context.clone())?;
    try_write_event_default_attributes(event, context.clone(), value_attrs_count)?;

    Ok(())
}

fn try_write_event_default_attributes(
    event: &BxesEvent,
    context: Rc<RefCell<BxesWriteContext>>,
    value_attrs_count: usize,
) -> Result<(), BxesWriteError> {
    let default_attrs_count = count(event.attributes.as_ref()) - value_attrs_count as u32;
    write_collection_and_count(context.clone(), true, default_attrs_count, || {
        if let Some(attributes) = event.attributes.as_ref() {
            for (key, value) in attributes {
                let should_write = if let Some(set) = context.borrow().value_attributes_set.as_ref()
                {
                    let desc = ValueAttributeDescriptor {
                        name: string_or_err(&key).ok().unwrap().as_ref().as_ref().clone(),
                        type_id: get_type_id(&value),
                    };

                    !set.contains(&desc)
                } else {
                    true
                };

                if should_write {
                    try_write_kv_index(context.clone(), &(key.clone(), value.clone()), true)?;
                }
            }
        }

        Ok(())
    })
}

fn try_write_event_value_attributes(
    event: &BxesEvent,
    context: Rc<RefCell<BxesWriteContext>>,
) -> Result<usize, BxesWriteError> {
    let mut value_attributes_count = 0usize;
    let mut attrs_to_write = vec![];

    let map = if let Some(attributes) = event.attributes.as_ref() {
        let mut map = HashMap::new();
        for event_attribute in attributes {
            if let BxesValue::String(string) = event_attribute.0.as_ref().as_ref() {
                map.insert(string.as_ref().as_ref(), &event_attribute.1);
            }
        }

        Some(map)
    } else {
        None
    };

    if let Some(value_attributes) = context.borrow().value_attributes.as_ref() {
        for value_attribute in value_attributes {
            if let Some(map) = map.as_ref() {
                if let Some(found_attribute) = map.get(&value_attribute.name) {
                    attrs_to_write.push(found_attribute.as_ref().as_ref());
                    value_attributes_count += 1;
                    continue;
                }
            }

            attrs_to_write.push(&BxesValue::Null);
        }
    }

    for attr in &attrs_to_write {
        try_write_value(&mut context.borrow_mut(), attr)?;
    }

    Ok(value_attributes_count)
}

fn is_value_attribute(
    attribute: &(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>),
    desc: &ValueAttributeDescriptor,
) -> bool {
    let key = string_or_err(&attribute.0.as_ref().as_ref()).ok().unwrap();
    key.as_ref().as_ref() == &desc.name && get_type_id(&attribute.1) == desc.type_id
}

pub fn try_write_log_metadata(
    log: &BxesEventLog,
    context: Rc<RefCell<BxesWriteContext>>,
) -> Result<(), BxesWriteError> {
    try_write_properties(context.clone(), log.metadata.properties.as_ref())?;
    try_write_extensions(context.clone(), log.metadata.extensions.as_ref())?;
    try_write_globals(context.clone(), log.metadata.globals.as_ref())?;
    try_write_classifiers(context.clone(), log.metadata.classifiers.as_ref())
}

struct BinaryWriterWrapper<'a, 'b> {
    writer: &'a mut BinaryWriter<'b>,
}

impl<'a, 'b> Write for BinaryWriterWrapper<'a, 'b> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self.writer.write_bytes(buf) {
            Ok(written) => Ok(written),
            Err(err) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                err.to_string(),
            )),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<'a, 'b> BinaryWriterWrapper<'a, 'b> {
    pub fn new(writer: &'a mut BinaryWriter<'b>) -> Self {
        Self { writer }
    }
}

pub fn try_write_leb_128<'a>(writer: &mut BinaryWriter, value: u32) -> Result<(), BxesWriteError> {
    let mut wrapper = BinaryWriterWrapper::new(writer);

    match leb128::write::unsigned(&mut wrapper, value as u64) {
        Ok(_) => Ok(()),
        Err(err) => Err(BxesWriteError::LebWriteError(err.to_string())),
    }
}

pub fn try_write_properties(
    context: Rc<RefCell<BxesWriteContext>>,
    properties: Option<&Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)>>,
) -> Result<(), BxesWriteError> {
    write_collection_and_count(context.clone(), false, count(properties), || {
        if let Some(properties) = properties {
            for property in properties {
                try_write_kv_index(
                    context.clone(),
                    &(property.0.clone(), property.1.clone()),
                    false,
                )?;
            }
        }

        Ok(())
    })
}

fn count<T>(vec: Option<&Vec<T>>) -> u32 {
    if let Some(vec) = vec {
        vec.len() as u32
    } else {
        0
    }
}

pub fn try_write_globals(
    context: Rc<RefCell<BxesWriteContext>>,
    globals: Option<&Vec<BxesGlobal>>,
) -> Result<(), BxesWriteError> {
    write_collection_and_count(context.clone(), false, count(globals), || {
        if let Some(globals) = globals {
            for global in globals {
                try_write_enum_value_no_type_index(
                    context.borrow_mut().writer.as_mut().unwrap(),
                    &global.entity_kind,
                )?;

                write_collection_and_count(
                    context.clone(),
                    false,
                    global.globals.len() as u32,
                    || {
                        for global in &global.globals {
                            try_write_kv_index(
                                context.clone(),
                                &(global.0.clone(), global.1.clone()),
                                false,
                            )?;
                        }

                        Ok(())
                    },
                )?;
            }
        }

        Ok(())
    })
}

pub fn try_write_kv_index(
    context: Rc<RefCell<BxesWriteContext>>,
    kv: &(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>),
    write_leb_128: bool,
) -> Result<(), BxesWriteError> {
    if !context.borrow().kv_indices.borrow().contains_key(kv) {
        Err(BxesWriteError::FailedToFindKeyValueIndex((
            kv.0.clone(),
            kv.1.clone(),
        )))
    } else {
        let index = *context.borrow().kv_indices.borrow().get(kv).unwrap() as u32;

        if write_leb_128 {
            try_write_leb_128(context.borrow_mut().writer.as_mut().unwrap(), index)
        } else {
            try_write_u32_no_type_id(context.borrow_mut().writer.as_mut().unwrap(), index)
        }
    }
}

pub fn try_write_extensions(
    context: Rc<RefCell<BxesWriteContext>>,
    extensions: Option<&Vec<BxesExtension>>,
) -> Result<(), BxesWriteError> {
    write_collection_and_count(context.clone(), false, count(extensions), || {
        if let Some(extensions) = extensions {
            for extension in extensions {
                try_write_value_index(context.clone(), extension.name.clone())?;
                try_write_value_index(context.clone(), extension.prefix.clone())?;
                try_write_value_index(context.clone(), extension.uri.clone())?;
            }
        }

        Ok(())
    })
}

pub fn try_write_classifiers(
    context: Rc<RefCell<BxesWriteContext>>,
    classifiers: Option<&Vec<BxesClassifier>>,
) -> Result<(), BxesWriteError> {
    write_collection_and_count(context.clone(), false, count(classifiers), || {
        if let Some(classifiers) = classifiers {
            for classifier in classifiers {
                try_write_value_index(context.clone(), classifier.name.clone())?;
                write_collection_and_count(
                    context.clone(),
                    false,
                    classifier.keys.len() as u32,
                    || {
                        for key in &classifier.keys {
                            try_write_value_index(context.clone(), key.clone())?;
                        }

                        Ok(())
                    },
                )?;
            }
        }

        Ok(())
    })
}

fn try_write_value_index(
    context: Rc<RefCell<BxesWriteContext>>,
    value: Rc<Box<BxesValue>>,
) -> Result<(), BxesWriteError> {
    let exists = context
        .borrow()
        .values_indices
        .borrow()
        .contains_key(&value);

    if !exists {
        Err(BxesWriteError::FailedToFindValueIndex(value.clone()))
    } else {
        let index = *context
            .borrow()
            .values_indices
            .borrow()
            .get(&value)
            .unwrap() as u32;

        try_write_u32_no_type_id(context.borrow_mut().writer.as_mut().unwrap(), index)
    }
}

pub fn try_write_attributes(
    context: Rc<RefCell<BxesWriteContext>>,
    attributes: Option<&Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)>>,
    write_leb_128_count: bool,
) -> Result<(), BxesWriteError> {
    write_collection_and_count(
        context.clone(),
        write_leb_128_count,
        count(attributes),
        || {
            if let Some(attributes) = attributes {
                for (key, value) in attributes {
                    try_write_kv_index(
                        context.clone(),
                        &(key.clone(), value.clone()),
                        write_leb_128_count,
                    )?;
                }
            }

            Ok(())
        },
    )
}

pub fn try_write_key_values(
    log: &BxesEventLog,
    context: Rc<RefCell<BxesWriteContext>>,
) -> Result<(), BxesWriteError> {
    write_collection_and_count_after(context.clone(), || {
        execute_with_kv_pairs(log, |value| {
            match value {
                ValueOrKeyValue::Value(_) => {}
                ValueOrKeyValue::KeyValue((key, value)) => {
                    let exists = context
                        .borrow()
                        .kv_indices
                        .borrow()
                        .contains_key(&(key.clone(), value.clone()));

                    if !exists {
                        let count = context.borrow().kv_indices.borrow().len();
                        let key_index = *context.borrow().values_indices.borrow().get(key).unwrap();
                        let value_index =
                            *context.borrow().values_indices.borrow().get(value).unwrap();

                        try_write_leb_128(
                            context.borrow_mut().writer.as_mut().unwrap(),
                            key_index as u32,
                        )?;

                        try_write_leb_128(
                            context.borrow_mut().writer.as_mut().unwrap(),
                            value_index as u32,
                        )?;

                        context
                            .borrow_mut()
                            .kv_indices
                            .borrow_mut()
                            .insert((key.clone(), value.clone()), count);
                    }
                }
            }

            Ok(())
        })?;

        Ok(context.borrow().kv_indices.borrow().len() as u32)
    })
}

pub enum ValueOrKeyValue<'a> {
    Value(&'a Rc<Box<BxesValue>>),
    KeyValue((&'a Rc<Box<BxesValue>>, &'a Rc<Box<BxesValue>>)),
}

fn execute_with_kv_pairs<'a>(
    log: &'a BxesEventLog,
    mut action: impl FnMut(ValueOrKeyValue<'a>) -> Result<(), BxesWriteError>,
) -> Result<(), BxesWriteError> {
    if let Some(properties) = log.metadata.properties.as_ref() {
        execute_with_attributes_kv_pairs(properties, &mut action)?;
    }

    if let Some(extensions) = log.metadata.extensions.as_ref() {
        for extension in extensions {
            action(ValueOrKeyValue::Value(&extension.name))?;
            action(ValueOrKeyValue::Value(&extension.prefix))?;
            action(ValueOrKeyValue::Value(&extension.uri))?;
        }
    }

    if let Some(globals) = log.metadata.globals.as_ref() {
        for global in globals {
            execute_with_attributes_kv_pairs(&global.globals, &mut action)?;
        }
    }

    if let Some(classifiers) = log.metadata.classifiers.as_ref() {
        for classifier in classifiers {
            action(ValueOrKeyValue::Value(&classifier.name))?;

            for key in &classifier.keys {
                action(ValueOrKeyValue::Value(&key))?;
            }
        }
    }

    for variant in &log.variants {
        execute_with_attributes_kv_pairs(&variant.metadata, &mut action)?;

        for event in &variant.events {
            action(ValueOrKeyValue::Value(&event.name))?;
            if let Some(attributes) = event.attributes.as_ref() {
                execute_with_attributes_kv_pairs(attributes, &mut action)?;
            }
        }
    }

    Ok(())
}

fn execute_with_attributes_kv_pairs<'a>(
    attributes: &'a Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)>,
    action: &mut impl FnMut(ValueOrKeyValue<'a>) -> Result<(), BxesWriteError>,
) -> Result<(), BxesWriteError> {
    for (key, value) in attributes {
        action(ValueOrKeyValue::Value(&key))?;
        action(ValueOrKeyValue::Value(&value))?;

        action(ValueOrKeyValue::KeyValue((&key, &value)))?;
    }

    Ok(())
}

pub fn try_write_version(writer: &mut BinaryWriter, version: u32) -> Result<(), BxesWriteError> {
    try_write_u32_no_type_id(writer, version)
}

pub fn try_write_values(
    log: &BxesEventLog,
    context: Rc<RefCell<BxesWriteContext>>,
) -> Result<(), BxesWriteError> {
    write_collection_and_count_after(context.clone(), || {
        execute_with_kv_pairs(log, |value| {
            match value {
                ValueOrKeyValue::Value(value) => {
                    try_write_value_if_not_present(value, &mut context.borrow_mut())?;
                }
                ValueOrKeyValue::KeyValue(_) => {}
            }

            Ok(())
        })?;

        Ok(context.borrow().values_indices.borrow().len() as u32)
    })
}

fn write_collection_and_count(
    context: Rc<RefCell<BxesWriteContext>>,
    write_leb_128_count: bool,
    count: u32,
    mut writer_action: impl FnMut() -> Result<(), BxesWriteError>,
) -> Result<(), BxesWriteError> {
    if write_leb_128_count {
        try_write_leb_128(context.borrow_mut().writer.as_mut().unwrap(), count)?
    } else {
        try_write_u32_no_type_id(context.borrow_mut().writer.as_mut().unwrap(), count)?;
    }

    writer_action()
}

fn write_collection_and_count_after(
    context: Rc<RefCell<BxesWriteContext>>,
    mut writer_action: impl FnMut() -> Result<u32, BxesWriteError>,
) -> Result<(), BxesWriteError> {
    let pos = try_tell_pos(context.borrow_mut().writer.as_mut().unwrap())?;

    try_write_u32_no_type_id(context.borrow_mut().writer.as_mut().unwrap(), 0)?;

    let count = writer_action()?;

    let current_pos = try_tell_pos(context.borrow_mut().writer.as_mut().unwrap())?;
    try_seek(context.borrow_mut().writer.as_mut().unwrap(), pos)?;

    try_write_u32_no_type_id(context.borrow_mut().writer.as_mut().unwrap(), count)?;

    try_seek(context.borrow_mut().writer.as_mut().unwrap(), current_pos)
}

fn try_seek(writer: &mut BinaryWriter, pos: usize) -> Result<(), BxesWriteError> {
    match writer.seek(pos) {
        Ok(_) => Ok(()),
        Err(err) => Err(BxesWriteError::FailedToSeek(err.to_string())),
    }
}

fn try_tell_pos(writer: &mut BinaryWriter) -> Result<usize, BxesWriteError> {
    match writer.tell() {
        Ok(pos) => Ok(pos),
        Err(err) => Err(BxesWriteError::FailedToGetWriterPosition(err.to_string())),
    }
}

pub fn try_write_value_if_not_present(
    value: &Rc<Box<BxesValue>>,
    context: &mut BxesWriteContext,
) -> Result<bool, BxesWriteError> {
    if context.values_indices.borrow().contains_key(value) {
        return Ok(false);
    }

    try_write_value(context, value.as_ref().as_ref())?;

    let len = context.values_indices.borrow().len();
    context
        .values_indices
        .borrow_mut()
        .insert(value.clone(), len);

    Ok(true)
}

fn try_write_value(
    context: &mut BxesWriteContext,
    value: &BxesValue,
) -> Result<(), BxesWriteError> {
    match value {
        BxesValue::Null => try_write_u8_no_type_id(context.writer.as_mut().unwrap(), 0),
        BxesValue::Int32(value) => try_write_i32(context.writer.as_mut().unwrap(), *value),
        BxesValue::Int64(value) => try_write_i64(context.writer.as_mut().unwrap(), *value),
        BxesValue::Uint32(value) => try_write_u32(context.writer.as_mut().unwrap(), *value),
        BxesValue::Uint64(value) => try_write_u64(context.writer.as_mut().unwrap(), *value),
        BxesValue::Float32(value) => try_write_f32(context.writer.as_mut().unwrap(), *value),
        BxesValue::Float64(value) => try_write_f64(context.writer.as_mut().unwrap(), *value),
        BxesValue::String(value) => {
            try_write_string(context.writer.as_mut().unwrap(), value.as_str())
        }
        BxesValue::Bool(value) => try_write_bool(context.writer.as_mut().unwrap(), *value),
        BxesValue::Timestamp(value) => {
            try_write_timestamp(context.writer.as_mut().unwrap(), *value)
        }
        BxesValue::BrafLifecycle(value) => {
            try_write_braf_lifecycle(context.writer.as_mut().unwrap(), value)
        }
        BxesValue::StandardLifecycle(value) => {
            try_write_standard_lifecycle(context.writer.as_mut().unwrap(), value)
        }
        BxesValue::Artifact(artifacts) => try_write_artifact(context, artifacts),
        BxesValue::Drivers(drivers) => try_write_drivers(context, drivers),
        BxesValue::Guid(guid) => try_write_guid(context.writer.as_mut().unwrap(), guid),
        BxesValue::SoftwareEventType(value) => {
            try_write_software_event_type(context.writer.as_mut().unwrap(), value)
        }
    }
}

pub fn try_write_software_event_type(
    writer: &mut BinaryWriter,
    value: &SoftwareEventType,
) -> Result<(), BxesWriteError> {
    try_write_enum_value(writer, &TypeIds::SoftwareEventType, value)
}

pub fn try_write_guid(writer: &mut BinaryWriter, guid: &uuid::Uuid) -> Result<(), BxesWriteError> {
    try_write(|| {
        writer.write_u8(get_type_id_byte(TypeIds::Guid))?;
        writer.write_bytes(guid.to_bytes_le())
    })
}

fn get_type_id_byte(type_id: TypeIds) -> u8 {
    TypeIds::to_u8(&type_id).unwrap()
}

pub fn try_write_artifact(
    context: &mut BxesWriteContext,
    artifact: &BxesArtifact,
) -> Result<(), BxesWriteError> {
    for artifact in &artifact.items {
        get_or_write_value_index(&artifact.model, context)?;
        get_or_write_value_index(&artifact.instance, context)?;
        get_or_write_value_index(&artifact.transition, context)?;
    }

    try_write_u8_no_type_id(
        context.writer.as_mut().unwrap(),
        get_type_id_byte(TypeIds::Artifact),
    )?;

    try_write_u32_no_type_id(
        context.writer.as_mut().unwrap(),
        artifact.items.len() as u32,
    )?;

    for artifact in &artifact.items {
        let index = get_index(&artifact.model, context)?;
        try_write_u32_no_type_id(context.writer.as_mut().unwrap(), index as u32)?;

        let index = get_index(&artifact.instance, context)?;
        try_write_u32_no_type_id(context.writer.as_mut().unwrap(), index as u32)?;

        let index = get_index(&artifact.transition, context)?;
        try_write_u32_no_type_id(context.writer.as_mut().unwrap(), index as u32)?;
    }

    Ok(())
}

fn get_index(
    value: &Rc<Box<BxesValue>>,
    context: &mut BxesWriteContext,
) -> Result<u32, BxesWriteError> {
    if let Some(index) = context.values_indices.borrow().get(value) {
        return Ok(*index as u32);
    }

    Err(BxesWriteError::FailedToFindValueIndex(value.clone()))
}

fn get_or_write_value_index(
    value: &Rc<Box<BxesValue>>,
    context: &mut BxesWriteContext,
) -> Result<u32, BxesWriteError> {
    try_write_value_if_not_present(value, context)?;
    let index = *context.values_indices.borrow().get(value).unwrap() as u32;

    return Ok(index);
}

pub fn try_write_drivers(
    context: &mut BxesWriteContext,
    drivers: &BxesDrivers,
) -> Result<(), BxesWriteError> {
    for driver in &drivers.drivers {
        get_or_write_value_index(&driver.name, context)?;
        get_or_write_value_index(&driver.driver_type, context)?;
    }

    try_write_u8_no_type_id(
        context.writer.as_mut().unwrap(),
        get_type_id_byte(TypeIds::Drivers),
    )?;

    try_write_u32_no_type_id(
        context.writer.as_mut().unwrap(),
        drivers.drivers.len() as u32,
    )?;

    for driver in &drivers.drivers {
        try_write_f64_no_type_id(context.writer.as_mut().unwrap(), driver.amount())?;

        let index = get_index(&driver.name, context)?;
        try_write_u32_no_type_id(context.writer.as_mut().unwrap(), index)?;

        let index = get_index(&driver.driver_type, context)?;
        try_write_u32_no_type_id(context.writer.as_mut().unwrap(), index)?;
    }

    Ok(())
}

pub fn try_write_i32(writer: &mut BinaryWriter, value: i32) -> Result<(), BxesWriteError> {
    try_write(|| {
        writer.write_u8(get_type_id_byte(TypeIds::I32))?;
        writer.write_i32(value)
    })
}

pub fn try_write_i64_no_type_id(
    writer: &mut BinaryWriter,
    value: i64,
) -> Result<(), BxesWriteError> {
    try_write(|| writer.write_i64(value))
}

pub fn try_write_i64(writer: &mut BinaryWriter, value: i64) -> Result<(), BxesWriteError> {
    try_write(|| {
        writer.write_u8(get_type_id_byte(TypeIds::I64))?;
        writer.write_i64(value)
    })
}

pub fn try_write_u32_no_type_id(
    writer: &mut BinaryWriter,
    value: u32,
) -> Result<(), BxesWriteError> {
    try_write(|| writer.write_u32(value))
}

pub fn try_write_u32(writer: &mut BinaryWriter, value: u32) -> Result<(), BxesWriteError> {
    try_write(|| {
        writer.write_u8(get_type_id_byte(TypeIds::U32))?;
        writer.write_u32(value)
    })
}

pub fn try_write_u64(writer: &mut BinaryWriter, value: u64) -> Result<(), BxesWriteError> {
    try_write(|| {
        writer.write_u8(get_type_id_byte(TypeIds::U64))?;
        writer.write_u64(value)
    })
}

pub fn try_write_f32(writer: &mut BinaryWriter, value: f32) -> Result<(), BxesWriteError> {
    try_write(|| {
        writer.write_u8(get_type_id_byte(TypeIds::F32))?;
        writer.write_f32(value)
    })
}

pub fn try_write_u8_no_type_id(writer: &mut BinaryWriter, value: u8) -> Result<(), BxesWriteError> {
    try_write(|| writer.write_u8(value))
}

pub fn try_write_f64(writer: &mut BinaryWriter, value: f64) -> Result<(), BxesWriteError> {
    try_write(|| {
        writer.write_u8(get_type_id_byte(TypeIds::F64))?;
        writer.write_f64(value)
    })
}

pub fn try_write_f64_no_type_id(
    writer: &mut BinaryWriter,
    value: f64,
) -> Result<(), BxesWriteError> {
    try_write(|| writer.write_f64(value))
}

pub fn try_write_bool(writer: &mut BinaryWriter, value: bool) -> Result<(), BxesWriteError> {
    try_write(|| {
        writer.write_u8(get_type_id_byte(TypeIds::Bool))?;
        writer.write_u8(if value { 1 } else { 0 })
    })
}

pub fn try_write_string(writer: &mut BinaryWriter, value: &str) -> Result<(), BxesWriteError> {
    try_write(|| {
        writer.write_u8(get_type_id_byte(TypeIds::String))?;
        writer.write_u64(value.len() as u64)?;
        writer.write_bytes(value.as_bytes())
    })
}

pub fn try_write_lifecycle(
    writer: &mut BinaryWriter,
    lifecycle: &Lifecycle,
) -> Result<(), BxesWriteError> {
    match lifecycle {
        Lifecycle::Braf(braf_lifecycle) => try_write_braf_lifecycle(writer, braf_lifecycle),
        Lifecycle::Standard(standard_lifecycle) => {
            try_write_standard_lifecycle(writer, standard_lifecycle)
        }
    }
}

pub fn try_write_braf_lifecycle(
    writer: &mut BinaryWriter,
    value: &BrafLifecycle,
) -> Result<(), BxesWriteError> {
    try_write_enum_value(writer, &TypeIds::BrafLifecycle, value)
}

fn try_write_enum_value_no_type_index<T: ToPrimitive>(
    writer: &mut BinaryWriter,
    value: &T,
) -> Result<(), BxesWriteError> {
    try_write(|| writer.write_u8(T::to_u8(value).unwrap()))
}

fn try_write_enum_value<T: ToPrimitive>(
    writer: &mut BinaryWriter,
    type_id: &TypeIds,
    value: &T,
) -> Result<(), BxesWriteError> {
    try_write(|| {
        writer.write_u8(TypeIds::to_u8(type_id).unwrap())?;
        writer.write_u8(T::to_u8(value).unwrap())
    })
}

pub fn try_write_standard_lifecycle(
    writer: &mut BinaryWriter,
    value: &StandardLifecycle,
) -> Result<(), BxesWriteError> {
    try_write_enum_value(writer, &TypeIds::StandardLifecycle, value)
}

pub fn try_write_timestamp(writer: &mut BinaryWriter, value: i64) -> Result<(), BxesWriteError> {
    try_write(|| {
        writer.write_u8(get_type_id_byte(TypeIds::Timestamp))?;
        writer.write_i64(value)
    })
}

fn try_write(
    mut write_func: impl FnMut() -> crate::binary_rw::core::Result<usize>,
) -> Result<(), BxesWriteError> {
    match write_func() {
        Ok(_) => Ok(()),
        Err(error) => Err(BxesWriteError::WriteError(error)),
    }
}

pub fn try_open_write(path: &str) -> Result<FileStream, BxesWriteError> {
    match FileStream::create(path) {
        Ok(stream) => Ok(stream),
        Err(err) => Err(BxesWriteError::FailedToOpenFileForWriting(err.to_string())),
    }
}

pub fn compress_to_archive(log_path: &str, save_path: &str) -> Result<(), BxesWriteError> {
    let file = File::create(save_path).or_else(|_| Err(BxesWriteError::FailedToCreateArchive))?;
    let mut zip_writer = ZipWriter::new(file);

    let archive_log_name = Path::new(save_path).file_name().unwrap().to_str().unwrap();
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .compression_level(Some(8));

    zip_writer
        .start_file(archive_log_name, options)
        .or_else(|_| Err(BxesWriteError::FailedToCreateArchive))?;

    let bytes = fs::read(log_path).unwrap();
    zip_writer
        .write_all(&bytes)
        .or_else(|_| Err(BxesWriteError::FailedToCreateArchive))?;

    zip_writer
        .flush()
        .or_else(|_| Err(BxesWriteError::FailedToCreateArchive))?;

    zip_writer
        .finish()
        .or_else(|_| Err(BxesWriteError::FailedToCreateArchive))?;

    Ok(())
}
