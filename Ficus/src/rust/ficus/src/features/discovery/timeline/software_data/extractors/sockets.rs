use crate::event_log::core::event::event::Event;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::software_data::extraction_config::{SocketAcceptConnectFailedConfig, SocketConnectAcceptStartConfig, SoftwareDataExtractionConfig};
use crate::features::discovery::timeline::software_data::extractors::core::{payload_value_or_none, prepare_functional_configs, SoftwareDataExtractionError, SoftwareDataExtractor};
use crate::features::discovery::timeline::software_data::models::{SocketConnectAcceptFailedMetadata, SocketConnectAcceptStartMetadata, SocketEvent, SoftwareData};
use derive_new::new;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, new)]
pub struct SocketsDataExtractor<'a> {
  config: &'a SoftwareDataExtractionConfig
}

impl<'a> SoftwareDataExtractor for SocketsDataExtractor<'a> {
  fn extract_from_events(&self, software_data: &mut SoftwareData, events: &[Rc<RefCell<XesEventImpl>>]) -> Result<(), SoftwareDataExtractionError> {
    let configs: &[(Option<&String>, &dyn Fn(&XesEventImpl) -> Result<Option<SocketEvent>, SoftwareDataExtractionError>)] = &[
      (self.config.socket_connect_start().as_ref().map(|c| c.event_class_regex()), &|event| { 
        create_connect_accept_start(event, self.config.socket_connect_start().as_ref().unwrap().info(), true) 
      }),

      (self.config.socket_accept_start().as_ref().map(|c| c.event_class_regex()), &|event| {
        create_connect_accept_start(event, self.config.socket_accept_start().as_ref().unwrap().info(), false)
      }),

      (self.config.socket_accept_stop().as_ref().map(|c| c.event_class_regex()), &|_| { Ok(Some(SocketEvent::AcceptStop)) }),
      (self.config.socket_connect_stop().as_ref().map(|c| c.event_class_regex()), &|_| { Ok(Some(SocketEvent::ConnectStop)) }),

      (self.config.socket_connect_failed().as_ref().map(|c| c.event_class_regex()), &|event| {
        create_connect_accept_failed(event, self.config.socket_connect_failed().as_ref().unwrap().info(), true)
      }),

      (self.config.socket_accept_failed().as_ref().map(|c| c.event_class_regex()), &|event| {
        create_connect_accept_failed(event, self.config.socket_accept_failed().as_ref().unwrap().info(), false)
      })
    ];
    
    let configs = prepare_functional_configs(&configs)?;
    
    for event in events {
      for (regex, factory) in &configs {
        if regex.is_match(event.borrow().name().as_str()).unwrap_or(false) {
          if let Some(socket_event) = factory(&event.borrow())? {
            software_data.socket_events_mut().push(socket_event);
          }
        }
      }
    }
    
    Ok(())
  }
}

fn create_connect_accept_start(event: &XesEventImpl, config: &SocketConnectAcceptStartConfig, connect: bool) -> Result<Option<SocketEvent>, SoftwareDataExtractionError> {
  if let Some(metadata) = extract_connect_accept_start_metadata(event, config) {
    Ok(Some(match connect {
      true => SocketEvent::ConnectStart(metadata),
      false => SocketEvent::AcceptStart(metadata)
    }))
  } else {
    Ok(None)
  }
}

fn create_connect_accept_failed(event: &XesEventImpl, config: &SocketAcceptConnectFailedConfig, connect: bool) -> Result<Option<SocketEvent>, SoftwareDataExtractionError> {
  if let Some(metadata) = extract_connect_accept_failed_metadata(event, config) {
    Ok(Some(match connect {
      true => SocketEvent::ConnectFailed(metadata),
      false => SocketEvent::AcceptFailed(metadata)
    }))
  } else {
    Ok(None)
  }
}

fn extract_connect_accept_start_metadata(event: &XesEventImpl, config: &SocketConnectAcceptStartConfig) -> Option<SocketConnectAcceptStartMetadata> {
  if let Some(payload) = event.payload_map() {
    payload_value_or_none(payload, config.address_attr()).map(|address| SocketConnectAcceptStartMetadata::new(address))
  } else {
    None
  }
}

fn extract_connect_accept_failed_metadata(event: &XesEventImpl, config: &SocketAcceptConnectFailedConfig) -> Option<SocketConnectAcceptFailedMetadata> {
  if let Some(payload) = event.payload_map() {
    let error_code = payload_value_or_none(payload, config.error_code_attr()).unwrap_or("NONE".to_string());
    let error_message = payload_value_or_none(payload, config.error_message_attr()).unwrap_or("NONE".to_string());

    Some(SocketConnectAcceptFailedMetadata::new(error_code, error_message))
  } else {
    None
  }
}