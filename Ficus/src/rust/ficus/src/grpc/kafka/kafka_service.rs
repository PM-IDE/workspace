use crate::event_log::bxes::bxes_to_xes_converter::read_bxes_events;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::ficus_proto::{
    grpc_kafka_result, GrpcContextKeyValue, GrpcGuid, GrpcKafkaConnectionMetadata, GrpcKafkaFailedResult, GrpcKafkaSuccessResult,
    GrpcPipeline, GrpcPipelineExecutionRequest, GrpcSubscribeToKafkaRequest,
};
use crate::grpc::events::events_handler::PipelineEvent;
use crate::grpc::events::events_handler::{PipelineEventsHandler, PipelineFinalResult};
use crate::grpc::events::kafka_events_handler::{KafkaEventsHandler, PipelineEventsProducer};
use crate::grpc::kafka::models::{
    ConsumerState, KafkaConsumerCreationDto, LogUpdateResult, PipelineExecutionDto, XesFromBxesKafkaTraceCreatingError, KAFKA_CASE_NAME,
    KAFKA_PROCESS_NAME,
};
use crate::grpc::pipeline_executor::ServicePipelineExecutionContext;
use crate::pipelines::context::LogMessageHandler;
use crate::pipelines::keys::context_keys::{CASE_NAME, EVENT_LOG_KEY, PROCESS_NAME, UNSTRUCTURED_METADATA};
use crate::pipelines::pipeline_parts::PipelineParts;
use crate::utils::user_data::user_data::UserData;
use bxes::models::domain::bxes_value::BxesValue;
use bxes_kafka::consumer::bxes_kafka_consumer::{BxesKafkaConsumer, BxesKafkaTrace};
use rdkafka::error::KafkaError;
use rdkafka::ClientConfig;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use tonic::Status;
use uuid::Uuid;

pub struct KafkaSubscriptionPipeline {
    request: GrpcPipelineExecutionRequest,
    execution_dto: PipelineExecutionDto,
}

impl KafkaSubscriptionPipeline {
    fn new(request: GrpcPipelineExecutionRequest, execution_dto: PipelineExecutionDto) -> Self {
        Self { request, execution_dto }
    }
}

pub struct KafkaService {
    names_to_logs: Arc<Mutex<HashMap<String, XesEventLogImpl>>>,
    pipeline_parts: Arc<Box<PipelineParts>>,
    consumers_states: Arc<Mutex<HashMap<Uuid, ConsumerState>>>,
    subscriptions_to_execution_requests: Arc<Mutex<HashMap<Uuid, HashMap<Uuid, KafkaSubscriptionPipeline>>>>,
}

impl KafkaService {
    pub fn new() -> Self {
        Self {
            names_to_logs: Arc::new(Mutex::new(HashMap::new())),
            pipeline_parts: Arc::new(Box::new(PipelineParts::new())),
            consumers_states: Arc::new(Mutex::new(HashMap::new())),
            subscriptions_to_execution_requests: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl KafkaService {
    pub(super) fn unsubscribe_from_kafka(&self, uuid: Uuid) -> grpc_kafka_result::Result {
        let mut states = self.consumers_states.lock().expect("Should take lock");
        match states.get_mut(&uuid) {
            None => grpc_kafka_result::Result::Failure(GrpcKafkaFailedResult {
                error_message: "There is not state for the supplied consumer uuid".to_string(),
            }),
            Some(state) => {
                *state = ConsumerState::ShutdownRequested;
                grpc_kafka_result::Result::Success(GrpcKafkaSuccessResult {
                    id: Some(GrpcGuid { guid: uuid.to_string() }),
                })
            }
        }
    }

    fn is_unsubscribe_requested(dto: KafkaConsumerCreationDto) -> bool {
        let states = dto.consumer_states.lock().expect("Should take lock");
        if let Some(ConsumerState::ShutdownRequested) = states.get(&dto.uuid) {
            true
        } else {
            false
        }
    }
}

impl KafkaService {
    pub(super) fn subscribe_to_kafka_topic(&self, request: GrpcSubscribeToKafkaRequest) -> Result<Uuid, KafkaError> {
        let creation_dto = self.create_kafka_creation_dto();
        let id = creation_dto.uuid.clone();

        match Self::spawn_consumer(request, creation_dto) {
            Ok(_) => Ok(id),
            Err(err) => Err(err),
        }
    }

    fn spawn_consumer(request: GrpcSubscribeToKafkaRequest, dto: KafkaConsumerCreationDto) -> Result<(), KafkaError> {
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
                    let should_stop = Self::execute_consumer_routine(&mut consumer, dto.clone());

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

    fn create_consumer(request: &GrpcSubscribeToKafkaRequest) -> Result<BxesKafkaConsumer, KafkaError> {
        let metadata = match request.connection_metadata.as_ref() {
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

    fn execute_consumer_routine(consumer: &mut BxesKafkaConsumer, dto: KafkaConsumerCreationDto) -> bool {
        if Self::is_unsubscribe_requested(dto.clone()) {
            return true;
        }

        match consumer.consume() {
            Ok(trace) => match trace {
                Some(trace) => Self::process_kafka_trace(trace, dto.clone()),
                None => {}
            },
            Err(err) => {
                print!("Failed to read messages from kafka: {:?}", err)
            }
        };

        false
    }

    fn process_kafka_trace(trace: BxesKafkaTrace, dto: KafkaConsumerCreationDto) {
        let update_result = match Self::update_log(dto.names_to_logs.clone(), trace) {
            Ok(update_result) => update_result,
            Err(err) => {
                let message = format!("Failed to get update result, err: {}", err.to_string());
                dto.logger.handle(message.as_str()).expect("Must log message");
                return;
            }
        };

        match dto
            .subscriptions_to_execution_requests
            .lock()
            .expect("Must acquire lock")
            .get(&dto.uuid)
        {
            None => {}
            Some(subscription_pipelines) => {
                for pipeline in subscription_pipelines {
                    let pipeline = pipeline.1;

                    if !pipeline.execution_dto.events_handler.is_alive() {
                        continue;
                    }

                    let update_result = update_result.clone();
                    let context = Self::create_pipeline_execution_context(&pipeline.request, &pipeline.execution_dto);

                    let execution_result = context.execute_grpc_pipeline(move |context| {
                        context.put_concrete(EVENT_LOG_KEY.key(), update_result.new_log);
                        context.put_concrete(PROCESS_NAME.key(), update_result.process_name);
                        context.put_concrete(CASE_NAME.key(), update_result.case_name);
                        context.put_concrete(UNSTRUCTURED_METADATA.key(), update_result.unstructured_metadata);
                    });

                    if let Err(err) = execution_result {
                        let err = PipelineFinalResult::Error(err.to_string());
                        pipeline.execution_dto.events_handler.handle(&PipelineEvent::FinalResult(err));
                    }
                }
            }
        }
    }
}

impl KafkaService {
    pub fn add_execution_request<T: PipelineEventsHandler + 'static>(
        &self,
        subscription_id: Uuid,
        handler: T,
        request: GrpcPipelineExecutionRequest,
    ) -> Uuid {
        let mut map = self.subscriptions_to_execution_requests.lock().expect("Must acquire lock");
        let pipeline_id = Uuid::new_v4();
        let kafka_pipeline = self.create_kafka_pipeline(request, handler);

        match map.get_mut(&subscription_id) {
            None => {
                let pipelines_map = HashMap::from_iter([(pipeline_id, kafka_pipeline)]);
                map.insert(subscription_id, pipelines_map);
            }
            Some(pipelines_map) => {
                pipelines_map.insert(pipeline_id, kafka_pipeline);
            }
        }

        pipeline_id
    }

    fn create_kafka_pipeline<T: PipelineEventsHandler + 'static>(
        &self,
        request: GrpcPipelineExecutionRequest,
        handler: T,
    ) -> KafkaSubscriptionPipeline {
        let handler = Arc::new(Box::new(handler) as Box<dyn PipelineEventsHandler>);
        let dto = PipelineExecutionDto::new(self.pipeline_parts.clone(), handler);
        KafkaSubscriptionPipeline::new(request, dto)
    }

    pub fn remove_execution_request(&self, subscription_id: &Uuid, pipeline_id: &Uuid) {
        let mut map = self.subscriptions_to_execution_requests.lock().expect("Must acquire lock");
        if let Some(map) = map.get_mut(subscription_id) {
            map.remove(pipeline_id);
        }
    }

    pub fn remove_all_execution_requests(&self, subscription_id: &Uuid) {
        let mut map = self.subscriptions_to_execution_requests.lock().expect("Must acquire lock");
        if let Some(map) = map.get_mut(subscription_id) {
            map.clear();
        }
    }
}

impl KafkaService {
    fn create_kafka_creation_dto(&self) -> KafkaConsumerCreationDto {
        KafkaConsumerCreationDto::new(
            self.consumers_states.clone(),
            self.names_to_logs.clone(),
            self.subscriptions_to_execution_requests.clone(),
        )
    }

    pub(super) fn create_pipeline_execution_context_from_proxy<'a>(
        pipeline: &'a GrpcPipeline,
        context_values: &'a Vec<GrpcContextKeyValue>,
        dto: &PipelineExecutionDto,
    ) -> ServicePipelineExecutionContext<'a> {
        ServicePipelineExecutionContext::new(pipeline, context_values, dto.pipeline_parts.clone(), dto.events_handler.clone())
    }

    fn create_pipeline_execution_context<'a>(
        pipeline_req: &'a GrpcPipelineExecutionRequest,
        dto: &PipelineExecutionDto,
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

        let process_name = Self::string_value_or_err(metadata, KAFKA_PROCESS_NAME)?;
        let case_name = Self::string_value_or_err(metadata, KAFKA_CASE_NAME)?;

        if !names_to_logs.contains_key(case_name.as_str()) {
            let new_log = XesEventLogImpl::empty();
            names_to_logs.insert(case_name.to_owned(), new_log);
        }

        let existing_log = names_to_logs.get_mut(case_name.as_str()).expect("Log should be present");

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
            unstructured_metadata: Self::metadata_to_string_string_pairs(metadata),
        };

        Ok(result)
    }

    fn string_value_or_err(
        metadata: &HashMap<String, Rc<Box<BxesValue>>>,
        key_name: &str,
    ) -> Result<String, XesFromBxesKafkaTraceCreatingError> {
        if let Some(value) = metadata.get(key_name) {
            if let BxesValue::String(process_name) = value.as_ref().as_ref() {
                Ok(process_name.as_ref().as_ref().to_owned())
            } else {
                Err(XesFromBxesKafkaTraceCreatingError::MetadataValueIsNotAString(key_name.to_string()))
            }
        } else {
            Err(XesFromBxesKafkaTraceCreatingError::MetadataValueNotFound(key_name.to_string()))
        }
    }

    fn metadata_to_string_string_pairs(metadata: &HashMap<String, Rc<Box<BxesValue>>>) -> Vec<(String, String)> {
        metadata
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
            .collect()
    }

    pub(super) fn create_kafka_events_handler(
        producer_metadata: Option<&GrpcKafkaConnectionMetadata>,
    ) -> Result<KafkaEventsHandler, Status> {
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

        Ok(KafkaEventsHandler::new(producer))
    }
}
