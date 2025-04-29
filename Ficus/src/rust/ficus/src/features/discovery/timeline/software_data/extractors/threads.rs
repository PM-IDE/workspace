use std::cell::RefCell;
use std::rc::Rc;
use derive_new::new;
use crate::event_log::core::event::event::Event;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::software_data::extraction_config::SoftwareDataExtractionConfig;
use crate::features::discovery::timeline::software_data::extractors::core::{parse_or_err, prepare_configs, prepare_functional_configs, regex_or_err, SoftwareDataExtractionError, SoftwareDataExtractor};
use crate::features::discovery::timeline::software_data::models::{SoftwareData, ThreadEvent, ThreadEventKind};

#[derive(Clone, Debug, new)]
pub struct ThreadDataExtractor<'a> {
  config: &'a SoftwareDataExtractionConfig,
}

impl<'a> SoftwareDataExtractor for ThreadDataExtractor<'a> {
  fn extract_from_events(&self, software_data: &mut SoftwareData, events: &[Rc<RefCell<XesEventImpl>>]) -> Result<(), SoftwareDataExtractionError> {
    let configs = [
      (self.config.thread_created(), ThreadEventKind::Created),
      (self.config.thread_created(), ThreadEventKind::Created),
    ];

    let configs = prepare_configs(&configs)?;
    if configs.is_empty() {
      return Ok(());
    }

    for event in events {
      for (regex, config, kind) in &configs {
        if regex.is_match(event.borrow().name().as_str()).unwrap_or(false) {
          if let Some(payload) = event.borrow().payload_map() {
            if let Some(thread_id) = payload.get(config.thread_id_attr()) {
              let thread_id = parse_or_err(thread_id.to_string_repr().as_str())?;
              software_data.thread_events_mut().push(ThreadEvent::new(thread_id, kind.clone()));
            }
          }
        } 
      }
    }

    Ok(())
  }
}
