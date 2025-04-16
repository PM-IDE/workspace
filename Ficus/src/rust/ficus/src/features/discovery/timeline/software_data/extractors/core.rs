use std::cell::RefCell;
use std::rc::Rc;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::software_data::models::SoftwareData;
use crate::pipelines::errors::pipeline_errors::PipelinePartExecutionError;

pub trait SoftwareDataExtractor {
  fn extract(&self, software_data: &mut SoftwareData, event_group: &Vec<Rc<RefCell<XesEventImpl>>>) -> Result<(), PipelinePartExecutionError>;
}