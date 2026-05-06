use crate::{
  ficus_proto::{
    GrpcContextKeyValue, GrpcGuid, GrpcKafkaConnectionMetadata, GrpcKafkaFailedResult, GrpcKafkaSuccessResult, GrpcPipeline,
    GrpcPipelineExecutionRequest, GrpcPipelineStreamingConfiguration, GrpcSubscribeToKafkaRequest, grpc_kafka_result,
  },
  grpc::{
    context_values_service::ContextValueService,
    events::{
      events_handler::{EmptyPipelineEventsHandler, PipelineEvent, PipelineEventsHandler, PipelineFinalResult},
      grpc_events_handler::GrpcPipelineEventsHandler,
      kafka_events_handler::{KafkaEventsHandler, PipelineEventsProducer, ProcessCaseMetadata},
    },
    kafka::{
      models::{ExtractedTraceMetadata, KafkaConsumerCreationDto, PipelineExecutionDto},
      streaming::{
        configs::StreamingConfiguration,
        processors::{KafkaTraceProcessingContext, TracesProcessor},
      },
    },
    logs_handler::ConsoleLogMessageHandler,
    pipeline_executor::ServicePipelineExecutionContext,
  },
};
use bxes_kafka::consumer::bxes_kafka_consumer::{BxesKafkaConsumer, BxesKafkaError, BxesKafkaTrace};
use ficus::{
  features::cases::CaseName,
  pipelines::{
    context::LogMessageHandler,
    keys::context_keys::{
      PIPELINE_ID_KEY, PIPELINE_NAME_KEY, PROCESS_NAME_KEY, SUBSCRIPTION_ID_KEY, SUBSCRIPTION_NAME_KEY, UNSTRUCTURED_METADATA_KEY,
    },
    pipeline_parts::PipelineParts,
  },
  utils::user_data::user_data::UserData,
};
use log::{debug, error, warn};
use rdkafka::{ClientConfig, error::KafkaError};
use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};
use tonic::Status;
use uuid::Uuid;

#[derive(Clone)]
pub struct KafkaSubscriptionPipeline {
  request: GrpcPipelineExecutionRequest,
  execution_dto: PipelineExecutionDto,
  name: String,
  processor: TracesProcessor,
}

impl KafkaSubscriptionPipeline {
  fn new(request: GrpcPipelineExecutionRequest, execution_dto: PipelineExecutionDto, name: String, processor: TracesProcessor) -> Self {
    Self {
      request,
      execution_dto,
      name,
      processor,
    }
  }
}

impl KafkaSubscriptionPipeline {
  pub fn name(&self) -> String {
    self.name.clone()
  }
}

#[derive(Clone)]
pub struct KafkaSubscription {
  name: Arc<str>,
  pipelines: HashMap<Uuid, KafkaSubscriptionPipeline>,
}

impl KafkaSubscription {
  fn new(name: Arc<str>) -> Self {
    Self {
      name,
      pipelines: HashMap::new(),
    }
  }

  pub fn name(&self) -> &str {
    self.name.as_ref()
  }
  pub fn pipelines(&self) -> Vec<(Uuid, KafkaSubscriptionPipeline)> {
    self.pipelines.iter().map(|p| (*p.0, p.1.clone())).collect()
  }
}

pub struct KafkaService {
  pipeline_parts: Arc<PipelineParts>,
  subscriptions_to_execution_requests: Arc<Mutex<HashMap<Uuid, KafkaSubscription>>>,
  cv_service: Arc<ContextValueService>,
  logger: ConsoleLogMessageHandler,
}

impl KafkaService {
  pub fn new(pipeline_parts: Arc<PipelineParts>, cv_service: Arc<ContextValueService>) -> Self {
    Self {
      pipeline_parts,
      subscriptions_to_execution_requests: Arc::new(Mutex::new(HashMap::new())),
      logger: ConsoleLogMessageHandler::new(),
      cv_service,
    }
  }
}

impl KafkaService {
  pub(super) fn unsubscribe_from_kafka(&self, uuid: Uuid) -> grpc_kafka_result::Result {
    let mut states = self.subscriptions_to_execution_requests.lock().expect("Should take lock");
    match states.remove(&uuid) {
      None => grpc_kafka_result::Result::Failure(GrpcKafkaFailedResult {
        error_message: "There is not state for the supplied consumer uuid".to_string(),
      }),
      Some(_) => grpc_kafka_result::Result::Success(GrpcKafkaSuccessResult {
        id: Some(GrpcGuid { guid: uuid.to_string() }),
      }),
    }
  }

  fn is_unsubscribe_requested(dto: &KafkaConsumerCreationDto) -> bool {
    let map = dto.subscriptions_to_execution_requests.lock().expect("Must acquire lock");
    !map.contains_key(&dto.uuid)
  }
}

impl KafkaService {
  pub(super) fn subscribe_to_kafka_topic(&self, request: GrpcSubscribeToKafkaRequest) -> Result<Uuid, KafkaError> {
    let name = request.subscription_metadata.as_ref().unwrap().subscription_name.clone();
    let creation_dto = self.create_kafka_creation_dto(name.into());
    let id = creation_dto.uuid;

    match Self::spawn_consumer(request, creation_dto) {
      Ok(_) => Ok(id),
      Err(err) => Err(err),
    }
  }

  fn spawn_consumer(request: GrpcSubscribeToKafkaRequest, dto: KafkaConsumerCreationDto) -> Result<(), KafkaError> {
    let mut consumer = match Self::create_consumer(&request) {
      Ok(consumer) => consumer,
      Err(err) => {
        error!("Failed to create kafka consumer: {}", err);
        return Err(err);
      }
    };

    match consumer.subscribe() {
      Ok(_) => {
        let mut map = dto.subscriptions_to_execution_requests.lock().expect("Must acquire lock");
        map.insert(dto.uuid, KafkaSubscription::new(dto.name.clone()));
      }
      Err(err) => {
        return match err {
          BxesKafkaError::Kafka(err) => Err(err),
          BxesKafkaError::Bxes(_) => Err(KafkaError::Subscription("Failed to subscribe".to_string())),
        };
      }
    }

    tokio::spawn(async move {
      let handle = tokio::task::spawn_blocking(move || {
        loop {
          let should_stop = Self::execute_consumer_routine(&mut consumer, &dto);

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
    let Some(metadata) = request.connection_metadata.as_ref() else {
      const ERROR_MESSAGE: &str = "Kafka connection metadata was not provided";
      return Err(KafkaError::Subscription(ERROR_MESSAGE.to_string()));
    };

    let mut config = ClientConfig::new();

    for metadata_pair in &metadata.metadata {
      config.set(metadata_pair.key.to_owned(), metadata_pair.value.to_owned());
    }

    let consumer = config.create()?;

    Ok(BxesKafkaConsumer::new(metadata.topic_name.to_owned(), consumer))
  }

  fn execute_consumer_routine(consumer: &mut BxesKafkaConsumer, dto: &KafkaConsumerCreationDto) -> bool {
    if Self::is_unsubscribe_requested(dto) {
      return true;
    }

    match consumer.consume() {
      Ok(trace) => {
        if let Some(trace) = trace {
          Self::process_kafka_trace(trace, dto)
        }
      }
      Err(err) => {
        print!("Failed to read messages from kafka: {:?}", err)
      }
    };

    false
  }

  fn process_kafka_trace(trace: BxesKafkaTrace, dto: &KafkaConsumerCreationDto) {
    let map = dto.subscriptions_to_execution_requests.lock().expect("Must acquire lock");
    let Some(kafka_subscription) = map.get(&dto.uuid).cloned() else {
      return;
    };

    drop(map);

    for (pipeline_id, pipeline) in &kafka_subscription.pipelines {
      let trace = trace.clone();
      let Ok(metadata) = ExtractedTraceMetadata::create_from(trace.metadata()) else {
        continue;
      };

      let execution_dto = PipelineExecutionDto::new(
        Arc::new(PipelineParts::new()),
        Arc::new(EmptyPipelineEventsHandler::new()) as Arc<dyn PipelineEventsHandler>,
      );

      let trace_processing_context = KafkaTraceProcessingContext { execution_dto, trace };

      match pipeline.processor.observe(trace_processing_context) {
        Ok(..) => {
          let metadata = ProcessCaseMetadata {
            pipeline_id: Some(*pipeline_id),
            pipeline_name: Some(pipeline.name.clone().into()),
            subscription_name: Some(dto.name.as_ref().into()),
            subscription_id: Some(dto.uuid),
            process_name: metadata.process.process_name,
            metadata: metadata.unstructured_metadata,
            case_name: CaseName {
              display_name: metadata.case.case_display_name,
              name_parts: metadata.case.case_name_parts,
            },
          };

          pipeline
            .execution_dto
            .events_handler
            .handle(&PipelineEvent::ProcessCaseMetadata(metadata));
        }
        Err(err) => {
          let message = format!("Failed to get update result, err: {}", err);
          dto.logger.handle(&message).expect("Must log message");
        }
      };
    }
  }

  pub(super) fn get_context_values(
    &self,
    sub_id: Uuid,
    pipeline_id: Uuid,
    case_name: &str,
    handler: Arc<GrpcPipelineEventsHandler>,
  ) -> Result<Uuid, Status> {
    let map = self.subscriptions_to_execution_requests.lock().expect("Must acquire lock");
    let Some(kafka_subscription) = map.get(&sub_id).cloned() else {
      warn!("Subscription {} not found. Map: {:?}", sub_id, map.keys());
      return Err(Status::not_found(format!("Failed to find subscription for id {sub_id}")));
    };

    drop(map);

    let Some(pipeline) = kafka_subscription.pipelines.get(&pipeline_id) else {
      warn!("Pipeline {} not found", pipeline_id);
      return Err(Status::not_found(format!("Failed to find pipeline for id {pipeline_id}")));
    };

    let handler = handler as Arc<dyn PipelineEventsHandler>;
    let execution_dto = PipelineExecutionDto::new(Arc::new(PipelineParts::new()), handler);
    let context = Self::create_pipeline_execution_context(&pipeline.request, &execution_dto);

    let result = context.execute_grpc_pipeline_and_fill_context_values(
      |context| {
        pipeline.processor.fill_pipeline_context(context, case_name);

        context.put_concrete(SUBSCRIPTION_ID_KEY.key(), sub_id);
        context.put_concrete(PIPELINE_ID_KEY.key(), pipeline_id);
        context.put_concrete(SUBSCRIPTION_NAME_KEY.key(), kafka_subscription.name.as_ref().into());
        context.put_concrete(PIPELINE_NAME_KEY.key(), pipeline.name.clone().into());

        Ok(())
      },
      self.cv_service.clone(),
    );

    debug!("Finished executing pipeline with result: {result:?}");

    result.map_err(|err| Status::internal(format!("Failed to execute pipeline, err: {}", err)))
  }
}

impl KafkaService {
  pub fn add_execution_request<T: PipelineEventsHandler + 'static>(
    &self,
    subscription_id: Uuid,
    handler: T,
    request: GrpcPipelineExecutionRequest,
    streaming_config: GrpcPipelineStreamingConfiguration,
    pipeline_name: String,
  ) -> Uuid {
    let mut map = self.subscriptions_to_execution_requests.lock().expect("Must acquire lock");
    let pipeline_id = Uuid::new_v4();
    let streaming_config = StreamingConfiguration::new(&streaming_config).unwrap_or(StreamingConfiguration::NotSpecified);
    let kafka_pipeline = self.create_kafka_pipeline(request, handler, pipeline_name, streaming_config);

    match map.get_mut(&subscription_id) {
      None => {
        self.logger.handle("Subscription must be present").expect("Must log");
      }
      Some(subscription) => {
        subscription.pipelines.insert(pipeline_id, kafka_pipeline);
      }
    }

    pipeline_id
  }

  fn create_kafka_pipeline<T: PipelineEventsHandler + 'static>(
    &self,
    request: GrpcPipelineExecutionRequest,
    handler: T,
    pipeline_name: String,
    streaming_config: StreamingConfiguration,
  ) -> KafkaSubscriptionPipeline {
    let handler = Arc::new(handler) as Arc<dyn PipelineEventsHandler>;
    let dto = PipelineExecutionDto::new(self.pipeline_parts.clone(), handler);
    KafkaSubscriptionPipeline::new(request, dto, pipeline_name, streaming_config.create_processor())
  }

  pub fn remove_execution_request(&self, subscription_id: &Uuid, pipeline_id: &Uuid) {
    let mut map = self.subscriptions_to_execution_requests.lock().expect("Must acquire lock");
    if let Some(map) = map.get_mut(subscription_id) {
      map.pipelines.remove(pipeline_id);
    }
  }

  pub fn remove_all_execution_requests(&self, subscription_id: &Uuid) {
    let mut map = self.subscriptions_to_execution_requests.lock().expect("Must acquire lock");
    if let Some(map) = map.get_mut(subscription_id) {
      map.pipelines.clear();
    }
  }

  pub fn get_all_subscriptions(&self) -> Vec<(Uuid, KafkaSubscription)> {
    let map = self.subscriptions_to_execution_requests.lock().expect("Must acquire lock");
    map.iter().map(|s| (*s.0, s.1.clone())).collect()
  }
}

impl KafkaService {
  fn create_kafka_creation_dto(&self, name: Arc<str>) -> KafkaConsumerCreationDto {
    KafkaConsumerCreationDto::new(name, self.subscriptions_to_execution_requests.clone())
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

  pub(super) fn create_kafka_events_handler(producer_metadata: Option<&GrpcKafkaConnectionMetadata>) -> Result<KafkaEventsHandler, Status> {
    producer_metadata
      .ok_or_else(|| Status::invalid_argument("Producer metadata must be provided"))
      .and_then(|metadata| match PipelineEventsProducer::create(metadata) {
        Ok(producer) => Ok(KafkaEventsHandler::new(producer)),
        Err(err) => {
          let message = format!("Failed to create producer: {}", err);
          Err(Status::invalid_argument(message))
        }
      })
  }
}
