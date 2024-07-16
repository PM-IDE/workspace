use std::sync::Arc;
use uuid::Uuid;

use crate::ficus_proto::{GrpcContextValueWithKeyName, GrpcUuid};
use crate::pipelines::context::PipelineInfrastructure;
use crate::{
    ficus_proto::{GrpcPipelinePartExecutionResult, GrpcPipelinePartResult},
    pipelines::{
        context::PipelineContext,
        errors::pipeline_errors::{MissingContextError, PipelinePartExecutionError},
        keys::{context_key::ContextKey},
        pipelines::{DefaultPipelinePart, PipelinePart},
    },
    utils::user_data::user_data::UserData,
};
use crate::pipelines::keys::context_keys::find_context_key;
use super::{
    backend_service::{GrpcResult, GrpcSender},
    converters::convert_to_grpc_context_value,
};

#[rustfmt::skip]
type GetContextHandler = Box<dyn Fn(Uuid, &mut PipelineContext, &PipelineInfrastructure, Vec<&dyn ContextKey>) -> Result<(), PipelinePartExecutionError>>;

pub struct GetContextValuePipelinePart {
    keys: Vec<String>,
    handler: GetContextHandler,
    uuid: Uuid,
}

impl GetContextValuePipelinePart {
    pub fn new(keys: Vec<String>, uuid: Uuid, handler: GetContextHandler) -> Self {
        Self { keys, handler, uuid }
    }

    pub fn create_context_pipeline_part(
        keys: Vec<String>,
        uuid: Uuid,
        sender: Arc<Box<GrpcSender>>,
        before_part: Option<Box<DefaultPipelinePart>>,
    ) -> Box<GetContextValuePipelinePart> {
        Box::new(GetContextValuePipelinePart::new(
            keys,
            uuid,
            Box::new(move |uuid, context, infra, context_keys| {
                if let Some(before_part) = before_part.as_ref() {
                    before_part.execute(context, infra)?;
                }

                let mut grpc_values = vec![];
                for key in context_keys {
                    match context.any(key.key()) {
                        Some(context_value) => {
                            let value = convert_to_grpc_context_value(key, context_value);
                            grpc_values.push(GrpcContextValueWithKeyName {
                                key_name: key.key().name().to_owned(),
                                value,
                            });
                        }
                        None => {
                            return Err(PipelinePartExecutionError::MissingContext(MissingContextError::new(
                                key.key().name().clone(),
                            )))
                        }
                    }
                }

                sender
                    .blocking_send(Ok(GrpcPipelinePartExecutionResult {
                        result: Some(GrpcResult::PipelinePartResult(GrpcPipelinePartResult {
                            uuid: Some(GrpcUuid { uuid: uuid.to_string() }),
                            context_values: grpc_values,
                        })),
                    }))
                    .ok();

                Ok(())
            }),
        ))
    }
}

impl PipelinePart for GetContextValuePipelinePart {
    fn execute(
        &self,
        context: &mut PipelineContext,
        infra: &PipelineInfrastructure,
    ) -> Result<(), PipelinePartExecutionError> {
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

        (self.handler)(self.uuid.clone(), context, infra, context_keys)
    }
}
