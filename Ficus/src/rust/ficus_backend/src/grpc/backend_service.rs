use super::events::{
  events_handler::{PipelineEvent, PipelineEventsHandler, PipelineFinalResult},
  grpc_events_handler::GrpcPipelineEventsHandler,
};
use crate::{
  ficus_proto::{
    GrpcFicusBackendInfo, GrpcGuid, GrpcPipelinePartDescriptor, GrpcPipelinePartExecutionResult, GrpcProxyPipelineExecutionRequest,
    grpc_backend_service_server::GrpcBackendService,
  },
  grpc::{context_values_service::ContextValueService, pipeline_executor::ServicePipelineExecutionContext},
};
use ficus::pipelines::pipeline_parts::PipelineParts;
use futures::Stream;
use std::{pin::Pin, sync::Arc};
use tokio::sync::mpsc::{self, Sender};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

pub(super) type GrpcResult = crate::ficus_proto::grpc_pipeline_part_execution_result::Result;
pub(super) type GrpcSender = Sender<Result<GrpcPipelinePartExecutionResult, Status>>;

pub struct FicusService {
  cv_service: Arc<ContextValueService>,
  pipeline_parts: Arc<PipelineParts>,
}

impl FicusService {
  pub fn new(cv_service: Arc<ContextValueService>) -> Self {
    Self {
      cv_service,
      pipeline_parts: Arc::new(PipelineParts::new()),
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

      let sender = Arc::new(GrpcPipelineEventsHandler::new(sender));
      let sender = sender as Arc<dyn PipelineEventsHandler>;
      let context = ServicePipelineExecutionContext::new(grpc_pipeline, &context_values, pipeline_parts, sender);

      match context.execute_grpc_pipeline_and_fill_context_values(|_| Ok(()), cv_service) {
        Ok(uuid) => {
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

  async fn drop_execution_result(&self, request: Request<GrpcGuid>) -> Result<Response<()>, Status> {
    self.cv_service.drop_execution_result(&request.get_ref().guid)
  }

  async fn get_backend_info(&self, _: Request<()>) -> Result<Response<GrpcFicusBackendInfo>, Status> {
    Ok(Response::new(GrpcFicusBackendInfo {
      name: "RUST_FICUS_BACKEND".to_string(),
      pipeline_parts: self
        .pipeline_parts
        .pipeline_parts_descriptors()
        .into_iter()
        .map(|d| GrpcPipelinePartDescriptor { name: d.name() })
        .collect(),
    }))
  }
}
