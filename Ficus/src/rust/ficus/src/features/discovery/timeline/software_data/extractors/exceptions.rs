use crate::event_log::core::event::event::Event;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::software_data::extraction_config::SoftwareDataExtractionConfig;
use crate::features::discovery::timeline::software_data::extractors::core::{payload_value_or_none, regex_or_err, SoftwareDataExtractionError, SoftwareDataExtractor};
use crate::features::discovery::timeline::software_data::models::{ExceptionEvent, SoftwareData};
use derive_new::new;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, new)]
pub struct ExceptionDataExtractor<'a> {
  config: &'a SoftwareDataExtractionConfig
}

impl<'a> SoftwareDataExtractor for ExceptionDataExtractor<'a> {
  fn extract_from_events(&self, software_data: &mut SoftwareData, events: &Vec<Rc<RefCell<XesEventImpl>>>) -> Result<(), SoftwareDataExtractionError> {
    if let Some(config) = self.config.exceptions() {
      let regex = regex_or_err(config.event_class_regex())?;

      for event in events {
        if let Some(payload) = event.borrow().payload_map() {
          if regex.is_match(event.borrow().name().as_str()).unwrap_or(false) {
            if let Some(exception_type) = payload_value_or_none(payload, config.info().type_name_attr()) {
              software_data.exception_events_mut().push(ExceptionEvent::new(exception_type))
            }
          }
        }
      }
    }

    Ok(())
  }
}