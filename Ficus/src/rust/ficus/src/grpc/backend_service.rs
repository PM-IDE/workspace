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
use crate::grpc::converters::convert_to_grpc_context_value;
use crate::grpc::pipeline_executor::ServicePipelineExecutionContext;
use crate::pipelines::keys::context_keys::find_context_key;
use crate::{
    ficus_proto::{
        grpc_backend_service_server::GrpcBackendService, GrpcGetContextValueRequest, GrpcGetContextValueResult, GrpcGuid,
        GrpcPipelineExecutionRequest, GrpcPipelinePartExecutionResult,
    },
    pipelines::{keys::context_key::ContextKey, pipeline_parts::PipelineParts},
    utils::user_data::user_data::{UserData, UserDataImpl},
};
use crate::ficus_proto::GrpcProxyPipelineExecutionRequest;
use crate::grpc::context_values_service::{ContextValueService, GrpcContextValueService};
use super::events::events_handler::{PipelineEvent, PipelineEventsHandler, PipelineFinalResult};
use super::events::grpc_events_handler::GrpcPipelineEventsHandler;

pub(super) type GrpcResult = crate::ficus_proto::grpc_pipeline_part_execution_result::Result;
pub(super) type GrpcSender = Sender<Result<GrpcPipelinePartExecutionResult, Status>>;

pub struct FicusService {
    cv_service: Arc<Mutex<ContextValueService>>,
    pipeline_parts: Arc<Box<PipelineParts>>,
    contexts: Arc<Box<Mutex<HashMap<String, UserDataImpl>>>>,
}

impl FicusService {
    pub fn new(cv_service: Arc<Mutex<ContextValueService>>) -> Self {
        Self {
            cv_service,
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
        request: Request<GrpcProxyPipelineExecutionRequest>,
    ) -> Result<Response<Self::ExecutePipelineStream>, Status> {
        let pipeline_parts = self.pipeline_parts.clone();
        let contexts = self.contexts.clone();
        let (sender, receiver) = mpsc::channel(4);

        let mut cv_service = self.cv_service.lock();
        let cv_service = cv_service.as_mut().expect("Must acquire lock");
        let context_values = match cv_service.reclaim_context_values(&request.get_ref().context_values_ids) {
            Ok(context_values) => context_values,
            Err(not_found_id) => {
                let message = format!("Failed to find context value for id {}", not_found_id);
                return Err(Status::invalid_argument(message));
            }
        };

        tokio::task::spawn_blocking(move || {
            let grpc_pipeline = request.get_ref().pipeline.as_ref().unwrap();

            let sender = Arc::new(Box::new(GrpcPipelineEventsHandler::new(sender)) as Box<dyn PipelineEventsHandler>);
            let context = ServicePipelineExecutionContext::new(grpc_pipeline, &context_values, pipeline_parts, sender);

            match context.execute_grpc_pipeline(|_| {}) {
                Ok((uuid, created_context)) => {
                    contexts.lock().as_mut().unwrap().insert(uuid.to_string(), created_context);

                    context
                        .sender()
                        .handle(PipelineEvent::FinalResult(PipelineFinalResult::Success(uuid)));
                }
                Err(error) => {
                    context
                        .sender()
                        .handle(PipelineEvent::FinalResult(PipelineFinalResult::Error(error.to_string())));
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
