use std::any::Any;
use std::sync::Arc;
use uuid::Uuid;

use super::events::events_handler::{CaseName, GetContextValuesEvent, PipelineEvent, PipelineEventsHandler, ProcessCaseMetadata};
use crate::pipelines::context::PipelineInfrastructure;
use crate::pipelines::keys::context_key::DefaultContextKey;
use crate::pipelines::keys::context_keys::{
    find_context_key, CASE_NAME, PIPELINE_ID, PIPELINE_NAME, PROCESS_NAME, SUBSCRIPTION_ID, SUBSCRIPTION_NAME, UNSTRUCTURED_METADATA,
};
use crate::{
    pipelines::{
        context::PipelineContext,
        errors::pipeline_errors::{MissingContextError, PipelinePartExecutionError},
        keys::context_key::ContextKey,
        pipelines::{DefaultPipelinePart, PipelinePart},
    },
    utils::user_data::user_data::UserData,
};

#[rustfmt::skip]
type GetContextHandler = Box<dyn Fn(Uuid, String, &mut PipelineContext, &PipelineInfrastructure, Vec<&dyn ContextKey>) -> Result<(), PipelinePartExecutionError>>;

pub struct GetContextValuePipelinePart {
    keys: Vec<String>,
    handler: GetContextHandler,
    uuid: Uuid,
    pipeline_part_name: String,
}

impl GetContextValuePipelinePart {
    pub fn new(keys: Vec<String>, uuid: Uuid, pipeline_part_name: String, handler: GetContextHandler) -> Self {
        Self {
            keys,
            handler,
            uuid,
            pipeline_part_name,
        }
    }

    pub fn create_context_pipeline_part(
        keys: Vec<String>,
        uuid: Uuid,
        pipeline_part_name: String,
        sender: Arc<Box<dyn PipelineEventsHandler>>,
        before_part: Option<Box<DefaultPipelinePart>>,
    ) -> Box<GetContextValuePipelinePart> {
        Box::new(GetContextValuePipelinePart::new(
            keys,
            uuid,
            pipeline_part_name,
            Box::new(move |uuid, pipeline_part_name, context, infra, context_keys| {
                if let Some(before_part) = before_part.as_ref() {
                    before_part.execute(context, infra)?;
                }

                let key_values = Self::find_context_values_for(&context_keys, context)?;
                let process_case_metadata = Self::create_process_case_metadata(context);

                sender.handle(&PipelineEvent::GetContextValuesEvent(GetContextValuesEvent {
                    process_case_metadata,
                    pipeline_part_name,
                    uuid,
                    key_values,
                }));

                Ok(())
            }),
        ))
    }

    fn create_process_case_metadata(context: &PipelineContext) -> ProcessCaseMetadata {
        let case_name = Self::value_or_default(context, &CASE_NAME, || CaseName::empty());
        let process_name = Self::value_or_default(context, &PROCESS_NAME, || "UNDEFINED_PROCESS".to_string());

        let subscription_id = Self::value_or_none(context, &SUBSCRIPTION_ID);
        let subscription_name = Self::value_or_none(context, &SUBSCRIPTION_NAME);

        let pipeline_id = Self::value_or_none(context, &PIPELINE_ID);
        let pipeline_name = Self::value_or_none(context, &PIPELINE_NAME);

        let metadata = Self::value_or_default(context, &UNSTRUCTURED_METADATA, || vec![]);

        ProcessCaseMetadata {
            process_name,
            case_name,
            subscription_id,
            subscription_name,
            pipeline_id,
            pipeline_name,
            metadata,
        }
    }

    fn value_or_default<'a, T: Clone>(context: &'a PipelineContext, key: &DefaultContextKey<T>, default_factory: impl Fn() -> T) -> T {
        match context.concrete(key.key()) {
            None => default_factory(),
            Some(value) => value.clone(),
        }
    }

    fn value_or_none<'a, T: Clone>(context: &'a PipelineContext, key: &DefaultContextKey<T>) -> Option<T> {
        match context.concrete(key.key()) {
            None => None,
            Some(value) => Some(value.clone()),
        }
    }

    fn find_context_values_for<'a>(
        keys: &Vec<&'a dyn ContextKey>,
        context: &'a PipelineContext,
    ) -> Result<Vec<(&'a dyn ContextKey, &'a dyn Any)>, PipelinePartExecutionError> {
        let mut key_values = vec![];
        for key in keys {
            match context.any(key.key()) {
                Some(context_value) => {
                    key_values.push((*key, context_value));
                }
                None => {
                    return Err(PipelinePartExecutionError::MissingContext(MissingContextError::new(
                        key.key().name().clone(),
                    )))
                }
            }
        }

        Ok(key_values)
    }
}

impl PipelinePart for GetContextValuePipelinePart {
    fn execute(&self, context: &mut PipelineContext, infra: &PipelineInfrastructure) -> Result<(), PipelinePartExecutionError> {
        let mut context_keys = vec![];
        for key_name in &self.keys {
            match find_context_key(key_name) {
                Some(key) => context_keys.push(key),
                None => {
                    return Err(PipelinePartExecutionError::MissingContext(MissingContextError::new(
                        key_name.clone(),
                    )))
                }
            }
        }

        (self.handler)(self.uuid.clone(), self.pipeline_part_name.to_owned(), context, infra, context_keys)
    }
}
