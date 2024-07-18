use std::sync::Arc;

use crate::{
    ficus_proto::{GrpcPipelinePartExecutionResult, GrpcPipelinePartLogMessage},
    pipelines::{
        context::LogMessageHandler,
        errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError},
    },
};

use super::backend_service::{GrpcResult, GrpcSender};

pub struct GrpcLogMessageHandlerImpl {
    sender: Arc<Box<GrpcSender>>,
}

impl LogMessageHandler for GrpcLogMessageHandlerImpl {
    fn handle(&self, message: &str) -> Result<(), PipelinePartExecutionError> {
        match self.sender.blocking_send(Ok(Self::create_log_message_result(&message))) {
            Ok(_) => Ok(()),
            Err(_) => {
                let message = format!("Failed to send log message: {}", &message);
                Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(message)))
            }
        }
    }
}

impl GrpcLogMessageHandlerImpl {
    pub fn new(sender: Arc<Box<GrpcSender>>) -> Self {
        Self { sender }
    }

    fn create_log_message_result(message: &str) -> GrpcPipelinePartExecutionResult {
        GrpcPipelinePartExecutionResult {
            result: Some(GrpcResult::LogMessage(GrpcPipelinePartLogMessage {
                message: message.to_owned(),
            })),
        }
    }
}

pub struct ConsoleLogMessageHandler {}

impl LogMessageHandler for ConsoleLogMessageHandler {
    fn handle(&self, message: &str) -> Result<(), PipelinePartExecutionError> {
        println!("{}", &message);

        Ok(())
    }
}

impl ConsoleLogMessageHandler {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct DelegatingLogMessageHandler {
    handlers: Vec<Box<dyn LogMessageHandler>>,
}

impl LogMessageHandler for DelegatingLogMessageHandler {
    fn handle(&self, message: &str) -> Result<(), PipelinePartExecutionError> {
        for handler in &self.handlers {
            handler.handle(message)?;
        }

        Ok(())
    }
}

impl DelegatingLogMessageHandler {
    pub fn new(handlers: Vec<Box<dyn LogMessageHandler>>) -> Self {
        Self { handlers }
    }
}
