use std::cell::RefCell;
use std::rc::Rc;
use derive_new::new;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::discovery::timeline::software_data::extraction_config::SoftwareDataExtractionConfig;
use crate::features::discovery::timeline::software_data::extractors::core::SoftwareDataExtractor;
use crate::features::discovery::timeline::software_data::models::SoftwareData;
use crate::pipelines::errors::pipeline_errors::PipelinePartExecutionError;

#[derive(Debug, Clone, new)]
pub struct AllocationDataExtractor<'a> {
  config: &'a SoftwareDataExtractionConfig
}

impl<'a> SoftwareDataExtractor for AllocationDataExtractor<'a> {
  fn extract(&self, software_data: &mut SoftwareData, event_group: &Vec<Rc<RefCell<XesEventImpl>>>) -> Result<(), PipelinePartExecutionError> {
    Ok(())
  }
}