use crate::event_log::core::event::event::Event;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::discovery::{TraceThread, TraceThreadEvent};
use crate::features::discovery::timeline::events_groups::EventGroup;
use crate::features::discovery::timeline::software_data::extractors::core::{SoftwareDataExtractionError, EventGroupSoftwareDataExtractor};
use crate::features::discovery::timeline::software_data::models::SoftwareData;
use crate::features::discovery::timeline::utils::{extract_thread_id, get_stamp};
use derive_new::new;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, Clone, new)]
pub struct EventClassesDataExtractor<'a> {
  thread_attribute: Option<&'a String>,
  time_attribute: Option<&'a String>,
}

impl<'a> EventGroupSoftwareDataExtractor for EventClassesDataExtractor<'a> {
  fn extract(&self, software_data: &mut SoftwareData, event_group: &EventGroup) -> Result<(), SoftwareDataExtractionError> {
    self.extract_from_events(software_data, event_group.control_flow_events())
  }

  fn extract_from_events(&self, software_data: &mut SoftwareData, events: &[Rc<RefCell<XesEventImpl>>]) -> Result<(), SoftwareDataExtractionError> {
    let mut threads = HashMap::new();

    for event in events {
      *software_data.event_classes_mut().entry(event.borrow().name().clone()).or_insert(0) += 1;

      if let Some(thread_attribute) = self.thread_attribute {
        let thread_id = extract_thread_id(event.borrow().deref(), thread_attribute);
        let stamp = match get_stamp(event.borrow().deref(), self.time_attribute) {
          Ok(stamp) => stamp,
          Err(_) => return Err(SoftwareDataExtractionError::FailedToGetStamp)
        };

        threads.entry(thread_id).or_insert(TraceThread::empty()).events_mut().push(TraceThreadEvent::new(event.clone(), stamp));
      }
    }

    if !threads.is_empty() {
      software_data.thread_diagram_fragment_mut().extend(threads.into_values());
    }

    Ok(())
  }
}
