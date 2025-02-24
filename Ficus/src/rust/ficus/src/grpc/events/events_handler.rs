use std::any::Any;

use uuid::Uuid;

use crate::pipelines::keys::context_key::ContextKey;

pub trait PipelineEventsHandler: Send + Sync {
  fn handle(&self, event: &PipelineEvent);
  fn is_alive(&self) -> bool;
}

pub struct EmptyPipelineEventsHandler {}

impl EmptyPipelineEventsHandler {
  pub fn new() -> Self {
    Self {}
  }
}

impl PipelineEventsHandler for EmptyPipelineEventsHandler {
  fn handle(&self, event: &PipelineEvent) {}

  fn is_alive(&self) -> bool {
    false
  }
}

pub struct GetContextValuesEvent<'a> {
  pub process_case_metadata: ProcessCaseMetadata,
  pub pipeline_part_name: String,
  pub uuid: Uuid,
  pub key_values: Vec<(&'a dyn ContextKey, &'a dyn Any)>,
}

#[derive(Clone, Debug)]
pub struct CaseName {
  pub display_name: String,
  pub name_parts: Vec<String>,
}

impl CaseName {
  pub fn empty() -> Self {
    Self {
      name_parts: vec![],
      display_name: "UNDEFINED".to_string(),
    }
  }
}

pub struct ProcessCaseMetadata {
  pub case_name: CaseName,
  pub process_name: String,
  pub subscription_id: Option<Uuid>,
  pub subscription_name: Option<String>,
  pub pipeline_id: Option<Uuid>,
  pub pipeline_name: Option<String>,
  pub metadata: Vec<(String, String)>,
}

pub enum PipelineFinalResult {
  Success(Uuid),
  Error(String),
}

pub enum PipelineEvent<'a> {
  GetContextValuesEvent(GetContextValuesEvent<'a>),
  LogMessage(String),
  FinalResult(PipelineFinalResult),
}
