use crate::pipelines::context::PipelineInfrastructure;
use crate::pipelines::keys::context_keys::COLORS_HOLDER_KEY;
use crate::utils::{
  colors::ColorsHolder,
  user_data::user_data::{UserData, UserDataImpl},
};

use super::{context::PipelineContext, errors::pipeline_errors::PipelinePartExecutionError};

pub struct Pipeline {
  parts: Vec<Box<dyn PipelinePart>>,
}

impl Pipeline {
  pub fn empty() -> Self {
    Self { parts: vec![] }
  }

  pub fn push(&mut self, part: Box<dyn PipelinePart>) {
    self.parts.push(part);
  }
}

impl PipelinePart for Pipeline {
  fn execute(&self, context: &mut PipelineContext, infra: &PipelineInfrastructure) -> Result<(), PipelinePartExecutionError> {
    self.put_default_concrete_keys(context);

    for part in &self.parts {
      part.execute(context, infra)?;
    }

    Ok(())
  }
}

impl Pipeline {
  fn put_default_concrete_keys(&self, context: &mut PipelineContext) {
    if let None = context.concrete(COLORS_HOLDER_KEY.key()) {
      context.put_concrete(COLORS_HOLDER_KEY.key(), ColorsHolder::empty());
    }
  }
}

pub trait PipelinePart {
  fn execute(&self, context: &mut PipelineContext, infra: &PipelineInfrastructure) -> Result<(), PipelinePartExecutionError>;
}

pub struct ParallelPipelinePart {
  parallel_pipelines: Vec<Pipeline>,
}

impl PipelinePart for ParallelPipelinePart {
  fn execute(&self, context: &mut PipelineContext, infra: &PipelineInfrastructure) -> Result<(), PipelinePartExecutionError> {
    for pipeline in &self.parallel_pipelines[0..(self.parallel_pipelines.len() - 1)] {
      pipeline.execute(&mut context.clone(), infra)?;
    }

    if let Some(last_pipeline) = self.parallel_pipelines.last() {
      last_pipeline.execute(context, infra)?;
    }

    Ok(())
  }
}

type PipelinePartExecutor =
Box<dyn Fn(&mut PipelineContext, &PipelineInfrastructure, &UserDataImpl) -> Result<(), PipelinePartExecutionError>>;

pub struct DefaultPipelinePart {
  name: String,
  config: Box<UserDataImpl>,
  executor: PipelinePartExecutor,
}

impl DefaultPipelinePart {
  pub fn new(name: String, config: Box<UserDataImpl>, executor: PipelinePartExecutor) -> Self {
    Self { name, config, executor }
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn config(&self) -> &UserDataImpl {
    &self.config
  }
}

impl PipelinePart for DefaultPipelinePart {
  fn execute(&self, context: &mut PipelineContext, infra: &PipelineInfrastructure) -> Result<(), PipelinePartExecutionError> {
    (self.executor)(context, infra, &self.config)
  }
}

pub(super) type PipelinePartFactory = Box<dyn Fn(Box<UserDataImpl>) -> DefaultPipelinePart>;
