use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use derive_new::new;
use crate::event_log::core::event::event::Event;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::discovery::{TraceThread, TraceThreadEvent};
use crate::features::discovery::timeline::software_data::extractors::core::SoftwareDataExtractor;
use crate::features::discovery::timeline::software_data::models::SoftwareData;
use crate::features::discovery::timeline::utils::{extract_thread_id, get_stamp};
use crate::pipelines::errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError};

#[derive(Debug, Clone, new)]
pub struct EventClassesDataExtractor<'a> {
  thread_attribute: &'a str,
  time_attribute: Option<&'a String>
}

impl<'a> SoftwareDataExtractor for EventClassesDataExtractor<'a> {
  fn extract(&self, software_data: &mut SoftwareData, event_group: &Vec<Rc<RefCell<XesEventImpl>>>) -> Result<(), PipelinePartExecutionError> {
    let mut threads = HashMap::new();

    for event in event_group {
      *software_data.event_classes_mut().entry(event.borrow().name().clone()).or_insert(0) += 1;

      let thread_id = extract_thread_id(event.borrow().deref(), self.thread_attribute);
      let stamp = match get_stamp(event.borrow().deref(), self.time_attribute) {
        Ok(stamp) => stamp,
        Err(_) => return Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new("Failed to get stamp".to_string())))
      };

      threads.entry(thread_id).or_insert(TraceThread::empty()).events_mut().push(TraceThreadEvent::new(event.clone(), stamp))
    }

    software_data.thread_diagram_fragment_mut().extend(threads.into_values());

    Ok(())
  }
}