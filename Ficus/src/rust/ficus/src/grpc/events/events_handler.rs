use std::any::Any;

use uuid::Uuid;

use crate::pipelines::keys::context_key::ContextKey;

pub trait PipelineEventsHandler: Send + Sync {
    fn handle(&self, event: PipelineEvent);
}

pub struct GetContextValuesEvent<'a> {
    pub uuid: Uuid,
    pub key_values: Vec<(&'a dyn ContextKey, &'a dyn Any)>,
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