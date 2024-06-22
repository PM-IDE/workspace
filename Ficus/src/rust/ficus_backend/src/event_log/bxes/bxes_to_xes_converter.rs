use std::{borrow::Borrow, cell::RefCell, collections::HashMap, rc::Rc};

use bxes::models::domain::bxes_event_log::{BxesEvent, BxesEventLog};
use bxes::models::domain::bxes_value::BxesValue;
use bxes::models::system_models::SystemMetadata;
use bxes::read::errors::BxesReadError;
use chrono::{TimeZone, Utc};

use crate::event_log::{
    core::{event::event::EventPayloadValue, event_log::EventLog, trace::trace::Trace},
    xes::{
        shared::{XesClassifier, XesEventLogExtension, XesProperty},
        xes_event::XesEventImpl,
        xes_event_log::XesEventLogImpl,
        xes_trace::XesTraceImpl,
    },
};
use crate::utils::user_data::user_data::UserDataOwner;

use super::conversions::{bxes_value_to_payload_value, global_type_to_string};

pub enum BxesToXesReadError {
    BxesReadError(BxesReadError),
    ConversionError(String),
}

impl ToString for BxesToXesReadError {
    fn to_string(&self) -> String {
        match self {
            BxesToXesReadError::BxesReadError(err) => err.to_string(),
            BxesToXesReadError::ConversionError(err) => err.to_string(),
        }
    }
}

pub struct BxesToXesConversionResult {
    pub xes_log: XesEventLogImpl,
    pub system_metadata: SystemMetadata,
}

pub fn read_bxes_into_xes_log(path: &str) -> Result<BxesToXesConversionResult, BxesToXesReadError> {
    let result = match bxes::read::single_file_bxes_reader::read_bxes(path) {
        Ok(log) => log,
        Err(error) => return Err(BxesToXesReadError::BxesReadError(error)),
    };

    let mut xes_log = XesEventLogImpl::empty();

    set_classifiers(&mut xes_log, &result.log)?;
    set_properties(&mut xes_log, &result.log)?;
    set_extensions(&mut xes_log, &result.log)?;
    set_globals(&mut xes_log, &result.log)?;

    for variant in &result.log.variants {
        let mut xes_trace = XesTraceImpl::empty();
        for event in &variant.events {
            xes_trace.push(Rc::new(RefCell::new(create_xes_event(event)?)));
        }

        xes_log.push(Rc::new(RefCell::new(xes_trace)));
    }

    Ok(BxesToXesConversionResult {
        xes_log,
        system_metadata: result.system_metadata,
    })
}

fn set_classifiers(xes_log: &mut XesEventLogImpl, log: &BxesEventLog) -> Result<(), BxesToXesReadError> {
    set_metadata_vector_item(xes_log.classifiers_mut(), log.metadata.classifiers.as_ref(), |classifier| {
        Ok(XesClassifier {
            name: string_or_err(&classifier.name, "Classifier")?,
            keys: vector_of_strings_or_err(&classifier.keys, "Classifier key")?,
        })
    })
}

fn set_metadata_vector_item<TBxesItem, TXesItem>(
    target: &mut Vec<TXesItem>,
    given: Option<&Vec<TBxesItem>>,
    conversion: impl Fn(&TBxesItem) -> Result<TXesItem, BxesToXesReadError>,
) -> Result<(), BxesToXesReadError> {
    if let Some(given) = given {
        for given_entity in given {
            target.push(conversion(given_entity)?);
        }
    }

    Ok(())
}

fn set_properties(xes_log: &mut XesEventLogImpl, log: &BxesEventLog) -> Result<(), BxesToXesReadError> {
    set_metadata_vector_item(xes_log.properties_mut(), log.metadata.properties.as_ref(), |property| {
        Ok(XesProperty {
            name: string_or_err(&property.0, "Property key")?,
            value: bxes_value_to_payload_value(&property.1),
        })
    })
}

fn set_extensions(xes_log: &mut XesEventLogImpl, log: &BxesEventLog) -> Result<(), BxesToXesReadError> {
    set_metadata_vector_item(xes_log.extensions_mut(), log.metadata.extensions.as_ref(), |extension| {
        Ok(XesEventLogExtension {
            name: string_or_err(&extension.name, "Extension name")?,
            uri: string_or_err(&extension.uri, "Extension uri")?,
            prefix: string_or_err(&extension.prefix, "Extension prefix")?,
        })
    })
}

fn set_globals(xes_log: &mut XesEventLogImpl, log: &BxesEventLog) -> Result<(), BxesToXesReadError> {
    if let Some(globals) = log.metadata.globals.as_ref() {
        for global in globals {
            let global_type = global_type_to_string(&global.entity_kind);

            let mut globals_map = HashMap::new();
            for global_value in &global.globals {
                let key = string_or_err(&global_value.0, format!("{} global kv key", &global_type).as_str())?;
                globals_map.insert(key, bxes_value_to_payload_value(&global_value.1));
            }

            xes_log.globals_mut().insert(global_type, globals_map);
        }
    }

    Ok(())
}

fn vector_of_strings_or_err(values: &Vec<Rc<Box<BxesValue>>>, entity_name: &str) -> Result<Vec<String>, BxesToXesReadError> {
    let mut result = vec![];
    for value in values {
        result.push(string_or_err(value, entity_name)?)
    }

    Ok(result)
}

fn string_or_err(value: &BxesValue, entity_name: &str) -> Result<String, BxesToXesReadError> {
    if let BxesValue::String(string) = value {
        Ok(string.as_ref().as_ref().to_owned())
    } else {
        return Err(BxesToXesReadError::ConversionError(format!("{} key was not a string", entity_name)));
    }
}

fn create_xes_event(bxes_event: &BxesEvent) -> Result<XesEventImpl, BxesToXesReadError> {
    let name = if let BxesValue::String(string) = bxes_event.name.as_ref().as_ref() {
        string.clone()
    } else {
        let message = format!("The name of event was not a string: {:?}", bxes_event.name);
        return Err(BxesToXesReadError::ConversionError(message));
    };

    let timestamp = Utc.timestamp_nanos(bxes_event.timestamp);
    let payload = create_xes_payload(bxes_event.attributes.as_ref())?;

    Ok(XesEventImpl::new_all_fields(name, timestamp, payload))
}

fn create_xes_payload(
    attributes: Option<&Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)>>,
) -> Result<Option<HashMap<String, EventPayloadValue>>, BxesToXesReadError> {
    if let Some(attributes) = attributes {
        let mut payload = HashMap::new();

        for (key, value) in attributes {
            let key = if let BxesValue::String(string) = key.as_ref().as_ref() {
                string.as_ref().as_ref().to_owned()
            } else {
                let message = format!("The attribute key is not a string: {:?}", key);
                return Err(BxesToXesReadError::ConversionError(message));
            };

            payload.insert(key, bxes_value_to_payload_value(&value));
        }

        Ok(Some(payload))
    } else {
        Ok(None)
    }
}
