use std::cell::RefCell;
use std::rc::Rc;
use derive_new::new;
use log::warn;
use crate::event_log::core::event::event::Event;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::software_data::extraction_config::SoftwareDataExtractionConfig;
use crate::features::discovery::timeline::software_data::extractors::core::{regex_or_err, SoftwareDataExtractionError, SoftwareDataExtractor};
use crate::features::discovery::timeline::software_data::models::{SocketEvent, SoftwareData};

#[derive(Clone, Debug, new)]
pub struct SocketsDataExtractor<'a> {
  config: &'a SoftwareDataExtractionConfig
}

impl<'a> SoftwareDataExtractor for SocketsDataExtractor<'a> {
  fn extract_from_events(&self, software_data: &mut SoftwareData, events: &Vec<Rc<RefCell<XesEventImpl>>>) -> Result<(), SoftwareDataExtractionError> {
    if let Some(config) = self.config.sockets() {
      let regex = regex_or_err(config.event_class_regex().as_str())?;

      for event in events {
        if regex.is_match(event.borrow().name().as_str()).unwrap_or(false) {
          if let Some(payload) = event.borrow().payload_map() {
            let address = payload.get(config.info().address_attr().as_str()).map(|v| v.to_string_repr().as_str().to_string());
            if let Some(address) = address {
              software_data.socket_events_mut().push(SocketEvent::new(address));
            } else {
              warn!("Address was not specified for socket event, skipping this event");
            }
          }
        }
      }
    }
    
    Ok(())
  }
}