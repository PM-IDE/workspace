use crate::event_log::core::event::event::EventPayloadValue;
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone)]
pub struct XesEventLogExtension {
  pub name: Arc<str>,
  pub prefix: Arc<str>,
  pub uri: Arc<str>,
}

#[derive(Debug, Clone)]
pub struct XesGlobal {
  pub scope: Arc<str>,
  pub default_values: HashMap<Arc<str>, EventPayloadValue>,
}

#[derive(Debug, Clone)]
pub struct XesClassifier {
  pub name: Arc<str>,
  pub keys: Vec<Arc<str>>,
}

#[derive(Debug, Clone)]
pub struct XesProperty {
  pub name: Arc<str>,
  pub value: EventPayloadValue,
}
