use std::cell::RefCell;
use std::rc::Rc;
use crate::event_log::core::event::event::Event;
use crate::features::discovery::timeline::events_groups::EventGroup;
use crate::features::discovery::timeline::software_data::extraction_config::SoftwareDataExtractionConfig;
use crate::features::discovery::timeline::software_data::extractors::core::{payload_value_or_none, regex_option_or_err, SoftwareDataExtractionError, SoftwareDataExtractor};
use crate::features::discovery::timeline::software_data::models::{MethodEvent, SoftwareData};
use derive_new::new;
use crate::event_log::xes::xes_event::XesEventImpl;

#[derive(Debug, Clone, new)]
pub struct MethodsDataExtractor<'a> {
  config: &'a SoftwareDataExtractionConfig,
}

impl<'a> SoftwareDataExtractor for MethodsDataExtractor<'a> {
  fn extract_from_events(&self, software_data: &mut SoftwareData, events: &[Rc<RefCell<XesEventImpl>>]) -> Result<(), SoftwareDataExtractionError> {
    let inlining_succeeded_regex = regex_option_or_err(self.config.method_inlining_success().as_ref().map(|c| c.event_class_regex()))?;
    let inlining_failed_regex = regex_option_or_err(self.config.method_inlining_failed().as_ref().map(|c| c.event_class_regex()))?;

    for event in events {
      if let Some(payload) = event.borrow().payload_map() {
        if let Some(inlining_succeeded_regex) = inlining_succeeded_regex.as_ref() {
          if inlining_succeeded_regex.is_match(event.borrow().name().as_str()).unwrap_or(false) {
            let method_name_attr = self.config.method_inlining_success().as_ref().unwrap().info().inlinee_method_attrs().name_attr();
            if let Some(method_name) = payload_value_or_none(payload, method_name_attr) {
              software_data.method_events_mut().push(MethodEvent::InliningSuccess(method_name));
            }

            continue;
          }
        }

        if let Some(inlining_failed_regex) = inlining_failed_regex.as_ref() {
          if inlining_failed_regex.is_match(event.borrow().name().as_str()).unwrap_or(false) {
            let method_name_attr = self.config.method_inlining_failed().as_ref().unwrap().info().inlinee_method_attrs().name_attr();
            let failed_reason_attr = self.config.method_inlining_failed().as_ref().unwrap().info().fail_reason_attr();

            let method_name = payload_value_or_none(payload, method_name_attr);
            let failed_reason = payload_value_or_none(payload, failed_reason_attr);

            if method_name.is_some() && failed_reason.is_some() {
              software_data.method_events_mut().push(MethodEvent::InliningFailed(method_name.unwrap(), failed_reason.unwrap()));
            }

            continue;
          }
        }
      }
    }

    Ok(())
  }
}