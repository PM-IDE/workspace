use std::collections::HashMap;
use std::rc::Rc;
use crate::event_log::core::event::event::EventPayloadValue;

#[derive(Debug, Clone)]
pub struct XesEventLogExtension {
  pub name: Rc<str>,
  pub prefix: Rc<str>,
  pub uri: Rc<str>,
}

#[derive(Debug, Clone)]
pub struct XesGlobal {
  pub scope: Rc<str>,
  pub default_values: HashMap<Rc<str>, EventPayloadValue>,
}

#[derive(Debug, Clone)]
pub struct XesClassifier {
  pub name: Rc<str>,
  pub keys: Vec<Rc<str>>,
}

#[derive(Debug, Clone)]
pub struct XesProperty {
  pub name: Rc<str>,
  pub value: EventPayloadValue,
}
