use crate::ficus_proto::grpc_kafka_service_server::GrpcKafkaService;
use crate::ficus_proto::{
    grpc_kafka_result, GrpcExecutePipelineAndProduceKafkaRequest, GrpcGuid, GrpcKafkaFailedResult, GrpcKafkaResult, GrpcKafkaSuccessResult,
    GrpcPipelinePartExecutionResult, GrpcSubscribeForKafkaTopicRequest, GrpcSubscribeToKafkaAndProduceToKafka,
    GrpcUnsubscribeFromKafkaRequest,
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

#[tonic::async_trait]
impl GrpcKafkaService for GrpcKafkaServiceImpl {
    async fn subscribe_for_kafka_topic_external(
        &self,
        request: Request<GrpcSubscribeToKafkaAndProduceToKafka>,
    ) -> Result<Response<GrpcKafkaResult>, Status> {
        let handler = KafkaService::create_kafka_events_handler(request.get_ref().producer_metadata.as_ref())?;

        let request = request.get_ref().request.as_ref().expect("Request should be supplied");
        let result = match self.kafka_service.subscribe_to_kafka_topic(Arc::new(handler), request.clone()) {
            Ok(consumer_uuid) => grpc_kafka_result::Result::Success(GrpcKafkaSuccessResult {
                subscription_id: Some(GrpcGuid {
                    guid: consumer_uuid.to_string(),
                }),
            }),
            Err(err) => grpc_kafka_result::Result::Failure(GrpcKafkaFailedResult {
                error_message: err.to_string(),
            }),
        };

        Ok(Response::new(GrpcKafkaResult { result: Some(result) }))
    }

    type SubscribeForKafkaTopicStreamStream =
        Pin<Box<dyn Stream<Item = Result<GrpcPipelinePartExecutionResult, Status>> + Send + Sync + 'static>>;

    async fn subscribe_for_kafka_topic_stream(
        &self,
        request: Request<GrpcSubscribeForKafkaTopicRequest>,
    ) -> Result<Response<Self::SubscribeForKafkaTopicStreamStream>, Status> {
        let (sender, receiver) = mpsc::channel(4);

        let handler = Box::new(GrpcPipelineEventsHandler::new(sender)) as Box<dyn PipelineEventsHandler>;
        let handler = Arc::new(handler);

        match self.kafka_service.subscribe_to_kafka_topic(handler, request.get_ref().clone()) {
            Ok(_) => Ok(Response::new(Box::pin(ReceiverStream::new(receiver)))),
            Err(err) => Err(Status::invalid_argument(err.to_string())),
        }
    }

    async fn unsubscribe_from_kafka_topic(
        &self,
        request: Request<GrpcUnsubscribeFromKafkaRequest>,
    ) -> Result<Response<GrpcKafkaResult>, Status> {
        let uuid = match Uuid::from_str(&request.get_ref().subscription_id.as_ref().unwrap().guid) {
            Ok(uuid) => uuid,
            Err(_) => return Err(Status::invalid_argument("Invalid uuid")),
        };

        let result = self.kafka_service.unsubscribe_from_kafka(uuid);

        Ok(Response::new(GrpcKafkaResult { result: Some(result) }))
    }

    type ExecutePipelineAndProduceToKafkaStream =
        Pin<Box<dyn Stream<Item = Result<GrpcPipelinePartExecutionResult, Status>> + Send + Sync + 'static>>;

    async fn execute_pipeline_and_produce_to_kafka(
        &self,
        request: Request<GrpcExecutePipelineAndProduceKafkaRequest>,
    ) -> Result<Response<Self::ExecutePipelineAndProduceToKafkaStream>, Status> {
        let (sender, receiver) = mpsc::channel(4);
        let kafka_handler = KafkaService::create_kafka_events_handler(request.get_ref().producer_metadata.as_ref())?;
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
