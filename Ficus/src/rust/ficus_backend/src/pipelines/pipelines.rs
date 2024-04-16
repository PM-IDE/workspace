use crate::pipelines::context::PipelineInfrastructure;
use crate::utils::{
    colors::ColorsHolder,
    user_data::user_data::{UserData, UserDataImpl},
};

use super::{context::PipelineContext, errors::pipeline_errors::PipelinePartExecutionError, keys::context_keys::ContextKeys};

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
    fn execute(
        &self,
        context: &mut PipelineContext,
        infra: &PipelineInfrastructure,
        keys: &ContextKeys,
    ) -> Result<(), PipelinePartExecutionError> {
        self.put_default_concrete_keys(context, keys);

        for part in &self.parts {
            part.execute(context, infra, keys)?;
        }

        Ok(())
    }
}

impl Pipeline {
    fn put_default_concrete_keys(&self, context: &mut PipelineContext, keys: &ContextKeys) {
        if let None = context.concrete(keys.colors_holder().key()) {
            context.put_concrete(keys.colors_holder().key(), ColorsHolder::empty());
        }
    }
}

pub trait PipelinePart {
    fn execute(
        &self,
        context: &mut PipelineContext,
        infra: &PipelineInfrastructure,
        keys: &ContextKeys,
    ) -> Result<(), PipelinePartExecutionError>;
}

pub struct ParallelPipelinePart {
    parallel_pipelines: Vec<Pipeline>,
}

impl PipelinePart for ParallelPipelinePart {
    fn execute(
        &self,
        context: &mut PipelineContext,
        infra: &PipelineInfrastructure,
        keys: &ContextKeys,
    ) -> Result<(), PipelinePartExecutionError> {
        for pipeline in &self.parallel_pipelines[0..(self.parallel_pipelines.len() - 1)] {
            pipeline.execute(&mut context.clone(), infra, keys)?;
        }

        if let Some(last_pipeline) = self.parallel_pipelines.last() {
            last_pipeline.execute(context, infra, keys)?;
        }

        Ok(())
    }
}

type PipelinePartExecutor =
    Box<dyn Fn(&mut PipelineContext, &PipelineInfrastructure, &ContextKeys, &UserDataImpl) -> Result<(), PipelinePartExecutionError>>;

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
    fn execute(
        &self,
        context: &mut PipelineContext,
        infra: &PipelineInfrastructure,
        keys: &ContextKeys,
    ) -> Result<(), PipelinePartExecutionError> {
        (self.executor)(context, infra, keys, &self.config)
    }
}

pub(super) type PipelinePartFactory = Box<dyn Fn(Box<UserDataImpl>) -> DefaultPipelinePart>;
