use super::events::events_handler::{PipelineEvent, PipelineEventsHandler, PipelineFinalResult};
use super::events::grpc_events_handler::GrpcPipelineEventsHandler;
use crate::ficus_proto::{GrpcContextKey, GrpcContextKeyValue, GrpcFicusBackendInfo, GrpcGetAllContextValuesResult, GrpcPipelinePartDescriptor, GrpcProxyPipelineExecutionRequest};
use crate::grpc::context_values_service::ContextValueService;
use crate::grpc::converters::convert_to_grpc_context_value;
use crate::grpc::pipeline_executor::ServicePipelineExecutionContext;
use crate::pipelines::keys::context_keys::find_context_key;
use crate::utils::context_key::{ContextKey, DefaultContextKey};
use crate::{
  ficus_proto::{
    grpc_backend_service_server::GrpcBackendService, GrpcGetContextValueRequest, GrpcGuid,
    GrpcPipelinePartExecutionResult,
  },
  pipelines::pipeline_parts::PipelineParts,
  utils::user_data::user_data::UserData,
};
use futures::Stream;
use std::{
  collections::HashMap,
  pin::Pin,
  sync::{Arc, Mutex},
};
use tokio::sync::mpsc::{self, Sender};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub(super) type GrpcResult = crate::ficus_proto::grpc_pipeline_part_execution_result::Result;
pub(super) type GrpcSender = Sender<Result<GrpcPipelinePartExecutionResult, Status>>;

pub struct FicusService {
  cv_service: Arc<ContextValueService>,
  pipeline_parts: Arc<Box<PipelineParts>>,
  contexts: Arc<Box<Mutex<HashMap<String, HashMap<String, Uuid>>>>>,
}

impl FicusService {
  pub fn new(cv_service: Arc<ContextValueService>) -> Self {
    Self {
      cv_service,
      pipeline_parts: Arc::new(Box::new(PipelineParts::new())),
      contexts: Arc::new(Box::new(Mutex::new(HashMap::new()))),
    }
  }
}

#[tonic::async_trait]
impl GrpcBackendService for FicusService {
  type ExecutePipelineStream = Pin<Box<dyn Stream<Item=Result<GrpcPipelinePartExecutionResult, Status>> + Send + Sync + 'static>>;

  async fn execute_pipeline(
    &self,
    request: Request<GrpcProxyPipelineExecutionRequest>,
  ) -> Result<Response<Self::ExecutePipelineStream>, Status> {
    let pipeline_parts = self.pipeline_parts.clone();
    let contexts = self.contexts.clone();
    let (sender, receiver) = mpsc::channel(4);

    let context_values = match self.cv_service.reclaim_context_values(&request.get_ref().context_values_ids) {
      Ok(context_values) => context_values,
      Err(not_found_id) => {
        let message = format!("Failed to find context value for id {}", not_found_id);
        return Err(Status::invalid_argument(message));
      }
    };

    let cv_service = self.cv_service.clone();

    tokio::task::spawn_blocking(move || {
      let grpc_pipeline = request.get_ref().pipeline.as_ref().unwrap();

      let sender = Arc::new(Box::new(GrpcPipelineEventsHandler::new(sender)) as Box<dyn PipelineEventsHandler>);
      let context = ServicePipelineExecutionContext::new(grpc_pipeline, &context_values, pipeline_parts, sender);

      match context.execute_grpc_pipeline(|_| Ok(())) {
        Ok((uuid, created_context)) => {
          if let Some(items) = created_context.items() {
            let mut ids = HashMap::new();

            for (key, value) in items {
              let context_key = DefaultContextKey::<()>::existing(key.id(), key.name().to_owned());

              if let Some(grpc_cv) = convert_to_grpc_context_value(&context_key, value) {
                let id = Uuid::new_v4();
                cv_service.put_context_value(id.to_string(), GrpcContextKeyValue {
                  key: Some(GrpcContextKey { name: key.name().to_owned() }),
                  value: Some(grpc_cv),
                });

                ids.insert(key.name().to_owned(), id);
              }
            }

            contexts.lock().as_mut().unwrap().insert(uuid.to_string(), ids);
          }

          context
            .sender()
            .handle(&PipelineEvent::FinalResult(PipelineFinalResult::Success(uuid)));
        }
        Err(error) => {
          context
            .sender()
            .handle(&PipelineEvent::FinalResult(PipelineFinalResult::Error(error.to_string())));
        }
      };
    });

    Ok(Response::new(Box::pin(ReceiverStream::new(receiver))))
  }

  async fn get_context_value(&self, request: Request<GrpcGetContextValueRequest>) -> Result<Response<GrpcGuid>, Status> {
    let key_name = &request.get_ref().key.as_ref().unwrap().name;
    match find_context_key(key_name) {
      None => Err(Status::not_found(format!("Failed to find key for key name {}", key_name))),
      Some(key) => {
        let id = request.get_ref().execution_id.as_ref().unwrap();
        match self.contexts.lock().as_mut().unwrap().get_mut(&id.guid) {
          None => Err(Status::not_found("Failed to get context for guid".to_string())),
          Some(keys_to_cv_ids) => match keys_to_cv_ids.get(key.key().name()) {
            None => Err(Status::not_found("Failed to get context for guid".to_string())),
            Some(id) => Ok(Response::new(GrpcGuid { guid: id.to_string() })),
          }
        }
      }
    }
  }

  async fn get_all_context_values(&self, request: Request<GrpcGuid>) -> Result<Response<GrpcGetAllContextValuesResult>, Status> {
    let id = request.get_ref();
    match self.contexts.lock().as_ref().unwrap().get(&id.guid) {
      None => Err(Status::not_found("The context values for supplied execution id are not found")),
      Some(keys_to_cv_ids) => Ok(Response::new(GrpcGetAllContextValuesResult {
        context_values: keys_to_cv_ids.values().into_iter().map(|id| GrpcGuid {
          guid: id.to_string()
        }).collect()
      }))
    }
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

  async fn get_backend_info(&self, _: Request<()>) -> Result<Response<GrpcFicusBackendInfo>, Status> {
    Ok(Response::new(GrpcFicusBackendInfo {
      name: "RUST_FICUS_BACKEND".to_string(),
      pipeline_parts: self.pipeline_parts
        .pipeline_parts_descriptors()
        .into_iter()
        .map(|d| GrpcPipelinePartDescriptor {
          name: d.name()
        })
        .collect(),
    }))
  }
}
