use std::sync::Arc;
use uuid::Uuid;

use super::events::events_handler::{GetContextValuesEvent, PipelineEvent, PipelineEventsHandler};
use crate::pipelines::context::PipelineInfrastructure;
use crate::pipelines::keys::context_keys::{find_context_key, CASE_NAME, PROCESS_NAME};
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
        Self { keys, handler, uuid, pipeline_part_name }
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

                let mut key_values = vec![];
                for key in context_keys {
                    match context.any(key.key()) {
                        Some(context_value) => {
                            key_values.push((key, context_value));
                        }
                        None => {
                            return Err(PipelinePartExecutionError::MissingContext(MissingContextError::new(
                                key.key().name().clone(),
                            )))
                        }
                    }
                }

                let case_name = match context.concrete(CASE_NAME.key()) {
                    None => "CASE_NAME_UNDEFINED".to_string(),
                    Some(case_name) => case_name.to_string(),
                };

                let process_name = match context.concrete(PROCESS_NAME.key()) {
                    None => "PROCESS_NAME_UNDEFINED".to_string(),
                    Some(process_name) => process_name.to_string(),
                };

                sender.handle(PipelineEvent::GetContextValuesEvent(GetContextValuesEvent {
                    case_name,
                    process_name,
                    pipeline_part_name,
                    uuid,
                    key_values
                }));

                Ok(())
            }),
        ))
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
