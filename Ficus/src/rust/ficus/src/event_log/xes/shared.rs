use std::collections::HashMap;

use crate::event_log::core::event::event::EventPayloadValue;

#[derive(Debug, Clone)]
pub struct XesEventLogExtension {
  pub name: String,
  pub prefix: String,
  pub uri: String,
}

#[derive(Debug, Clone)]
pub struct XesGlobal {
  pub scope: String,
  pub default_values: HashMap<String, EventPayloadValue>,
}

#[derive(Debug, Clone)]
pub struct XesClassifier {
  pub name: String,
  pub keys: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct XesProperty {
  pub name: String,
  pub value: EventPayloadValue,
}
