use std::rc::Rc;

use bxes::models::domain::bxes_event_log::{BxesEvent, BxesEventLog, BxesTraceVariant};
use bxes::models::domain::bxes_log_metadata::{BxesClassifier, BxesEventLogMetadata, BxesExtension, BxesGlobal};
use bxes::models::domain::bxes_value::BxesValue;
use bxes::models::system_models::SystemMetadata;
use bxes::writer::writer_utils::BxesLogWriteData;
use bxes::writer::{errors::BxesWriteError, single_file_bxes_writer::write_bxes};

use crate::event_log::{
    core::{
        event::event::{Event, EventPayloadValue},
        event_log::EventLog,
        trace::trace::Trace,
    },
    xes::{constants::EVENT_TAG_NAME_STR, shared::XesEventLogExtension, xes_event::XesEventImpl, xes_event_log::XesEventLogImpl},
};

use crate::utils::user_data::user_data::UserDataOwner;

use super::conversions::{parse_entity_kind, payload_value_to_bxes_value};

pub enum XesToBxesWriterError {
    BxesWriteError(BxesWriteError),
    ConversionError(String),
}

impl ToString for XesToBxesWriterError {
    fn to_string(&self) -> String {
        match self {
            XesToBxesWriterError::BxesWriteError(err) => err.to_string(),
            XesToBxesWriterError::ConversionError(err) => err.to_owned(),
        }
    }
}

pub fn write_event_log_to_bxes(log: &XesEventLogImpl, metadata: Option<&SystemMetadata>, path: &str) -> Result<(), XesToBxesWriterError> {
    let bxes_log = BxesEventLog {
        metadata: BxesEventLogMetadata {
            classifiers: Some(create_bxes_classifiers(log)),
            extensions: Some(create_bxes_extensions(log)),
            globals: Some(create_bxes_globals(log)?),
            properties: Some(create_bxes_properties(log)),
        },
        variants: create_bxes_traces(log),
        version: 1,
    };

    let data = BxesLogWriteData {
        log: bxes_log,
        system_metadata: match metadata {
            Some(metadata) => metadata.clone(),
            None => SystemMetadata::new(None),
        },
    };

    match write_bxes(path, &data) {
        Ok(()) => Ok(()),
        Err(error) => Err(XesToBxesWriterError::BxesWriteError(error)),
    }
}

fn create_bxes_traces(log: &XesEventLogImpl) -> Vec<BxesTraceVariant> {
    log.traces()
        .iter()
        .map(|trace| BxesTraceVariant {
            traces_count: 1,
            metadata: vec![],
            events: trace
                .borrow()
                .events()
                .iter()
                .map(|event| create_bxes_event(log, &event.borrow()))
                .collect(),
        })
        .collect()
}

fn create_bxes_event(log: &XesEventLogImpl, event: &XesEventImpl) -> BxesEvent {
    let bxes_event = BxesEvent {
        name: Rc::new(Box::new(BxesValue::String(event.name_pointer().clone()))),
        timestamp: event.timestamp().timestamp_nanos(),
        attributes: Some(
            event
                .ordered_payload()
                .iter()
                .filter(|kv| is_not_default_attribute(log, kv))
                .map(|kv| kv_pair_to_bxes_pair(kv))
                .collect(),
        ),
    };

    bxes_event
}

fn is_not_default_attribute(log: &XesEventLogImpl, kv: &(&String, &EventPayloadValue)) -> bool {
    if let Some(event_globals) = log.globals_map().get(EVENT_TAG_NAME_STR) {
        if let Some(default_value) = event_globals.get(kv.0) {
            default_value != kv.1
        } else {
            true
        }
    } else {
        true
    }
}

fn create_bxes_classifiers(log: &XesEventLogImpl) -> Vec<BxesClassifier> {
    log.classifiers()
        .iter()
        .map(|c| BxesClassifier {
            keys: c
                .keys
                .iter()
                .map(|x| Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(x.to_owned()))))))
                .collect(),
            name: Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(c.name.to_owned()))))),
        })
        .collect()
}

fn create_bxes_extensions(log: &XesEventLogImpl) -> Vec<BxesExtension> {
    log.extensions().iter().map(|e| convert_to_bxes_extension(e)).collect()
}

fn convert_to_bxes_extension(e: &XesEventLogExtension) -> BxesExtension {
    BxesExtension {
        name: Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(e.name.to_owned()))))),
        prefix: Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(e.prefix.to_owned()))))),
        uri: Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(e.uri.to_owned()))))),
    }
}

fn create_bxes_globals(log: &XesEventLogImpl) -> Result<Vec<BxesGlobal>, XesToBxesWriterError> {
    let mut globals = vec![];
    for xes_global in log.ordered_globals().iter() {
        globals.push(BxesGlobal {
            entity_kind: parse_entity_kind(xes_global.0.as_str())?,
            globals: xes_global.1.iter().map(|kv| convert_to_bxes_global_attribute(kv)).collect(),
        })
    }

    Ok(globals)
}

fn convert_to_bxes_global_attribute(kv: &(&String, &EventPayloadValue)) -> (Rc<Box<BxesValue>>, Rc<Box<BxesValue>>) {
    let key = Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(kv.0.to_owned())))));
    let value = Rc::new(Box::new(payload_value_to_bxes_value(kv.1)));

    (key, value)
}

fn create_bxes_properties(log: &XesEventLogImpl) -> Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)> {
    log.properties_map()
        .iter()
        .map(|kv| kv_pair_to_bxes_pair(&(&kv.name, &kv.value)))
        .collect()
}

fn kv_pair_to_bxes_pair(kv: &(&String, &EventPayloadValue)) -> (Rc<Box<BxesValue>>, Rc<Box<BxesValue>>) {
    let bxes_value = payload_value_to_bxes_value(kv.1);
    let key = Rc::new(Box::new(BxesValue::String(Rc::new(Box::new(kv.0.to_owned())))));

    (key, Rc::new(Box::new(bxes_value)))
}
