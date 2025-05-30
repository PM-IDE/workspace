use super::{
  reader::file_xes_log_reader::XesEventLogItem,
  shared::{XesClassifier, XesEventLogExtension, XesProperty},
  xes_event::XesEventImpl,
  xes_trace::XesTraceImpl,
};

use crate::event_log::core::{
  event::{event::EventPayloadValue, event_hasher::EventHasher, events_holder::EventSequenceInfo},
  event_log::EventLog,
  trace::traces_holder::EventLogBase,
};
use crate::utils::user_data::user_data::UserDataImpl;
use crate::utils::user_data::user_data::UserDataOwner;
use crate::utils::vec_utils;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct XesEventLogImpl {
  base: EventLogBase<XesTraceImpl>,
  globals: HashMap<String, HashMap<String, EventPayloadValue>>,
  extensions: Vec<XesEventLogExtension>,
  classifiers: Vec<XesClassifier>,
  properties: Vec<XesProperty>,
}

impl XesEventLogImpl {
  pub fn globals_map(&self) -> &HashMap<String, HashMap<String, EventPayloadValue>> {
    &self.globals
  }

  pub fn extensions(&self) -> &Vec<XesEventLogExtension> {
    &self.extensions
  }

  pub fn classifiers(&self) -> &Vec<XesClassifier> {
    &self.classifiers
  }

  pub fn properties_map(&self) -> &Vec<XesProperty> {
    &self.properties
  }

  pub fn globals_mut(&mut self) -> &mut HashMap<String, HashMap<String, EventPayloadValue>> {
    &mut self.globals
  }

  pub fn extensions_mut(&mut self) -> &mut Vec<XesEventLogExtension> {
    &mut self.extensions
  }

  pub fn properties_mut(&mut self) -> &mut Vec<XesProperty> {
    &mut self.properties
  }

  pub fn classifiers_mut(&mut self) -> &mut Vec<XesClassifier> {
    &mut self.classifiers
  }

  pub fn ordered_properties(&self) -> Vec<(&String, &EventPayloadValue)> {
    let mut properties = Vec::new();
    for property in self.properties_map() {
      properties.push((&property.name, &property.value));
    }

    vec_utils::sort_by_first(&mut properties);
    properties
  }

  pub fn ordered_globals(&self) -> Vec<(&String, Vec<(&String, &EventPayloadValue)>)> {
    let mut globals = Vec::new();
    for (key, value) in self.globals_map() {
      let mut defaults = Vec::new();
      for (default_key, default_value) in value {
        defaults.push((default_key, default_value));
      }

      vec_utils::sort_by_first(&mut defaults);
      globals.push((key, defaults));
    }

    vec_utils::sort_by_first(&mut globals);
    globals
  }
}

impl XesEventLogImpl {
  pub fn new<'a, TLogReader>(event_log_reader: &'a mut TLogReader) -> Option<XesEventLogImpl>
  where
    TLogReader: Iterator<Item=XesEventLogItem<'a>>,
  {
    let mut extensions = Vec::new();
    let mut globals = HashMap::new();
    let mut traces = Vec::new();
    let mut classifiers = Vec::new();
    let mut properties = Vec::new();

    for item in event_log_reader {
      match item {
        XesEventLogItem::Trace(trace_reader) => match XesTraceImpl::new(trace_reader) {
          Some(trace) => traces.push(Rc::new(RefCell::new(trace))),
          None => continue,
        },
        XesEventLogItem::Global(global) => _ = globals.insert(global.scope, global.default_values),
        XesEventLogItem::Extension(extension) => extensions.push(extension),
        XesEventLogItem::Classifier(classifier) => classifiers.push(classifier),
        XesEventLogItem::Property(property) => _ = properties.push(property),
      }
    }

    let log = XesEventLogImpl {
      base: EventLogBase::new(traces),
      globals,
      extensions,
      classifiers,
      properties,
    };

    Some(log)
  }
}

impl Clone for XesEventLogImpl {
  fn clone(&self) -> Self {
    Self {
      base: self.base.clone(),
      globals: self.globals.clone(),
      extensions: self.extensions.clone(),
      classifiers: self.classifiers.clone(),
      properties: self.properties.clone(),
    }
  }
}

impl UserDataOwner for XesEventLogImpl {
  fn user_data(&self) -> &UserDataImpl {
    self.base.user_data()
  }

  fn user_data_mut(&mut self) -> &mut UserDataImpl {
    self.base.user_data_mut()
  }
}

impl EventLog for XesEventLogImpl {
  type TEvent = XesEventImpl;
  type TTraceInfo = EventSequenceInfo;
  type TTrace = XesTraceImpl;

  fn empty() -> Self {
    Self {
      base: EventLogBase::empty(),
      globals: HashMap::new(),
      extensions: Vec::new(),
      classifiers: Vec::new(),
      properties: Vec::new(),
    }
  }

  fn traces(&self) -> &Vec<Rc<RefCell<Self::TTrace>>> {
    &self.base.get_traces()
  }

  fn push(&mut self, trace: Rc<RefCell<Self::TTrace>>) {
    self.base.push(trace);
  }

  fn to_raw_vector(&self) -> Vec<Vec<String>> {
    self.base.to_raw_vector()
  }

  fn to_hashes_event_log<THasher>(&self, hasher: &THasher) -> Vec<Vec<u64>>
  where
    THasher: EventHasher<Self::TEvent>,
  {
    self.base.to_hashes_vectors(hasher)
  }

  fn filter_events_by<TPred>(&mut self, predicate: TPred)
  where
    TPred: Fn(&Self::TEvent) -> bool,
  {
    self.base.filter_events_by(predicate);
  }

  fn mutate_events<TMutator>(&mut self, mutator: TMutator)
  where
    TMutator: Fn(&mut Self::TEvent),
  {
    self.base.mutate_events(mutator);
  }

  fn filter_traces(&mut self, predicate: &impl Fn(&Self::TTrace, &usize) -> bool) {
    self.base.filter_traces(predicate);
  }
}
