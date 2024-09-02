use futures::Stream;
use std::any::Any;
use std::{
    collections::HashMap,
    pin::Pin,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc::{self, Sender};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use crate::ficus_proto::grpc_get_context_value_result::ContextValueResult;
use crate::ficus_proto::GrpcPipelineFinalResult;
use crate::grpc::converters::convert_to_grpc_context_value;
use crate::grpc::pipeline_executor::ServicePipelineExecutionContext;
use crate::pipelines::keys::context_keys::find_context_key;
use crate::{
    ficus_proto::{
        grpc_backend_service_server::GrpcBackendService, grpc_pipeline_final_result::ExecutionResult, GrpcGetContextValueRequest,
        GrpcGetContextValueResult, GrpcGuid, GrpcPipelineExecutionRequest, GrpcPipelinePartExecutionResult,
    },
    pipelines::{keys::context_key::ContextKey, pipeline_parts::PipelineParts},
    utils::user_data::user_data::{UserData, UserDataImpl},
};

pub(super) type GrpcResult = crate::ficus_proto::grpc_pipeline_part_execution_result::Result;
pub(super) type GrpcSender = Sender<Result<GrpcPipelinePartExecutionResult, Status>>;

pub struct FicusService {
    pipeline_parts: Arc<Box<PipelineParts>>,
    contexts: Arc<Box<Mutex<HashMap<String, UserDataImpl>>>>,
}

impl FicusService {
    pub fn new() -> Self {
        Self {
            pipeline_parts: Arc::new(Box::new(PipelineParts::new())),
            contexts: Arc::new(Box::new(Mutex::new(HashMap::new()))),
        }
    }
}

#[tonic::async_trait]
impl GrpcBackendService for FicusService {
    type ExecutePipelineStream = Pin<Box<dyn Stream<Item = Result<GrpcPipelinePartExecutionResult, Status>> + Send + Sync + 'static>>;

    async fn execute_pipeline(
        &self,
        request: Request<GrpcPipelineExecutionRequest>,
    ) -> Result<Response<Self::ExecutePipelineStream>, Status> {
        let pipeline_parts = self.pipeline_parts.clone();
        let contexts = self.contexts.clone();
        let (sender, receiver) = mpsc::channel(4);

        tokio::task::spawn_blocking(move || {
            let grpc_pipeline = request.get_ref().pipeline.as_ref().unwrap();
            let context_values = &request.get_ref().initial_context;
            let context = ServicePipelineExecutionContext::new(grpc_pipeline, context_values, pipeline_parts, sender);

            match context.execute_grpc_pipeline() {
                Ok((guid, created_context)) => {
                    contexts.lock().as_mut().unwrap().insert(guid.guid.to_owned(), created_context);

                    context
                        .sender()
                        .blocking_send(Ok(Self::create_final_result(ExecutionResult::Success(guid))))
                        .ok();
                }
                Err(error) => {
                    context
                        .sender()
                        .blocking_send(Ok(Self::create_final_result(ExecutionResult::Error(error.to_string()))))
                        .ok();
                }
            };
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(receiver))))
    }

    async fn get_context_value(&self, request: Request<GrpcGetContextValueRequest>) -> Result<Response<GrpcGetContextValueResult>, Status> {
        let key_name = &request.get_ref().key.as_ref().unwrap().name;
        let result = match find_context_key(key_name) {
            None => Self::create_get_context_value_error("Failed to find key for key name".to_string()),
            Some(key) => {
                let id = request.get_ref().execution_id.as_ref().unwrap();
                match self.contexts.lock().as_mut().unwrap().get_mut(&id.guid) {
                    None => Self::create_get_context_value_error("Failed to get context for guid".to_string()),
                    Some(value) => match value.any(key.key()) {
                        None => {
                            if let Some(created_value) = value.any(key.key()) {
                                self.try_convert_context_value(key, created_value)
                            } else {
                                Self::create_get_context_value_error("Failed to find context value for key".to_string())
                            }
                        }
                        Some(context_value) => self.try_convert_context_value(key, context_value),
                    },
                }
            }
        };

        Ok(Response::new(result))
    }

    async fn drop_execution_result(&self, request: Request<GrpcGuid>) -> Result<Response<()>, Status> {
        let mut contexts = self.contexts.lock();
        let contexts = contexts.as_mut().ok().unwrap();
        let guid_str = &request.get_ref().guid;

        match contexts.remove(guid_str) {
            None => Err(Status::not_found(format!("The session for {} does not exist", guid_str))),
            Some(_) => Ok(Response::new(())),
        }
    }
}

impl FicusService {
    fn create_final_result(execution_result: ExecutionResult) -> GrpcPipelinePartExecutionResult {
        GrpcPipelinePartExecutionResult {
            result: Some(GrpcResult::FinalResult(GrpcPipelineFinalResult {
                execution_result: Some(execution_result),
            })),
        }
    }

    fn create_get_context_value_error(message: String) -> GrpcGetContextValueResult {
        GrpcGetContextValueResult {
            context_value_result: Some(ContextValueResult::Error(message)),
        }
    }

    fn try_convert_context_value(&self, key: &dyn ContextKey, context_value: &dyn Any) -> GrpcGetContextValueResult {
        let value = convert_to_grpc_context_value(key, context_value);
        if let Some(grpc_context_value) = value {
            GrpcGetContextValueResult {
                context_value_result: Some(ContextValueResult::Value(grpc_context_value)),
            }
        } else {
            let msg = "Can not convert context value to grpc model".to_string();
            Self::create_get_context_value_error(msg)
        }
    }
}
