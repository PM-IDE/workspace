use crate::ficus_proto::grpc_kafka_service_server::GrpcKafkaService;
use crate::ficus_proto::{
    grpc_kafka_result, GrpcAddPipelineRequest, GrpcAddPipelineStreamRequest, GrpcExecutePipelineAndProduceKafkaRequest, GrpcGuid,
    GrpcKafkaFailedResult, GrpcKafkaResult, GrpcKafkaSuccessResult, GrpcPipelinePartExecutionResult, GrpcRemoveAllPipelinesRequest,
    GrpcRemovePipelineRequest, GrpcSubscribeToKafkaRequest, GrpcUnsubscribeFromKafkaRequest,
};
use crate::grpc::context_values_service::ContextValueService;
use crate::grpc::events::delegating_events_handler::DelegatingEventsHandler;
use crate::grpc::events::events_handler::{PipelineEvent, PipelineEventsHandler, PipelineFinalResult};
use crate::grpc::events::grpc_events_handler::GrpcPipelineEventsHandler;
use crate::grpc::kafka::kafka_service::KafkaService;
use crate::grpc::kafka::models::PipelineExecutionDto;
use crate::pipelines::keys::context_keys::{CASE_NAME, PROCESS_NAME};
use crate::pipelines::pipeline_parts::PipelineParts;
use crate::utils::user_data::user_data::UserData;
use futures::Stream;
use std::pin::Pin;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub struct GrpcKafkaServiceImpl {
    context_values_service: Arc<Mutex<ContextValueService>>,
    kafka_service: KafkaService,
    pipeline_parts: Arc<Box<PipelineParts>>,
}

impl GrpcKafkaServiceImpl {
    pub fn new(context_values_service: Arc<Mutex<ContextValueService>>) -> Self {
        Self {
            context_values_service,
            kafka_service: KafkaService::new(),
            pipeline_parts: Arc::new(Box::new(PipelineParts::new())),
        }
    }
}

impl GrpcGuid {
    pub fn to_uuid(&self) -> Result<Uuid, Status> {
        match Uuid::from_str(&self.guid) {
            Ok(uuid) => Ok(uuid),
            Err(_) => Err(Status::invalid_argument("Invalid uuid")),
        }
    }
}

impl From<Uuid> for GrpcGuid {
    fn from(value: Uuid) -> Self {
        GrpcGuid { guid: value.to_string() }
    }
}

impl GrpcKafkaResult {
    pub fn success(uuid: Uuid) -> GrpcKafkaResult {
        GrpcKafkaResult {
            result: Some(grpc_kafka_result::Result::Success(GrpcKafkaSuccessResult {
                id: Some(GrpcGuid::from(uuid)),
            })),
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

    async fn add_pipeline_to_subscription(&self, request: Request<GrpcAddPipelineRequest>) -> Result<Response<GrpcKafkaResult>, Status> {
        let uuid = request
            .get_ref()
            .subscription_id
            .as_ref()
            .expect("Subscription id must be provided")
            .to_uuid()?;

        let handler = KafkaService::create_kafka_events_handler(request.get_ref().results_to_kafka_topic.as_ref())?;

        let request = request.get_ref().pipeline_request.as_ref().unwrap().clone();
        let pipeline_id = self.kafka_service.add_execution_request(uuid, handler, request);

        Ok(Response::new(GrpcKafkaResult::success(pipeline_id)))
    }

    type AddPipelineToSubscriptionStreamStream =
        Pin<Box<dyn Stream<Item = Result<GrpcPipelinePartExecutionResult, Status>> + Send + Sync + 'static>>;

    async fn add_pipeline_to_subscription_stream(
        &self,
        request: Request<GrpcAddPipelineStreamRequest>,
    ) -> Result<Response<Self::AddPipelineToSubscriptionStreamStream>, Status> {
        let uuid = request
            .get_ref()
            .subscription_id
            .as_ref()
            .expect("Subscription id must be provided")
            .to_uuid()?;

        let (sender, receiver) = mpsc::channel(4);
        let handler = GrpcPipelineEventsHandler::new(sender);

        let request = request.get_ref().pipeline_request.as_ref().unwrap().clone();

        self.kafka_service.add_execution_request(uuid, handler, request);

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

        Ok(Response::new(GrpcKafkaResult::success(pipeline_id.clone())))
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

        Ok(Response::new(GrpcKafkaResult::success(subscription_id.clone())))
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

        let handler = Box::new(DelegatingEventsHandler::new(vec![kafka_handler, grpc_handler]));
        let handler = handler as Box<dyn PipelineEventsHandler>;
        let dto = PipelineExecutionDto::new(self.pipeline_parts.clone(), Arc::new(handler));

        let mut cv_service = self.context_values_service.lock();
        let cv_service = cv_service.as_mut().expect("Must acquire lock");

        let context_values =
            match cv_service.reclaim_context_values(&request.get_ref().pipeline_request.as_ref().unwrap().context_values_ids) {
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

            let case_info = request.get_ref().case_info.as_ref().expect("Case info must be supplied");
            let case_name = case_info.case_name.clone();
            let process_name = case_info.process_name.clone();

            let context = KafkaService::create_pipeline_execution_context_from_proxy(pipeline, &context_values, &dto);

            let execution_result = context.execute_grpc_pipeline(move |context| {
                context.put_concrete(PROCESS_NAME.key(), process_name);
                context.put_concrete(CASE_NAME.key(), case_name);
            });

            match execution_result {
                Ok((uuid, _)) => {
                    dto.events_handler
                        .handle(&PipelineEvent::FinalResult(PipelineFinalResult::Success(uuid)));
                }
                Err(err) => {
                    dto.events_handler
                        .handle(&PipelineEvent::FinalResult(PipelineFinalResult::Error(err.to_string())));
                }
            };
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(receiver))))
    }
}
