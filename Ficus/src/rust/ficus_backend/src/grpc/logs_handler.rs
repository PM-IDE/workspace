use std::sync::Arc;

use crate::{
    ficus_proto::{GrpcPipelinePartExecutionResult, GrpcPipelinePartLogMessage},
    pipelines::{
        context::LogMessageHandler,
        errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError},
    },
};

use super::backend_service::{GrpcResult, GrpcSender};

pub struct LogMessageHandlerImpl {
    sender: Arc<Box<GrpcSender>>,
}

impl LogMessageHandler for LogMessageHandlerImpl {
    fn handle(&self, message: String) -> Result<(), PipelinePartExecutionError> {
        match self.sender.blocking_send(Ok(Self::create_log_message_result(&message))) {
            Ok(_) => Ok(()),
            Err(_) => {
                let message = format!("Failed to send log message: {}", &message);
                Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(message)))
            }
        }
    }
}

impl LogMessageHandlerImpl {
    pub fn new(sender: Arc<Box<GrpcSender>>) -> Self {
        Self { sender }
    }

    fn create_log_message_result(message: &String) -> GrpcPipelinePartExecutionResult {
        GrpcPipelinePartExecutionResult {
            result: Some(GrpcResult::LogMessage(GrpcPipelinePartLogMessage {
                message: message.to_owned(),
            })),
        }
    }
}
