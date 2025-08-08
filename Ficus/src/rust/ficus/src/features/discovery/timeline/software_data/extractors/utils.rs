use crate::event_log::core::event::event::{Event, EventPayloadValue};
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::software_data::extraction_config::NameCreationStrategy;
use crate::features::discovery::timeline::software_data::extractors::core::SoftwareDataExtractionError;
use fancy_regex::Regex;
use std::collections::HashMap;

pub type RegexParingResult = Result<Regex, SoftwareDataExtractionError>;

impl NameCreationStrategy {
  pub(super) fn create(&self, event: &XesEventImpl) -> String {
    if let Some(map) = event.payload_map() {
      match self {
        NameCreationStrategy::SingleAttribute(single_attribute) => self.value_or_fallback(single_attribute.name(), map),
        NameCreationStrategy::ManyAttributes(many_attributes) => {
          let mut result = String::new();
          for attr in many_attributes.attributes() {
            result.push_str(self.value_or_fallback(attr, map).as_str());
            result.push_str(many_attributes.separator());
          }

          if many_attributes.attributes().len() > 0 {
            result.remove(result.len() - 1);
          }

          result
        }
      }
    } else {
      self.fallback_value()
    }
  }

  fn value_or_fallback(&self, attr: &String, payload: &HashMap<String, EventPayloadValue>) -> String {
    if let Some(attr_value) = payload.get(attr) {
      attr_value.to_string_repr().to_string()
    } else {
      self.fallback_value()
    }
  }
}
