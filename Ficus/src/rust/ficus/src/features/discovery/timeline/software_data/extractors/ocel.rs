use crate::event_log::core::event::event::{Event, EventPayloadValue};
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::software_data::extraction_config::SoftwareDataExtractionConfig;
use crate::features::discovery::timeline::software_data::extractors::core::{EventGroupSoftwareDataExtractor, SoftwareDataExtractionError};
use crate::features::discovery::timeline::software_data::models::{OcelData, OcelObjectAction, SoftwareData};
use derive_new::new;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use fancy_regex::Regex;
use log::{debug};

#[derive(Debug, Clone, new)]
pub struct OcelDataExtractor<'a> {
  config: &'a SoftwareDataExtractionConfig,
}

impl<'a> EventGroupSoftwareDataExtractor for OcelDataExtractor<'a> {
  fn extract_from_events(
    &self,
    software_data: &mut SoftwareData,
    events: &[Rc<RefCell<XesEventImpl>>],
  ) -> Result<(), SoftwareDataExtractionError> {
    if let Some(ocel_config) = self.config.ocel().as_ref() {
      let raw_regex = ocel_config.event_class_regex();
      let regex = Regex::new(raw_regex).map_err(|e| SoftwareDataExtractionError::FailedToParseRegex(raw_regex.to_owned()))?;

      for event in events {
        if !regex.is_match(event.borrow().name()).unwrap_or(false) {
          continue;
        }

        let object_type = ocel_config.info().object_type_attr().create(&event.borrow());
        let object_id = ocel_config.info().object_id_attr().create(&event.borrow());

        let action = if let Some(action_attr) = ocel_config.info().object_action_type_attr().as_ref() {
          let related_objs_ids = ocel_config.info().related_object_ids_attr().as_ref();
          if let Some(ocel_action) = Self::parse_ocel_object_action(&event.borrow(), action_attr, related_objs_ids) {
            ocel_action
          } else {
            get_fallback_ocel_object_action()
          }
        } else {
          get_fallback_ocel_object_action()
        };

        software_data.ocel_data_mut().push(OcelData::new(object_type, object_id, action));
      }
    }

    Ok(())
  }
}

fn get_fallback_ocel_object_action() -> OcelObjectAction {
  let fallback_action = OcelObjectAction::Allocate;
  debug!("Failed to get OCEL objet action, will use {} action as fallback value", fallback_action);

  fallback_action
}

impl<'a> OcelDataExtractor<'a> {
  fn parse_ocel_object_action(
    event: &XesEventImpl,
    action_attr: &String,
    related_objects_ids_attr: Option<&String>,
  ) -> Option<OcelObjectAction> {
    if let Some(map) = event.payload_map().as_ref() {
      if let Some(action_value) = map.get(action_attr).as_ref() {
        match action_value.to_string_repr().as_str() {
          "Allocate" => return Some(OcelObjectAction::Allocate),
          "Consume" => return Some(OcelObjectAction::Consume),
          "AllocateMerged" => return match Self::parse_related_objects_ids(map, related_objects_ids_attr) {
            None => Some(OcelObjectAction::Allocate),
            Some(ids) => Some(OcelObjectAction::AllocateMerged(ids))
          },
          "ConsumeWithProduce" => return match Self::parse_related_objects_ids(map, related_objects_ids_attr) {
            None => Some(OcelObjectAction::Consume),
            Some(ids) => Some(OcelObjectAction::ConsumeWithProduce(ids))
          },
          _ => {}
        }
      }
    }

    None
  }

  fn parse_related_objects_ids(
    payload: &HashMap<String, EventPayloadValue>,
    related_objects_ids_attr: Option<&String>,
  ) -> Option<Vec<String>> {
    if let Some(related_objects_ids_attr) = related_objects_ids_attr {
      if let Some(objects_ids) = payload.get(related_objects_ids_attr) {
        let parsed_ids: Vec<String> = objects_ids.to_string_repr()
          .trim()
          .split(' ')
          .filter_map(|s| if s.len() > 0 { Some(s.to_string()) } else { None })
          .collect();

        if parsed_ids.len() == 0 {
          return None
        }

        return Some(parsed_ids)
      }
    }

    None
  }
}