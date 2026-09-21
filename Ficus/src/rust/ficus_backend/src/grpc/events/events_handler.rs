use crate::{ficus_proto::GrpcPipelinePartExecutionResult, grpc::events::kafka_events_handler::ProcessCaseMetadata};
use ficus::utils::context_key::ContextKey;
use std::{any::Any, cell::Cell};
use uuid::Uuid;

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
  fn handle(&self, _: &PipelineEvent) {}

  fn is_alive(&self) -> bool {
    false
  }
}

pub trait PipelineEventsHandlerWithRecords {
  fn drain_recorded_events(&self) -> Option<Vec<GrpcPipelinePartExecutionResult>> {
    None
  }
}

pub struct GetContextValuesEvent<'a> {
  pub process_case_metadata: ProcessCaseMetadata,
  pub pipeline_part_name: String,
  pub pipeline_part_id: Uuid,
  pub execution_id: Uuid,
  pub key_values: Vec<(&'a dyn ContextKey, &'a dyn Any)>,
}

pub enum PipelinePartExecResult<'a> {
  Default(GetContextValuesEvent<'a>),
  Recorded(Cell<GrpcPipelinePartExecutionResult>),
}

pub enum PipelineFinalResult {
  Success(Uuid),
  Error(String),
}

pub enum PipelineEvent<'a> {
  GetContextValuesEvent(PipelinePartExecResult<'a>),
  LogMessage(String),
  FinalResult(PipelineFinalResult),
  ProcessCaseMetadata(ProcessCaseMetadata),
}
