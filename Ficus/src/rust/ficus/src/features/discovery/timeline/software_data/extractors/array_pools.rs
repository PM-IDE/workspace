use crate::event_log::core::event::event::Event;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::software_data::extraction_config::{ArrayPoolExtractionConfig, SoftwareDataExtractionConfig};
use crate::features::discovery::timeline::software_data::extractors::core::{parse_or_err, prepare_configs, SoftwareDataExtractionError, SoftwareDataExtractor};
use crate::features::discovery::timeline::software_data::models::{ArrayPoolEvent, ArrayPoolEventKind, SoftwareData};
use derive_new::new;
use fancy_regex::Regex;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, new)]
pub struct ArrayPoolDataExtractor<'a> {
  config: &'a SoftwareDataExtractionConfig,
}

impl<'a> SoftwareDataExtractor for ArrayPoolDataExtractor<'a> {
  fn extract_from_events(&self, software_data: &mut SoftwareData, events: &[Rc<RefCell<XesEventImpl>>]) -> Result<(), SoftwareDataExtractionError> {
    let configs = [
      (self.config.array_pool_array_created(), ArrayPoolEventKind::Created),
      (self.config.array_pool_array_rented(), ArrayPoolEventKind::Rented),
      (self.config.array_pool_array_trimmed(), ArrayPoolEventKind::Trimmed),
      (self.config.array_pool_array_returned(), ArrayPoolEventKind::Returned),
    ];

    let processed_configs = prepare_configs(&configs)?;
    if processed_configs.is_empty() {
      return Ok(());
    }

    for event in events {
      for config in &processed_configs {
        if let Some(array_pool_event) = Self::extract_array_pool_event(event, &config.0, config.1, config.2.clone())? {
          software_data.pool_events_mut().push(array_pool_event);
        }
      }
    }

    Ok(())
  }
}

impl<'a> ArrayPoolDataExtractor<'a> {
  fn extract_array_pool_event(
    event: &Rc<RefCell<XesEventImpl>>,
    regex: &Regex,
    config: &ArrayPoolExtractionConfig,
    event_kind: ArrayPoolEventKind,
  ) -> Result<Option<ArrayPoolEvent>, SoftwareDataExtractionError> {
    if regex.is_match(event.borrow().name()).unwrap_or(false) {
      if let Some(payload) = event.borrow().payload_map() {
        let buffer_id = payload.get(config.buffer_id_attr().as_str());
        let buffer_size = payload.get(config.buffer_size_attr().as_str());

        if buffer_id.is_none() || buffer_size.is_none() {
          return Ok(None);
        }

        let buffer_id = parse_or_err(buffer_id.unwrap().to_string_repr().as_str())?;
        let buffer_size_bytes = parse_or_err(buffer_size.unwrap().to_string_repr().as_str())?;

        return Ok(Some(ArrayPoolEvent::new(buffer_id, buffer_size_bytes, event_kind)));
      }
    }

    Ok(None)
  }
}
