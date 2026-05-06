use super::conversions::{parse_entity_kind, payload_value_to_bxes_value};
use crate::event_log::{
  core::{
    event::event::{Event, EventPayloadValue},
    event_log::EventLog,
    trace::trace::Trace,
  },
  xes::{constants::EVENT_TAG_NAME_STR, shared::XesEventLogExtension, xes_event::XesEventImpl, xes_event_log::XesEventLogImpl},
};
use bxes::{
  models::{
    domain::{
      bxes_event_log::{BxesEvent, BxesEventLog, BxesTraceVariant},
      bxes_log_metadata::{BxesClassifier, BxesEventLogMetadata, BxesExtension, BxesGlobal},
      bxes_value::BxesValue,
    },
    system_models::SystemMetadata,
  },
  writer::{
    errors::BxesWriteError,
    single_file_bxes_writer::{write_bxes, write_bxes_to_bytes},
    writer_utils::BxesLogWriteData,
  },
};
use std::{fmt::Display, sync::Arc};

pub enum XesToBxesWriterError {
  BxesWriteError(BxesWriteError),
  ConversionError(String),
}

impl Display for XesToBxesWriterError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(
      match self {
        XesToBxesWriterError::BxesWriteError(err) => err.to_string(),
        XesToBxesWriterError::ConversionError(err) => err.to_owned(),
      }
      .as_str(),
    )
  }
}

pub fn write_event_log_to_bxes_bytes(log: &XesEventLogImpl, metadata: Option<&SystemMetadata>) -> Result<Vec<u8>, XesToBxesWriterError> {
  let data = create_bxes_write_data(log, metadata)?;
  match write_bxes_to_bytes(&data) {
    Ok(bytes) => Ok(bytes),
    Err(error) => Err(XesToBxesWriterError::BxesWriteError(error)),
  }
}

pub fn write_event_log_to_bxes(log: &XesEventLogImpl, metadata: Option<&SystemMetadata>, path: &str) -> Result<(), XesToBxesWriterError> {
  let data = create_bxes_write_data(log, metadata)?;
  match write_bxes(path, &data) {
    Ok(()) => Ok(()),
    Err(error) => Err(XesToBxesWriterError::BxesWriteError(error)),
  }
}

fn create_bxes_write_data(log: &XesEventLogImpl, metadata: Option<&SystemMetadata>) -> Result<BxesLogWriteData, XesToBxesWriterError> {
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

  Ok(BxesLogWriteData {
    log: bxes_log,
    system_metadata: match metadata {
      Some(metadata) => metadata.clone(),
      None => SystemMetadata::new(None),
    },
  })
}

fn create_bxes_traces(log: &XesEventLogImpl) -> Vec<BxesTraceVariant> {
  log
    .traces()
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
  BxesEvent {
    name: Arc::new(BxesValue::String(event.name_pointer().clone())),
    timestamp: event.timestamp().timestamp_nanos_opt().expect("timestamp_nanos_opt"),
    attributes: Some(
      event
        .ordered_payload()
        .iter()
        .filter(|kv| is_not_default_attribute(log, kv))
        .map(kv_pair_to_bxes_pair)
        .collect(),
    ),
  }
}

fn is_not_default_attribute(log: &XesEventLogImpl, kv: &(&Arc<str>, &EventPayloadValue)) -> bool {
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
  log
    .classifiers()
    .iter()
    .map(|c| BxesClassifier {
      keys: c.keys.iter().map(|x| Arc::new(BxesValue::String(x.to_owned()))).collect(),
      name: Arc::new(BxesValue::String(c.name.to_owned())),
    })
    .collect()
}

fn create_bxes_extensions(log: &XesEventLogImpl) -> Vec<BxesExtension> {
  log.extensions().iter().map(convert_to_bxes_extension).collect()
}

fn convert_to_bxes_extension(e: &XesEventLogExtension) -> BxesExtension {
  BxesExtension {
    name: Arc::new(BxesValue::String(e.name.to_owned())),
    prefix: Arc::new(BxesValue::String(e.prefix.to_owned())),
    uri: Arc::new(BxesValue::String(e.uri.to_owned())),
  }
}

fn create_bxes_globals(log: &XesEventLogImpl) -> Result<Vec<BxesGlobal>, XesToBxesWriterError> {
  let mut globals = vec![];
  for xes_global in log.ordered_globals().iter() {
    globals.push(BxesGlobal {
      entity_kind: parse_entity_kind(xes_global.0.as_ref())?,
      globals: xes_global.1.iter().map(convert_to_bxes_global_attribute).collect(),
    })
  }

  Ok(globals)
}

fn convert_to_bxes_global_attribute(kv: &(&Arc<str>, &EventPayloadValue)) -> (Arc<BxesValue>, Arc<BxesValue>) {
  let key = Arc::new(BxesValue::String(kv.0.clone()));
  let value = Arc::new(payload_value_to_bxes_value(kv.1));

  (key, value)
}

fn create_bxes_properties(log: &XesEventLogImpl) -> Vec<(Arc<BxesValue>, Arc<BxesValue>)> {
  log
    .properties_map()
    .iter()
    .map(|kv| kv_pair_to_bxes_pair(&(&kv.name, &kv.value)))
    .collect()
}

fn kv_pair_to_bxes_pair(kv: &(&Arc<str>, &EventPayloadValue)) -> (Arc<BxesValue>, Arc<BxesValue>) {
  let bxes_value = payload_value_to_bxes_value(kv.1);
  let key = Arc::new(BxesValue::String(kv.0.clone()));

  (key, Arc::new(bxes_value))
}
