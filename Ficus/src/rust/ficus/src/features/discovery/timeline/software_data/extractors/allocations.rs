use std::cell::RefCell;
use std::rc::Rc;
use derive_new::new;
use fancy_regex::Regex;
use log::warn;
use crate::event_log::core::event::event::Event;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::software_data::extraction_config::SoftwareDataExtractionConfig;
use crate::features::discovery::timeline::software_data::extractors::core::{SoftwareDataExtractionError, SoftwareDataExtractor};
use crate::features::discovery::timeline::software_data::extractors::event_classes::parse_or_err;
use crate::features::discovery::timeline::software_data::models::{AllocationEvent, SoftwareData};

#[derive(Debug, Clone, new)]
pub struct AllocationDataExtractor<'a> {
  config: &'a SoftwareDataExtractionConfig
}

impl<'a> SoftwareDataExtractor for AllocationDataExtractor<'a> {
  fn extract(&self, software_data: &mut SoftwareData, event_group: &Vec<Rc<RefCell<XesEventImpl>>>) -> Result<(), SoftwareDataExtractionError> {
    if let Some(config) = self.config.allocation() {
      let regex = match Regex::new(config.event_class_regex()) {
        Ok(regex) => regex,
        Err(_) => return Err(SoftwareDataExtractionError::FailedToParseRegex(config.event_class_regex().to_owned()))
      };

      for event in event_group {
        if regex.is_match(event.borrow().name()).unwrap_or(false) {
          if let Some(payload) = event.borrow().payload_map() {
            let type_name = payload.get(config.info().type_name_attr());
            let allocated_count = payload.get(config.info().allocated_count_attr());

            if type_name.is_none() || allocated_count.is_none() {
              warn!("Failed to get type_name or allocated_count attributes for object allocation event, skipping this event");
              continue;
            }

            let allocated_objects_count = parse_or_err(allocated_count.unwrap().to_string_repr().as_str())?;

            let allocated_bytes = if let Some(object_size_attr) = config.info().object_size_bytes_attr() {
              allocated_objects_count * parse_or_err::<usize>(object_size_attr.as_str())?
            } else if let Some(total_allocated_bytes) = config.info().total_allocated_bytes_attr() {
              parse_or_err(total_allocated_bytes)?
            } else {
              warn!("Failed to get object_size_attr or total_allocated_bytes attributes, skipping this event");
              continue;
            };

            software_data.allocation_events_mut().push(AllocationEvent::new(
              type_name.unwrap().to_string_repr().to_string(),
              allocated_objects_count,
              allocated_bytes
            ))
          }
        }
      }
    }

    Ok(())
  }
}