use super::xes_event::XesEventImpl;
use crate::event_log::core::event::event::EventPayloadValue;
use crate::event_log::core::{
  event::events_holder::{EventSequenceInfo, EventsHolder, EventsPositions},
  trace::trace::Trace,
};
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

pub struct XesTraceImpl {
  events_holder: EventsHolder<XesEventImpl>,
  metadata: HashMap<String, EventPayloadValue>,
}

impl XesTraceImpl {
  pub fn new<TTraceReader>(trace_reader: TTraceReader) -> Option<XesTraceImpl>
  where
    TTraceReader: Iterator<Item = XesEventImpl>,
  {
    let mut events: Vec<Rc<RefCell<XesEventImpl>>> = Vec::new();
    for event in trace_reader {
      events.push(Rc::new(RefCell::new(event)));
    }

    Some(XesTraceImpl {
      events_holder: EventsHolder::new(events),
      metadata: HashMap::new(),
    })
  }
}

impl Clone for XesTraceImpl {
  fn clone(&self) -> Self {
    Self {
      events_holder: self.events_holder.clone(),
      metadata: self.metadata.clone(),
    }
  }
}

impl Trace for XesTraceImpl {
  type TEvent = XesEventImpl;
  type TTraceInfo = EventSequenceInfo;
  type TTracePositions = EventsPositions;

  fn empty() -> Self {
    Self {
      events_holder: EventsHolder::empty(),
      metadata: HashMap::new(),
    }
  }

  fn events(&self) -> &Vec<Rc<RefCell<Self::TEvent>>> {
    &self.events_holder.events()
  }

  fn events_mut(&mut self) -> &mut Vec<Rc<RefCell<Self::TEvent>>> {
    self.events_holder.events_mut()
  }

  fn push(&mut self, event: Rc<RefCell<Self::TEvent>>) {
    self.events_holder.push(event);
  }

  fn to_names_vec(&self) -> Vec<String> {
    self.events_holder.to_names_vec()
  }

  fn get_or_create_trace_info(&mut self) -> &Self::TTraceInfo {
    self.events_holder.get_or_create_event_sequence_info()
  }

  fn get_or_create_events_positions(&mut self) -> &Self::TTracePositions {
    self.events_holder.get_or_create_events_positions()
  }

  fn remove_events_by<TPred>(&mut self, predicate: TPred)
  where
    TPred: Fn(&Self::TEvent) -> bool,
  {
    self.events_holder.remove_events_by(predicate);
  }

  fn mutate_events<TMutator>(&mut self, mutator: TMutator)
  where
    TMutator: Fn(&mut Self::TEvent),
  {
    self.events_holder.mutate_events(mutator);
  }

  fn metadata(&self) -> &HashMap<String, EventPayloadValue> {
    &self.metadata
  }

  fn metadata_mut(&mut self) -> &mut HashMap<String, EventPayloadValue> {
    &mut self.metadata
  }
}
