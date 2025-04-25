use std::cell::RefCell;
use std::rc::Rc;
use derive_new::new;
use log::warn;
use crate::event_log::core::event::event::Event;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::software_data::extraction_config::SoftwareDataExtractionConfig;
use crate::features::discovery::timeline::software_data::extractors::core::{regex_or_err, SoftwareDataExtractionError, SoftwareDataExtractor};
use crate::features::discovery::timeline::software_data::models::{HTTPEvent, SoftwareData};

#[derive(Clone, Debug, new)]
pub struct HTTPSoftwareDataExtractor<'a> {
  config: &'a SoftwareDataExtractionConfig
}

impl<'a> SoftwareDataExtractor for HTTPSoftwareDataExtractor<'a> {
  fn extract_from_events(&self, software_data: &mut SoftwareData, events: &[Rc<RefCell<XesEventImpl>>]) -> Result<(), SoftwareDataExtractionError> {
    if let Some(config) = self.config.http() {
      let regex = regex_or_err(config.event_class_regex().as_str())?;
      
      for event in events {
        if regex.is_match(event.borrow().name().as_str()).unwrap_or(false) {
          if let Some(payload) = event.borrow().payload_map() {
            let host = payload.get(config.info().host_attr()).map(|v| v.to_string_repr().as_str().to_owned());
            let path = payload.get(config.info().path_attr()).map(|v| v.to_string_repr().as_str().to_owned());
            let port = payload.get(config.info().port_attr()).map(|v| v.to_string_repr().as_str().to_owned());
            let query = payload.get(config.info().query_attr()).map(|v| v.to_string_repr().as_str().to_owned());
            let scheme = payload.get(config.info().scheme_attr()).map(|v| v.to_string_repr().as_str().to_owned());

            if host.is_none() || path.is_none() || port.is_none() || query.is_none() || scheme.is_none() {
              warn!("Some attributes of HTTP event were not specified, skipping this event");
              continue;
            }

            software_data.http_events_mut().push(HTTPEvent::new(host.unwrap(), port.unwrap(), scheme.unwrap(), path.unwrap(), query.unwrap()));
          } 
        }
      }
    }
    
    Ok(())
  }
}