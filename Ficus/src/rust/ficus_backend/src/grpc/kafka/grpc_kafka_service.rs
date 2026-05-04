use crate::ficus_proto::GrpcGetCurrentContextValuesRequest;
use crate::{
  ficus_proto::{
    GrpcAddPipelineRequest, GrpcAddPipelineStreamRequest, GrpcExecutePipelineAndProduceKafkaRequest,
    GrpcGetAllSubscriptionsAndPipelinesResponse, GrpcGuid, GrpcKafkaFailedResult, GrpcKafkaResult, GrpcKafkaSubscription,
    GrpcKafkaSubscriptionMetadata, GrpcKafkaSuccessResult, GrpcPipelineMetadata, GrpcPipelinePartExecutionResult,
    GrpcRemoveAllPipelinesRequest, GrpcRemovePipelineRequest, GrpcSubscribeToKafkaRequest, GrpcSubscriptionPipeline,
    GrpcUnsubscribeFromKafkaRequest, grpc_kafka_result, grpc_kafka_service_server::GrpcKafkaService,
  },
  grpc::{
    context_values_service::ContextValueService,
    events::{
      delegating_events_handler::DelegatingEventsHandler,
      events_handler::{PipelineEvent, PipelineEventsHandler, PipelineFinalResult},
      grpc_events_handler::GrpcPipelineEventsHandler,
    },
    kafka::{kafka_service::KafkaService, models::PipelineExecutionDto},
  },
};
use ficus::{
  features::cases::CaseName,
  pipelines::{
    keys::context_keys::{CASE_NAME_KEY, PIPELINE_ID_KEY, PIPELINE_NAME_KEY, PROCESS_NAME_KEY, SUBSCRIPTION_ID_KEY, SUBSCRIPTION_NAME_KEY},
    pipeline_parts::PipelineParts,
  },
  utils::user_data::user_data::UserData,
};
use futures::Stream;
use std::{pin::Pin, rc::Rc, sync::Arc};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub struct GrpcKafkaServiceImpl {
  cv_service: Arc<ContextValueService>,
  kafka_service: KafkaService,
  pipeline_parts: Arc<PipelineParts>,
}

impl GrpcKafkaServiceImpl {
  pub fn new(cv_service: Arc<ContextValueService>) -> Self {
    let pipeline_parts = Arc::new(PipelineParts::default());
    Self {
      cv_service: cv_service.clone(),
      kafka_service: KafkaService::new(pipeline_parts.clone(), cv_service),
      pipeline_parts,
    }
  }
}

#[tonic::async_trait]
impl GrpcKafkaService for GrpcKafkaServiceImpl {
  async fn subscribe_for_kafka_topic(&self, request: Request<GrpcSubscribeToKafkaRequest>) -> Result<Response<GrpcKafkaResult>, Status> {
    let result = match self.kafka_service.subscribe_to_kafka_topic(request.get_ref().clone()) {
      Ok(consumer_uuid) => grpc_kafka_result::Result::Success(GrpcKafkaSuccessResult {
        id: Some(GrpcGuid::from(consumer_uuid)),
      }),
      Err(err) => grpc_kafka_result::Result::Failure(GrpcKafkaFailedResult {
        error_message: err.to_string(),
      }),
    };

    Ok(Response::new(GrpcKafkaResult { result: Some(result) }))
  }

  async fn unsubscribe_from_kafka_topic(
    &self,
    request: Request<GrpcUnsubscribeFromKafkaRequest>,
  ) -> Result<Response<GrpcKafkaResult>, Status> {
    let uuid = request
      .get_ref()
      .subscription_id
      .as_ref()
      .expect("Subscription id must be provided")
      .to_uuid()?;

    let result = self.kafka_service.unsubscribe_from_kafka(uuid);

    Ok(Response::new(GrpcKafkaResult { result: Some(result) }))
  }

  async fn get_current_context_values(&self, request: Request<GrpcGetCurrentContextValuesRequest>) -> Result<Response<GrpcGuid>, Status> {
    let pipeline_id = request.get_ref().pipeline_id.as_ref().unwrap().to_uuid()?;
    let subscription_id = request.get_ref().subscription_id.as_ref().unwrap().to_uuid()?;
    let process_name = request.get_ref().process_name.as_str();

    self
      .kafka_service
      .get_context_values(pipeline_id, subscription_id, process_name)
      .map(|id| Response::new(GrpcGuid { guid: id.to_string() }))
  }

  async fn add_pipeline_to_subscription(&self, request: Request<GrpcAddPipelineRequest>) -> Result<Response<GrpcKafkaResult>, Status> {
    let dto = request.get_ref().to_dto();
    let handler = KafkaService::create_kafka_events_handler(request.get_ref().producer_kafka_metadata.as_ref())?;
    let pipeline_id =
      self
        .kafka_service
        .add_execution_request(dto.subscription_id, handler, dto.request, dto.streaming_configuration, dto.name);

    Ok(Response::new(GrpcKafkaResult::success(pipeline_id)))
  }

  type AddPipelineToSubscriptionStreamStream =
    Pin<Box<dyn Stream<Item = Result<GrpcPipelinePartExecutionResult, Status>> + Send + Sync + 'static>>;

  async fn add_pipeline_to_subscription_stream(
    &self,
    request: Request<GrpcAddPipelineStreamRequest>,
  ) -> Result<Response<Self::AddPipelineToSubscriptionStreamStream>, Status> {
    let dto = request.get_ref().to_dto();
    let (sender, receiver) = mpsc::channel(4);
    let handler = GrpcPipelineEventsHandler::new(sender);

    self
      .kafka_service
      .add_execution_request(dto.subscription_id, handler, dto.request, dto.streaming_configuration, dto.name);

    Ok(Response::new(Box::pin(ReceiverStream::new(receiver))))
  }

  async fn remove_pipeline_subscription(&self, request: Request<GrpcRemovePipelineRequest>) -> Result<Response<GrpcKafkaResult>, Status> {
    let subscription_id = request
      .get_ref()
      .subscription_id
      .as_ref()
      .expect("Subscription id must be provided")
      .to_uuid()?;

    let pipeline_id = request
      .get_ref()
      .pipeline_id
      .as_ref()
      .expect("Pipeline id must be provided")
      .to_uuid()?;

    self.kafka_service.remove_execution_request(&subscription_id, &pipeline_id);

    Ok(Response::new(GrpcKafkaResult::success(pipeline_id)))
  }

  async fn remove_all_pipeline_subscriptions(
    &self,
    request: Request<GrpcRemoveAllPipelinesRequest>,
  ) -> Result<Response<GrpcKafkaResult>, Status> {
    let subscription_id = request
      .get_ref()
      .subscription_id
      .as_ref()
      .expect("Subscription id must be provided")
      .to_uuid()?;

    self.kafka_service.remove_all_execution_requests(&subscription_id);

    Ok(Response::new(GrpcKafkaResult::success(subscription_id)))
  }

  async fn get_all_subscriptions_and_pipelines(
    &self,
    _: Request<()>,
  ) -> Result<Response<GrpcGetAllSubscriptionsAndPipelinesResponse>, Status> {
    let subscriptions = self
      .kafka_service
      .get_all_subscriptions()
      .into_iter()
      .map(|(id, s)| GrpcKafkaSubscription {
        id: Some(GrpcGuid::from(id)),
        metadata: Some(GrpcKafkaSubscriptionMetadata {
          subscription_name: s.name().to_owned(),
        }),
        pipelines: s
          .pipelines()
          .into_iter()
          .map(|(id, p)| GrpcSubscriptionPipeline {
            id: Some(GrpcGuid::from(id)),
            metadata: Some(GrpcPipelineMetadata { name: p.name() }),
          })
          .collect(),
      })
      .collect();

    Ok(Response::new(GrpcGetAllSubscriptionsAndPipelinesResponse { subscriptions }))
  }

  type ExecutePipelineAndProduceToKafkaStream =
    Pin<Box<dyn Stream<Item = Result<GrpcPipelinePartExecutionResult, Status>> + Send + Sync + 'static>>;

  async fn execute_pipeline_and_produce_to_kafka(
    &self,
    request: Request<GrpcExecutePipelineAndProduceKafkaRequest>,
  ) -> Result<Response<Self::ExecutePipelineAndProduceToKafkaStream>, Status> {
    let (sender, receiver) = mpsc::channel(4);
    let kafka_handler = KafkaService::create_kafka_events_handler(request.get_ref().producer_metadata.as_ref())?;
    let kafka_handler = Box::new(kafka_handler) as Box<dyn PipelineEventsHandler>;
    let grpc_handler = Box::new(GrpcPipelineEventsHandler::new(sender)) as Box<dyn PipelineEventsHandler>;

    let handler = DelegatingEventsHandler::new(vec![kafka_handler, grpc_handler]);
    let handler = Arc::new(handler) as Arc<dyn PipelineEventsHandler>;
    let dto = PipelineExecutionDto::new(self.pipeline_parts.clone(), handler);

    let context_values = match self
      .cv_service
      .reclaim_context_values(&request.get_ref().pipeline_request.as_ref().unwrap().context_values_ids)
    {
      Ok(context_values) => context_values,
      Err(not_found_id) => {
        let message = format!("Failed to find context value for id {}", not_found_id);
        return Err(Status::invalid_argument(message));
      }
    };

    tokio::task::spawn_blocking(move || {
      let pipeline = request
        .get_ref()
        .pipeline_request
        .as_ref()
        .expect("Pipeline request must be supplied")
        .pipeline
        .as_ref()
        .expect("Pipeline must be supplied");

      let request = request.get_ref();
      let case_info = request.case_info.as_ref().expect("Case info must be supplied");
      let case_name: Rc<str> = case_info.case_name.clone().into();
      let process_name = case_info.process_name.clone();
      let pipeline_id = Uuid::parse_str(request.pipeline_id.as_ref().expect("Must be supplied").guid.as_str());
      let subscription_id = Uuid::parse_str(request.subscription_id.as_ref().expect("Must be supplied").guid.as_str());
      let pipeline_name = request.pipeline_name.clone();
      let subscription_name = request.subscription_name.clone();

      let context = KafkaService::create_pipeline_execution_context_from_proxy(pipeline, &context_values, &dto);

      let execution_result = context.execute_grpc_pipeline(move |context| {
        context.put_concrete(SUBSCRIPTION_ID_KEY.key(), subscription_id.unwrap());
        context.put_concrete(PIPELINE_ID_KEY.key(), pipeline_id.unwrap());
        context.put_concrete(SUBSCRIPTION_NAME_KEY.key(), subscription_name.into());
        context.put_concrete(PIPELINE_NAME_KEY.key(), pipeline_name.into());

        context.put_concrete(PROCESS_NAME_KEY.key(), process_name.into());
        context.put_concrete(
          CASE_NAME_KEY.key(),
          CaseName {
            display_name: case_name.clone(),
            name_parts: vec![case_name],
          },
        );

        Ok(())
      });

      match execution_result {
        Ok((uuid, _)) => {
          dto
            .events_handler
            .handle(&PipelineEvent::FinalResult(PipelineFinalResult::Success(uuid)));
        }
        Err(err) => {
          dto
            .events_handler
            .handle(&PipelineEvent::FinalResult(PipelineFinalResult::Error(err.to_string())));
        }
      };
    });

    Ok(Response::new(Box::pin(ReceiverStream::new(receiver))))
  }
}
