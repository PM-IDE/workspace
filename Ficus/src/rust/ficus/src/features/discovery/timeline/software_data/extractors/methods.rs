use crate::event_log::core::event::event::{Event, EventPayloadValue};
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::software_data::extraction_config::{MethodCommonAttributesConfig, MethodInliningConfig, SoftwareDataExtractionConfig};
use crate::features::discovery::timeline::software_data::extractors::core::{payload_value_or_none, regex_option_or_err, SoftwareDataExtractionError, SoftwareDataExtractor};
use crate::features::discovery::timeline::software_data::models::{MethodInliningData, MethodInliningEvent, MethodLoadUnloadEvent, MethodNameParts, SoftwareData};
use derive_new::new;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, new)]
pub struct MethodsDataExtractor<'a> {
  config: &'a SoftwareDataExtractionConfig,
}

impl<'a> SoftwareDataExtractor for MethodsDataExtractor<'a> {
  fn extract_from_events(&self, software_data: &mut SoftwareData, events: &[Rc<RefCell<XesEventImpl>>]) -> Result<(), SoftwareDataExtractionError> {
    let inlining_succeeded_regex = regex_option_or_err(self.config.method_inlining_success().as_ref().map(|c| c.event_class_regex()))?;
    let inlining_failed_regex = regex_option_or_err(self.config.method_inlining_failed().as_ref().map(|c| c.event_class_regex()))?;

    let method_load_regex = regex_option_or_err(self.config.method_load().as_ref().map(|c| c.event_class_regex()))?;
    let method_unload_regex = regex_option_or_err(self.config.method_unload().as_ref().map(|c| c.event_class_regex()))?;

    for event in events {
      if let Some(payload) = event.borrow().payload_map() {
        if let Some(inlining_succeeded_regex) = inlining_succeeded_regex.as_ref() {
          if inlining_succeeded_regex.is_match(event.borrow().name().as_str()).unwrap_or(false) {
            let config = self.config.method_inlining_success().as_ref().unwrap().info().inlining_config();
            if let Some(method_info) = extract_method_inlining_info(payload, config) {
              software_data.method_inlinings_events_mut().push(MethodInliningEvent::InliningSuccess(method_info));
            }

            continue;
          }
        }

        if let Some(inlining_failed_regex) = inlining_failed_regex.as_ref() {
          if inlining_failed_regex.is_match(event.borrow().name().as_str()).unwrap_or(false) {
            let failed_reason_attr = self.config.method_inlining_failed().as_ref().unwrap().info().fail_reason_attr();

            let failed_reason = payload_value_or_none(payload, failed_reason_attr);

            let config = self.config.method_inlining_failed().as_ref().unwrap().info().inlining_config();
            let method_info = extract_method_inlining_info(payload, config);

            if method_info.is_some() && failed_reason.is_some() {
              software_data.method_inlinings_events_mut().push(MethodInliningEvent::InliningFailed(method_info.unwrap(), failed_reason.unwrap()));
            }

            continue;
          }
        }

        if let Some(method_load_regex) = method_load_regex.as_ref() {
          if method_load_regex.is_match(event.borrow().name().as_str()).unwrap_or(false) {
            let method_name_parts = extract_method_name_parts(payload, self.config.method_load().as_ref().unwrap().info().common_attrs());
            if let Some(method_name_parts) = method_name_parts {
              software_data.method_load_unload_events_mut().push(MethodLoadUnloadEvent::Load(method_name_parts))
            }
          }
        }

        if let Some(method_unload_regex) = method_unload_regex.as_ref() {
          if method_unload_regex.is_match(event.borrow().name().as_str()).unwrap_or(false) {
            let method_name_parts = extract_method_name_parts(payload, self.config.method_unload().as_ref().unwrap().info().common_attrs());
            if let Some(method_name_parts) = method_name_parts {
              software_data.method_load_unload_events_mut().push(MethodLoadUnloadEvent::Unload(method_name_parts))
            }
          }
        }
      }
    }

    Ok(())
  }
}

fn extract_method_inlining_info(payload: &HashMap<String, EventPayloadValue>, config: &MethodInliningConfig) -> Option<MethodInliningData> {
  Some(MethodInliningData::new(
    extract_method_name_parts(payload, config.inlinee_method_attrs())?,
    extract_method_name_parts(payload, config.inliner_method_attrs())?,
  ))
}

fn extract_method_name_parts(payload: &HashMap<String, EventPayloadValue>, config: &MethodCommonAttributesConfig) -> Option<MethodNameParts> {
  Some(MethodNameParts::new(
    payload_value_or_none(payload, config.name_attr())?,
    payload_value_or_none(payload, config.namespace_attr())?,
    payload_value_or_none(payload, config.signature_attr())?,
  ))
}
