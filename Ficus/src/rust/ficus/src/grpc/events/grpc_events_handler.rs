use std::sync::Arc;

use super::events_handler::{GetContextValuesEvent, PipelineEvent, PipelineEventsHandler, PipelineFinalResult};
use crate::grpc::events::utils::{create_grpc_context_values, send_grpc_message};
use crate::{
  ficus_proto::{
    grpc_pipeline_final_result::ExecutionResult, GrpcGuid, GrpcPipelineFinalResult, GrpcPipelinePartExecutionResult,
    GrpcPipelinePartLogMessage, GrpcPipelinePartResult,
  },
  grpc::{
    backend_service::{GrpcResult, GrpcSender},
    logs_handler::ConsoleLogMessageHandler,
  },
  pipelines::context::LogMessageHandler,
};

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
  fn handle(&self, event: &PipelineEvent) {
    let result = match event {
      PipelineEvent::GetContextValuesEvent(event) => self.create_get_context_values_event(event),
      PipelineEvent::LogMessage(message) => self.create_log_message_result(&message),
      PipelineEvent::FinalResult(result) => self.create_final_result(match result {
        PipelineFinalResult::Success(uuid) => ExecutionResult::Success(GrpcGuid { guid: uuid.to_string() }),
        PipelineFinalResult::Error(error_message) => ExecutionResult::Error(error_message.to_string()),
      }),
    };

    if !self.is_alive() {
      let message = "The channel is closed, will not send the event";
      self.console_logs_handler.handle(message).ok();

      return;
    }

    send_grpc_message(self.sender.as_ref().as_ref(), &self.console_logs_handler, result);
  }

  fn is_alive(&self) -> bool {
    !self.sender.is_closed()
  }
}

impl GrpcPipelineEventsHandler {
  fn create_get_context_values_event(&self, event: &GetContextValuesEvent) -> GrpcPipelinePartExecutionResult {
    GrpcPipelinePartExecutionResult {
      result: Some(GrpcResult::PipelinePartResult(GrpcPipelinePartResult {
        guid: Some(GrpcGuid {
          guid: event.pipeline_part_id.to_string(),
        }),
        context_values: create_grpc_context_values(&event.key_values),
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
