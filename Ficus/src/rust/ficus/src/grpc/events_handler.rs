use std::{any::Any, sync::Arc};

use uuid::Uuid;

use crate::{
    ficus_proto::{
        grpc_pipeline_final_result::ExecutionResult, GrpcContextValueWithKeyName, GrpcGuid, GrpcPipelineFinalResult,
        GrpcPipelinePartExecutionResult, GrpcPipelinePartLogMessage, GrpcPipelinePartResult, GrpcUuid,
    },
    pipelines::{context::LogMessageHandler, errors::pipeline_errors::PipelinePartExecutionError, keys::context_key::ContextKey},
    utils::user_data::keys::Key,
};

use super::{
    backend_service::{GrpcResult, GrpcSender},
    converters::convert_to_grpc_context_value,
    logs_handler::ConsoleLogMessageHandler,
};

pub trait PipelineEventsHandler: Send + Sync {
    fn handle(&self, event: PipelineEvent);
}

pub struct GetContextValuesEvent<'a> {
    pub(super) uuid: Uuid,
    pub(super) key_values: Vec<(&'a dyn ContextKey, &'a dyn Any)>,
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

pub struct GrpcPipelineEventsHandler {
    sender: Arc<Box<GrpcSender>>,
    console_logs_handler: ConsoleLogMessageHandler,
}

impl GrpcPipelineEventsHandler {
    pub fn new(sender: GrpcSender) -> Self {
        Self {
            sender: Arc::new(Box::new(sender)),
            console_logs_handler: ConsoleLogMessageHandler::new(),
        }
    }
}

impl PipelineEventsHandler for GrpcPipelineEventsHandler {
    fn handle(&self, event: PipelineEvent) {
        let result = match event {
            PipelineEvent::GetContextValuesEvent(event) => self.create_get_context_values_event(event),
            PipelineEvent::LogMessage(message) => self.create_log_message_result(&message),
            PipelineEvent::FinalResult(result) => self.create_final_result(match result {
                PipelineFinalResult::Success(uuid) => ExecutionResult::Success(GrpcGuid { guid: uuid.to_string() }),
                PipelineFinalResult::Error(error_message) => ExecutionResult::Error(error_message.to_string()),
            }),
        };

        match self.sender.blocking_send(Ok(result)) {
            Ok(_) => (),
            Err(err) => {
                let message = format!("Failed to send event, error: {}", err.to_string());
                self.console_logs_handler.handle(message.as_str()).ok();
            }
        }
    }
}

impl GrpcPipelineEventsHandler {
    fn create_get_context_values_event(&self, event: GetContextValuesEvent) -> GrpcPipelinePartExecutionResult {
        let mut grpc_values = vec![];
        for (key, context_value) in event.key_values {
            let value = convert_to_grpc_context_value(key, context_value);
            grpc_values.push(GrpcContextValueWithKeyName {
                key_name: key.key().name().to_owned(),
                value,
            });
        }

        GrpcPipelinePartExecutionResult {
            result: Some(GrpcResult::PipelinePartResult(GrpcPipelinePartResult {
                uuid: Some(GrpcUuid {
                    uuid: event.uuid.to_string(),
                }),
                context_values: grpc_values,
            })),
        }
    }

    fn create_log_message_result(&self, message: &str) -> GrpcPipelinePartExecutionResult {
        GrpcPipelinePartExecutionResult {
            result: Some(GrpcResult::LogMessage(GrpcPipelinePartLogMessage {
                message: message.to_owned(),
            })),
        }
    }

    fn create_final_result(&self, execution_result: ExecutionResult) -> GrpcPipelinePartExecutionResult {
        GrpcPipelinePartExecutionResult {
            result: Some(GrpcResult::FinalResult(GrpcPipelineFinalResult {
                execution_result: Some(execution_result),
            })),
        }
    }
}
