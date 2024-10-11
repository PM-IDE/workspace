use super::events::events_handler::{PipelineEventsHandler, PipelineFinalResult};
use crate::event_log::bxes::bxes_to_xes_converter::{read_bxes_events, BxesToXesReadError};
use crate::event_log::core::event_log::EventLog;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::ficus_proto::grpc_kafka_service_server::GrpcKafkaService;
use crate::ficus_proto::{grpc_kafka_result, GrpcExecutePipelineAndProduceKafkaRequest, GrpcGuid, GrpcKafkaConnectionMetadata, GrpcKafkaFailedResult, GrpcKafkaResult, GrpcKafkaSuccessResult, GrpcPipelineExecutionRequest, GrpcPipelinePartExecutionResult, GrpcSubscribeForKafkaTopicRequest, GrpcSubscribeToKafkaAndProduceToKafka, GrpcUnsubscribeFromKafkaRequest};
use crate::grpc::events::events_handler::PipelineEvent;
use crate::grpc::events::grpc_events_handler::GrpcPipelineEventsHandler;
use crate::grpc::events::kafka_events_handler::{KafkaEventsHandler, PipelineEventsProducer};
use crate::grpc::pipeline_executor::ServicePipelineExecutionContext;
use crate::pipelines::keys::context_keys::{CASE_NAME, EVENT_LOG_KEY, PROCESS_NAME, UNSTRUCTURED_METADATA};
use crate::pipelines::pipeline_parts::PipelineParts;
use crate::utils::user_data::user_data::UserData;
use bxes::models::domain::bxes_value::BxesValue;
use bxes_kafka::consumer::bxes_kafka_consumer::{BxesKafkaConsumer, BxesKafkaTrace};
use futures::Stream;
use rdkafka::error::KafkaError;
use rdkafka::ClientConfig;
use std::cell::RefCell;
use std::collections::HashMap;
use std::pin::Pin;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub struct KafkaService {
    names_to_logs: Arc<Mutex<HashMap<String, XesEventLogImpl>>>,
    pipeline_parts: Arc<Box<PipelineParts>>,
    consumers_states: Arc<Mutex<HashMap<Uuid, ConsumerState>>>,
}

impl KafkaService {
    pub fn new() -> Self {
        Self {
            names_to_logs: Arc::new(Mutex::new(HashMap::new())),
            pipeline_parts: Arc::new(Box::new(PipelineParts::new())),
            consumers_states: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

enum ConsumerState {
    Consuming,
    ShutdownRequested,
}

const KAFKA_CASE_NAME: &'static str = "case_name";
const KAFKA_PROCESS_NAME: &'static str = "process_name";

#[tonic::async_trait]
impl GrpcKafkaService for KafkaService {
    async fn subscribe_for_kafka_topic_external(
        &self,
        request: Request<GrpcSubscribeToKafkaAndProduceToKafka>,
    ) -> Result<Response<GrpcKafkaResult>, Status> {
        let handler = Self::create_kafka_events_handler(request.get_ref().producer_metadata.as_ref())?;

        let creation_dto = self.create_kafka_creation_dto(Arc::new(handler));
        let request = request.get_ref().request.as_ref().expect("Request should be supplied");
        let result = match Self::subscribe_to_kafka_topic(creation_dto, request.clone()) {
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
        let creation_dto = self.create_kafka_creation_dto(Arc::new(
            Box::new(GrpcPipelineEventsHandler::new(sender)) as Box<dyn PipelineEventsHandler>
        ));

        match Self::subscribe_to_kafka_topic(creation_dto, request.get_ref().clone()) {
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

        let mut states = self.consumers_states.lock().expect("Should take lock");
        let result = match states.get_mut(&uuid) {
            None => grpc_kafka_result::Result::Failure(GrpcKafkaFailedResult {
                error_message: "There is not state for the supplied consumer uuid".to_string(),
            }),
            Some(state) => {
                *state = ConsumerState::ShutdownRequested;
                grpc_kafka_result::Result::Success(GrpcKafkaSuccessResult {
                    subscription_id: Some(request.get_ref().subscription_id.as_ref().unwrap().clone()),
                })
            }
        };

        Ok(Response::new(GrpcKafkaResult { result: Some(result) }))
    }

    async fn execute_pipeline_and_produce_to_kafka(
        &self,
        request: Request<GrpcExecutePipelineAndProduceKafkaRequest>
    ) -> Result<Response<()>, Status> {
        let handler = Self::create_kafka_events_handler(request.get_ref().producer_metadata.as_ref())?;
        let pipeline_error = Status::invalid_argument("pipeline_request");
        let pipeline_request = request.get_ref().pipeline_request.as_ref().ok_or(pipeline_error)?;

        let dto = PipelineExecutionDto::new(
            self.pipeline_parts.clone(),
            Arc::new(handler)
        );

        let context = Self::create_pipeline_execution_context(pipeline_request, &dto);
        let case_error = Status::invalid_argument("case_info");
        let case_name = request.get_ref().case_info.as_ref().ok_or(case_error)?.case_name.clone();

        let execution_result = context.execute_grpc_pipeline(move |context| {
            context.put_concrete(CASE_NAME.key(), case_name);
        });

        match execution_result {
            Ok(_) => Ok(Response::new(())),
            Err(err) => {
                let message = format!("Failed to execute pipeline, error: {}", err.to_string());
                Err(Status::internal(message))
            }
        }
    }
}

#[derive(Debug)]
enum XesFromBxesKafkaTraceCreatingError {
    CaseNameNotFound,
    CaseNameNotString,
    ProcessNameNotFound,
    ProcessNameNotString,
    BxesToXexConversionError(BxesToXesReadError),
}

#[derive(Clone)]
struct PipelineExecutionDto {
    pipeline_parts: Arc<Box<PipelineParts>>,
    events_handler: Arc<Box<dyn PipelineEventsHandler>>,
}

impl PipelineExecutionDto {
    pub fn new(pipeline_parts: Arc<Box<PipelineParts>>, events_handler: Arc<Box<dyn PipelineEventsHandler>>) -> Self {
        Self {
            pipeline_parts,
            events_handler
        }
    }
}

#[derive(Clone)]
struct KafkaConsumerCreationDto {
    uuid: Uuid,
    consumer_states: Arc<Mutex<HashMap<Uuid, ConsumerState>>>,
    names_to_logs: Arc<Mutex<HashMap<String, XesEventLogImpl>>>,
    pipeline_execution_dto: PipelineExecutionDto,
}

impl KafkaConsumerCreationDto {
    pub fn new(
        consumer_states: Arc<Mutex<HashMap<Uuid, ConsumerState>>>,
        names_to_logs: Arc<Mutex<HashMap<String, XesEventLogImpl>>>,
        pipeline_execution_dto: PipelineExecutionDto
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            consumer_states,
            names_to_logs,
            pipeline_execution_dto
        }
    }
}

struct LogUpdateResult {
    pub process_name: String,
    pub case_name: String,
    pub new_log: XesEventLogImpl,
    pub unstructured_metadata: Vec<(String, String)>,
}

impl KafkaService {
    fn create_kafka_creation_dto(&self, events_handler: Arc<Box<dyn PipelineEventsHandler>>) -> KafkaConsumerCreationDto {
        KafkaConsumerCreationDto::new(
            self.consumers_states.clone(),
            self.names_to_logs.clone(),
            PipelineExecutionDto::new(self.pipeline_parts.clone(), events_handler)
        )
    }

    fn subscribe_to_kafka_topic(
        creation_dto: KafkaConsumerCreationDto,
        request: GrpcSubscribeForKafkaTopicRequest,
    ) -> Result<Uuid, KafkaError> {
        let consumer_uuid = creation_dto.uuid;
        match Self::spawn_consumer(request, creation_dto) {
            Ok(_) => Ok(consumer_uuid),
            Err(err) => Err(err),
        }
    }

    fn spawn_consumer(request: GrpcSubscribeForKafkaTopicRequest, dto: KafkaConsumerCreationDto) -> Result<(), KafkaError> {
        let mut consumer = match Self::create_consumer(&request) {
            Ok(consumer) => consumer,
            Err(err) => {
                println!("Failed to create kafka consumer: {}", err.to_string());
                return Err(err);
            }
        };

        tokio::spawn(async move {
            let handle = tokio::task::spawn_blocking(move || {
                Self::subscribe(&mut consumer, dto.clone());

                loop {
                    let should_stop = Self::execute_consumer_routine(&mut consumer, &request, dto.clone());

                    if should_stop {
                        consumer.unsubscribe();
                        return;
                    }
                }
            });

            handle.await
        });

        Ok(())
    }

    fn create_consumer(request: &GrpcSubscribeForKafkaTopicRequest) -> Result<BxesKafkaConsumer, KafkaError> {
        let metadata = match request.kafka_connection_metadata.as_ref() {
            None => return Err(KafkaError::Subscription("Kafka connection metadata was not provided".to_string())),
            Some(metadata) => metadata,
        };

        let mut config = ClientConfig::new();

        for metadata_pair in &metadata.metadata {
            config.set(metadata_pair.key.to_owned(), metadata_pair.value.to_owned());
        }

        let consumer = config.create()?;

        Ok(BxesKafkaConsumer::new(metadata.topic_name.to_owned(), consumer))
    }

    fn subscribe(consumer: &mut BxesKafkaConsumer, creation_dto: KafkaConsumerCreationDto) {
        match consumer.subscribe() {
            Ok(_) => {
                let mut states = creation_dto.consumer_states.lock().expect("Should take lock");
                if states.contains_key(&creation_dto.uuid) {
                    if let Some(ConsumerState::ShutdownRequested) = states.get(&creation_dto.uuid) {
                        consumer.unsubscribe();
                        return;
                    } else {
                        println!("Invalid state: consumer already in subscribed state");
                    }
                }

                states.insert(creation_dto.uuid.clone(), ConsumerState::Consuming);
            }
            Err(err) => println!("Failed to subscribe to topic: {:?}", err),
        };
    }

    fn execute_consumer_routine(
        consumer: &mut BxesKafkaConsumer,
        request: &GrpcSubscribeForKafkaTopicRequest,
        dto: KafkaConsumerCreationDto,
    ) -> bool {
        if !dto.pipeline_execution_dto.events_handler.is_alive() || Self::is_unsubscribe_requested(dto.clone()) {
            return true;
        }

        match consumer.consume() {
            Ok(trace) => match trace {
                Some(trace) => Self::process_kafka_trace(trace, request, dto.clone()),
                None => {}
            },
            Err(err) => {
                print!("Failed to read messages from kafka: {:?}", err)
            }
        };

        false
    }

    fn is_unsubscribe_requested(dto: KafkaConsumerCreationDto) -> bool {
        let states = dto.consumer_states.lock().expect("Should take lock");
        if let Some(ConsumerState::ShutdownRequested) = states.get(&dto.uuid) {
            true
        } else {
            false
        }
    }

    fn process_kafka_trace(trace: BxesKafkaTrace, request: &GrpcSubscribeForKafkaTopicRequest, dto: KafkaConsumerCreationDto) {
        let update_result = match Self::update_log(dto.names_to_logs.clone(), trace) {
            Ok(update_result) => update_result,
            Err(_) => return (),
        };

        let pipeline_req = request.pipeline_request.as_ref().expect("Pipeline should be supplied");
        let context = Self::create_pipeline_execution_context(pipeline_req, &dto.pipeline_execution_dto);

        let execution_result = context.execute_grpc_pipeline(move |context| {
            context.put_concrete(EVENT_LOG_KEY.key(), update_result.new_log);
            context.put_concrete(PROCESS_NAME.key(), update_result.process_name);
            context.put_concrete(CASE_NAME.key(), update_result.case_name);
            context.put_concrete(UNSTRUCTURED_METADATA.key(), update_result.unstructured_metadata);
        });

        if let Err(err) = execution_result {
            let err = PipelineFinalResult::Error(err.to_string());
            dto.pipeline_execution_dto.events_handler.handle(PipelineEvent::FinalResult(err));
        }
    }

    fn create_pipeline_execution_context<'a>(
        pipeline_req: &'a GrpcPipelineExecutionRequest,
        dto: &PipelineExecutionDto
    ) -> ServicePipelineExecutionContext<'a> {
        let grpc_pipeline = pipeline_req.pipeline.as_ref().expect("Pipeline should be supplied");

        ServicePipelineExecutionContext::new(
            grpc_pipeline,
            &pipeline_req.initial_context,
            dto.pipeline_parts.clone(),
            dto.events_handler.clone(),
        )
    }

    fn update_log(
        names_to_logs: Arc<Mutex<HashMap<String, XesEventLogImpl>>>,
        trace: BxesKafkaTrace,
    ) -> Result<LogUpdateResult, XesFromBxesKafkaTraceCreatingError> {
        let metadata = trace.metadata();
        let mut names_to_logs = names_to_logs.lock();
        let names_to_logs = match names_to_logs.as_mut() {
            Ok(names_to_logs) => names_to_logs,
            Err(_) => panic!("Failed to acquire a names_to_logs map from mutex"),
        };

        let process_name = if let Some(process_name) = metadata.get(KAFKA_PROCESS_NAME) {
            if let BxesValue::String(process_name) = process_name.as_ref().as_ref() {
                process_name.as_ref().as_ref().to_owned()
            } else {
                return Err(XesFromBxesKafkaTraceCreatingError::ProcessNameNotString);
            }
        } else {
            return Err(XesFromBxesKafkaTraceCreatingError::ProcessNameNotFound);
        };

        if let Some(case_name) = metadata.get(KAFKA_CASE_NAME) {
            if let BxesValue::String(case_name) = case_name.as_ref().as_ref() {
                let case_name = case_name.as_ref().as_ref();
                if !names_to_logs.contains_key(case_name) {
                    let new_log = XesEventLogImpl::empty();
                    names_to_logs.insert(case_name.to_owned(), new_log);
                }

                let mut existing_log = names_to_logs.get_mut(case_name).expect("Log should be present");

                let xes_trace = match read_bxes_events(trace.events()) {
                    Ok(xes_trace) => xes_trace,
                    Err(err) => return Err(XesFromBxesKafkaTraceCreatingError::BxesToXexConversionError(err)),
                };

                let xes_trace = Rc::new(RefCell::new(xes_trace));
                existing_log.push(xes_trace);

                let result = LogUpdateResult {
                    process_name,
                    case_name: case_name.to_owned(),
                    new_log: existing_log.clone(),
                    unstructured_metadata: metadata
                        .iter()
                        .map(|pair| {
                            if pair.0 == KAFKA_CASE_NAME || pair.0 == KAFKA_PROCESS_NAME {
                                None
                            } else {
                                if let BxesValue::String(value) = pair.1.as_ref().as_ref() {
                                    Some((pair.0.to_owned(), value.as_ref().as_ref().to_owned()))
                                } else {
                                    None
                                }
                            }
                        })
                        .filter(|kv| kv.is_some())
                        .map(|kv| kv.unwrap())
                        .collect(),
                };

                Ok(result)
            } else {
                Err(XesFromBxesKafkaTraceCreatingError::CaseNameNotString)
            }
        } else {
            Err(XesFromBxesKafkaTraceCreatingError::CaseNameNotFound)
        }
    }

    fn create_kafka_events_handler(producer_metadata: Option<&GrpcKafkaConnectionMetadata>) -> Result<Box<dyn PipelineEventsHandler>, Status> {
        let producer_metadata = match producer_metadata.as_ref() {
            None => return Err(Status::invalid_argument("Producer metadata must be provided")),
            Some(metadata) => metadata,
        };

        let producer = match PipelineEventsProducer::create(producer_metadata) {
            Ok(producer) => producer,
            Err(err) => {
                let message = format!("Failed to create producer: {}", err.to_string());
                return Err(Status::invalid_argument(message));
            }
        };

        Ok(Box::new(KafkaEventsHandler::new(producer)) as Box<dyn PipelineEventsHandler>)
    }
}
