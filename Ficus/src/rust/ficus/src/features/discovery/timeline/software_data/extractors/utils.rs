use crate::{
  event_log::{
    core::event::event::{Event, EventPayloadValue},
    xes::xes_event::XesEventImpl,
  },
  features::discovery::timeline::software_data::{extraction_config::NameCreationStrategy, extractors::core::SoftwareDataExtractionError},
};
use fancy_regex::Regex;
use std::{collections::HashMap, sync::Arc};

pub type RegexParingResult = Result<Regex, SoftwareDataExtractionError>;

impl NameCreationStrategy {
  pub(super) fn create(&self, event: &XesEventImpl) -> Arc<str> {
    if let Some(map) = event.payload_map() {
      match self {
        NameCreationStrategy::SingleAttribute(single_attribute) => self.value_or_fallback(single_attribute.name(), map),
        NameCreationStrategy::ManyAttributes(many_attributes) => {
          let mut result = String::new();
          for attr in many_attributes.attributes() {
            result.push_str(self.value_or_fallback(attr, map).as_ref());
            result.push_str(many_attributes.separator());
          }

          if !many_attributes.attributes().is_empty() {
            result.remove(result.len() - 1);
          }

          Arc::from(result)
        }
      }
    } else {
      self.fallback_value()
    }
  }

  fn value_or_fallback(&self, attr: &str, payload: &HashMap<Arc<str>, EventPayloadValue>) -> Arc<str> {
    if let Some(attr_value) = payload.get(attr) {
      attr_value.to_string_repr()
    } else {
      self.fallback_value()
    }
  }
}
