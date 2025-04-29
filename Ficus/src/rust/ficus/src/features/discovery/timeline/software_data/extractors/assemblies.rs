use std::cell::RefCell;
use std::rc::Rc;
use derive_new::new;
use crate::event_log::core::event::event::Event;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::software_data::extraction_config::SoftwareDataExtractionConfig;
use crate::features::discovery::timeline::software_data::extractors::core::{prepare_configs, prepare_functional_configs, SoftwareDataExtractionError, SoftwareDataExtractor};
use crate::features::discovery::timeline::software_data::models::{AssemblyEvent, AssemblyEventKind, SoftwareData};

#[derive(Clone, Debug, new)]
pub struct AssemblySoftwareDataExtractor<'a> {
  config: &'a SoftwareDataExtractionConfig
}

impl<'a> SoftwareDataExtractor for AssemblySoftwareDataExtractor<'a> {
  fn extract_from_events(&self, software_data: &mut SoftwareData, events: &[Rc<RefCell<XesEventImpl>>]) -> Result<(), SoftwareDataExtractionError> {
    let configs = [
      (self.config.assembly_load(), AssemblyEventKind::Load),
      (self.config.assembly_unload(), AssemblyEventKind::Unload)
    ];

    let configs = prepare_configs(&configs)?;
    if configs.is_empty() {
      return Ok(());
    }

    for event in events {
      for (regex, info, kind) in &configs {
        if regex.is_match(event.borrow().name().as_str()).unwrap_or(false) {
          if let Some(payload) = event.borrow().payload_map() {
            if let Some(assembly_name) = payload.get(info.asembly_name_attr()) {
              let event = AssemblyEvent::new(assembly_name.to_string_repr().as_str().to_owned(), kind.clone());
              software_data.assembly_events_mut().push(event);
            }
          }
        }
      }
    }

    Ok(())
  }
}