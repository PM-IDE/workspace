use crate::event_log::core::event::event::Event;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::software_data::extraction_config::SoftwareDataExtractionConfig;
use crate::features::discovery::timeline::software_data::extractors::core::{parse_or_err, regex_or_err, SoftwareDataExtractionError, EventGroupSoftwareDataExtractor};
use crate::features::discovery::timeline::software_data::models::{AllocationEvent, SoftwareData};
use derive_new::new;
use log::{error, warn};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, new)]
pub struct AllocationDataExtractor<'a> {
  config: &'a SoftwareDataExtractionConfig,
}

impl<'a> EventGroupSoftwareDataExtractor for AllocationDataExtractor<'a> {
  fn extract_from_events(&self, software_data: &mut SoftwareData, events: &[Rc<RefCell<XesEventImpl>>]) -> Result<(), SoftwareDataExtractionError> {
    if let Some(config) = self.config.allocation() {
      let regex = regex_or_err(config.event_class_regex())?;

      for event in events {
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
              if let Some(object_size) = payload.get(object_size_attr) {
                allocated_objects_count * parse_or_err::<usize>(object_size.to_string_repr().as_str())?
              } else {
                warn!("Failed to get object_size_attr attribute, skipping this event");
                continue;
              }
            } else if let Some(total_allocated_bytes_attr) = config.info().total_allocated_bytes_attr() {
              if let Some(total_allocated_bytes) = payload.get(total_allocated_bytes_attr) {
                parse_or_err(total_allocated_bytes.to_string_repr().as_str())?
              } else {
                warn!("Failed to get total_allocated_bytes attribute, skipping this event");
                continue;
              }
            } else {
              error!("No allocation size attributes were present in the event, skipping this event");
              continue;
            };

            software_data.allocation_events_mut().push(AllocationEvent::new(
              type_name.unwrap().to_string_repr().to_string(),
              allocated_objects_count,
              allocated_bytes,
            ))
          }
        }
      }
    }

    Ok(())
  }
}