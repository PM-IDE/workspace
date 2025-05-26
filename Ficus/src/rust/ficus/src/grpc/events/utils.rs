use crate::ficus_proto::GrpcContextValueWithKeyName;
use crate::grpc::converters::convert_to_grpc_context_value;
use crate::grpc::logs_handler::ConsoleLogMessageHandler;
use crate::pipelines::context::LogMessageHandler;
use crate::utils::context_key::ContextKey;
use std::any::Any;
use tokio::sync::mpsc::Sender;
use tonic::Status;

pub(super) fn create_grpc_context_values(key_values: &Vec<(&dyn ContextKey, &dyn Any)>) -> Vec<GrpcContextValueWithKeyName> {
  let mut grpc_values = vec![];
  for (key, context_value) in key_values {
    let value = convert_to_grpc_context_value(*key, *context_value);
    grpc_values.push(GrpcContextValueWithKeyName {
      key_name: key.key().name().to_owned(),
      value,
    });
  }

  grpc_values
}

pub(super) fn send_grpc_message<T>(sender: &Sender<Result<T, Status>>, logs_handler: &ConsoleLogMessageHandler, value: T) {
  match sender.blocking_send(Ok(value)) {
    Ok(_) => {}
    Err(err) => {
      let message = format!("Failed to send event, error: {}", err.to_string());
      logs_handler.handle(message.as_str()).ok();
    }
  }
}
