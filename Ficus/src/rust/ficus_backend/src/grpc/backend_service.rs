use std::str::FromStr;
use std::{
    any::Any,
    collections::HashMap,
    pin::Pin,
    sync::{Arc, Mutex},
};

use futures::Stream;
use tokio::sync::mpsc::{self, Sender};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use super::{
    converters::{convert_to_grpc_context_value, create_initial_context, put_into_user_data},
    get_context_pipeline::GetContextValuePipelinePart,
    logs_handler::LogMessageHandlerImpl,
};
use crate::pipelines::context::PipelineInfrastructure;
use crate::{
    ficus_proto::{
        grpc_backend_service_server::GrpcBackendService, grpc_get_context_value_result::ContextValueResult,
        grpc_pipeline_final_result::ExecutionResult, grpc_pipeline_part_base::Part, GrpcContextKeyValue, GrpcGetContextValueRequest,
        GrpcGetContextValueResult, GrpcGuid, GrpcPipeline, GrpcPipelineExecutionRequest, GrpcPipelineFinalResult, GrpcPipelinePart,
        GrpcPipelinePartExecutionResult,
    },
    pipelines::{
        context::LogMessageHandler,
        errors::pipeline_errors::PipelinePartExecutionError,
        keys::{context_key::ContextKey, context_keys::ContextKeys},
        pipeline_parts::PipelineParts,
        pipelines::{DefaultPipelinePart, Pipeline, PipelinePart},
    },
    utils::user_data::user_data::{UserData, UserDataImpl},
};

pub(super) type GrpcResult = crate::ficus_proto::grpc_pipeline_part_execution_result::Result;
pub(super) type GrpcSender = Sender<Result<GrpcPipelinePartExecutionResult, Status>>;

pub struct FicusService {
    pipeline_parts: Arc<Box<PipelineParts>>,
    context_keys: Arc<Box<ContextKeys>>,
    contexts: Arc<Box<Mutex<HashMap<String, UserDataImpl>>>>,
}

impl FicusService {
    pub fn new(types: Arc<Box<ContextKeys>>) -> Self {
        Self {
            pipeline_parts: Arc::new(Box::new(PipelineParts::new())),
            context_keys: types,
            contexts: Arc::new(Box::new(Mutex::new(HashMap::new()))),
        }
    }
}

pub(super) struct ServicePipelineExecutionContext<'a> {
    grpc_pipeline: &'a GrpcPipeline,
    context_values: &'a Vec<GrpcContextKeyValue>,
    context_keys: Arc<Box<ContextKeys>>,
    pipeline_parts: Arc<Box<PipelineParts>>,
    sender: Arc<Box<GrpcSender>>,
    log_message_handler: Arc<Box<dyn LogMessageHandler>>,
}

impl<'a> ServicePipelineExecutionContext<'a> {
    pub fn new(
        grpc_pipeline: &'a GrpcPipeline,
        context_values: &'a Vec<GrpcContextKeyValue>,
        context_keys: Arc<Box<ContextKeys>>,
        pipeline_parts: Arc<Box<PipelineParts>>,
        sender: GrpcSender,
    ) -> Self {
        let sender = Arc::new(Box::new(sender));
        let log_message_handler = Self::create_log_message_handler(sender.clone());

        Self {
            grpc_pipeline,
            context_values,
            context_keys,
            pipeline_parts,
            sender,
            log_message_handler,
        }
    }

    fn create_log_message_handler(sender: Arc<Box<GrpcSender>>) -> Arc<Box<dyn LogMessageHandler>> {
        let log_message_handler = LogMessageHandlerImpl::new(sender.clone());
        let log_message_handler = Box::new(log_message_handler) as Box<dyn LogMessageHandler>;
        Arc::new(log_message_handler)
    }

    pub fn sender(&self) -> Arc<Box<GrpcSender>> {
        self.sender.clone()
    }

    pub fn grpc_pipeline(&self) -> &GrpcPipeline {
        &self.grpc_pipeline
    }

    pub fn keys(&self) -> &ContextKeys {
        &self.context_keys
    }

    pub fn parts(&self) -> &PipelineParts {
        &self.pipeline_parts
    }

    pub fn context_values(&self) -> &Vec<GrpcContextKeyValue> {
        &self.context_values
    }

    pub fn log_message_handler(&self) -> Arc<Box<dyn LogMessageHandler>> {
        self.log_message_handler.clone()
    }

    pub fn with_pipeline(&self, new_grpc_pipeline: &'a GrpcPipeline) -> Self {
        Self {
            grpc_pipeline: new_grpc_pipeline,
            context_values: self.context_values,
            context_keys: self.context_keys.clone(),
            pipeline_parts: self.pipeline_parts.clone(),
            sender: self.sender.clone(),
            log_message_handler: self.log_message_handler.clone(),
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
        let context_keys = self.context_keys.clone();
        let pipeline_parts = self.pipeline_parts.clone();
        let contexts = self.contexts.clone();
        let (sender, receiver) = mpsc::channel(4);

        tokio::task::spawn_blocking(move || {
            let grpc_pipeline = request.get_ref().pipeline.as_ref().unwrap();
            let context_values = &request.get_ref().initial_context;
            let context = ServicePipelineExecutionContext::new(grpc_pipeline, context_values, context_keys, pipeline_parts, sender);

            match Self::execute_grpc_pipeline(&context) {
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
        let result = match self.context_keys.find_key(key_name) {
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
    fn execute_grpc_pipeline<'a>(
        context: &ServicePipelineExecutionContext,
    ) -> Result<(GrpcGuid, UserDataImpl), PipelinePartExecutionError> {
        let id = Uuid::new_v4();
        let pipeline = Self::to_pipeline(context);
        let mut pipeline_context = create_initial_context(context);
        let infra = PipelineInfrastructure::new(Some(context.log_message_handler()));

        match pipeline.execute(&mut pipeline_context, &infra, context.keys()) {
            Ok(()) => Ok((GrpcGuid { guid: id.to_string() }, pipeline_context.devastate_user_data())),
            Err(err) => Err(err),
        }
    }

    pub(super) fn to_pipeline(context: &ServicePipelineExecutionContext) -> Pipeline {
        let mut pipeline = Pipeline::empty();
        for grpc_part in &context.grpc_pipeline().parts {
            match grpc_part.part.as_ref().unwrap() {
                Part::DefaultPart(grpc_default_part) => match Self::find_default_part(grpc_default_part, context) {
                    Some(found_part) => {
                        pipeline.push(found_part);
                    }
                    None => todo!(),
                },
                Part::ParallelPart(_) => todo!(),
                Part::SimpleContextRequestPart(part) => {
                    let key_name = part.key.as_ref().unwrap().name.clone();
                    let uuid = Uuid::from_str(&part.frontend_part_uuid.as_ref().unwrap().uuid).ok().unwrap();

                    pipeline.push(Self::create_get_context_part(vec![key_name], uuid, &context.sender(), None));
                }
                Part::ComplexContextRequestPart(part) => {
                    let grpc_default_part = part.before_pipeline_part.as_ref().unwrap();
                    let uuid = Uuid::from_str(&part.frontend_part_uuid.as_ref().unwrap().uuid).ok().unwrap();

                    match Self::find_default_part(grpc_default_part, context) {
                        Some(found_part) => {
                            let key_names = part.keys.iter().map(|x| x.name.to_owned()).collect();
                            pipeline.push(Self::create_get_context_part(key_names, uuid, &context.sender(), Some(found_part)));
                        }
                        None => todo!(),
                    }
                }
            }
        }

        pipeline
    }

    fn create_get_context_part(
        key_names: Vec<String>,
        uuid: Uuid,
        sender: &Arc<Box<GrpcSender>>,
        before_part: Option<Box<DefaultPipelinePart>>,
    ) -> Box<GetContextValuePipelinePart> {
        let sender = sender.clone();
        GetContextValuePipelinePart::create_context_pipeline_part(key_names, uuid, sender, before_part)
    }

    fn find_default_part(
        grpc_default_part: &GrpcPipelinePart,
        context: &ServicePipelineExecutionContext,
    ) -> Option<Box<DefaultPipelinePart>> {
        let mut part_config = UserDataImpl::new();
        let grpc_config = &grpc_default_part.configuration.as_ref().unwrap();

        for conf_value in &grpc_config.configuration_parameters {
            let key_name = conf_value.key.as_ref().unwrap().name.as_ref();
            if let Some(key) = context.keys().find_key(key_name) {
                let value = conf_value.value.as_ref().unwrap().context_value.as_ref().unwrap();
                put_into_user_data(key.key(), value, &mut part_config, context);
            }
        }

        match context.parts().find_part(&grpc_default_part.name) {
            Some(default_part) => Some(Box::new(default_part(Box::new(part_config)))),
            None => None,
        }
    }

    fn create_get_context_value_error(message: String) -> GrpcGetContextValueResult {
        GrpcGetContextValueResult {
            context_value_result: Some(ContextValueResult::Error(message)),
        }
    }

    fn try_convert_context_value(&self, key: &Box<dyn ContextKey>, context_value: &dyn Any) -> GrpcGetContextValueResult {
        let value = convert_to_grpc_context_value(key.as_ref(), context_value, &self.context_keys);
        if let Some(grpc_context_value) = value {
            GrpcGetContextValueResult {
                context_value_result: Some(ContextValueResult::Value(grpc_context_value)),
            }
        } else {
            let msg = "Can not convert context value to grpc model".to_string();
            Self::create_get_context_value_error(msg)
        }
    }

    fn create_final_result(execution_result: ExecutionResult) -> GrpcPipelinePartExecutionResult {
        GrpcPipelinePartExecutionResult {
            result: Some(GrpcResult::FinalResult(GrpcPipelineFinalResult {
                execution_result: Some(execution_result),
            })),
        }
    }
}
