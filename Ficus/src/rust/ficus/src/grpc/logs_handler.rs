use std::sync::Arc;
use log::info;
use crate::pipelines::{context::LogMessageHandler, errors::pipeline_errors::PipelinePartExecutionError};

use super::events::events_handler::{PipelineEvent, PipelineEventsHandler};

pub struct GrpcLogMessageHandlerImpl {
    sender: Arc<Box<dyn PipelineEventsHandler>>,
}

impl LogMessageHandler for GrpcLogMessageHandlerImpl {
    fn handle(&self, message: &str) -> Result<(), PipelinePartExecutionError> {
        self.sender.handle(&PipelineEvent::LogMessage(message.to_string()));
        Ok(())
    }
}

impl GrpcLogMessageHandlerImpl {
    pub fn new(sender: Arc<Box<dyn PipelineEventsHandler>>) -> Self {
        Self { sender }
    }
}

#[derive(Clone)]
pub struct ConsoleLogMessageHandler {}

impl LogMessageHandler for ConsoleLogMessageHandler {
    fn handle(&self, message: &str) -> Result<(), PipelinePartExecutionError> {
        info!("{}", &message);

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
