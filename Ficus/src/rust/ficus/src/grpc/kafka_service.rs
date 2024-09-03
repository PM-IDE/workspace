use super::events::events_handler::{PipelineEventsHandler, PipelineFinalResult};
use crate::event_log::bxes::bxes_to_xes_converter::{read_bxes_events, BxesToXesReadError};
use crate::event_log::core::event_log::EventLog;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::ficus_proto::grpc_kafka_service_server::GrpcKafkaService;
use crate::ficus_proto::{
    grpc_kafka_result, GrpcGuid, GrpcKafkaFailedResult, GrpcKafkaResult, GrpcKafkaSuccessResult,
    GrpcSubscribeForKafkaTopicRequest, GrpcUnsubscribeFromKafkaRequest,
};
use crate::grpc::events::events_handler::PipelineEvent;
use crate::grpc::events::kafka_events_handler::KafkaEventsHandler;
use crate::grpc::pipeline_executor::ServicePipelineExecutionContext;
use crate::pipelines::keys::context_keys::EVENT_LOG_KEY;
use crate::pipelines::pipeline_parts::PipelineParts;
use crate::utils::user_data::user_data::UserData;
use bxes::models::domain::bxes_value::BxesValue;
use bxes_kafka::consumer::bxes_kafka_consumer::{BxesKafkaConsumer, BxesKafkaTrace};
use rdkafka::ClientConfig;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub struct KafkaService {
    names_to_logs: Arc<Mutex<HashMap<String, XesEventLogImpl>>>,
    pipeline_parts: Arc<Box<PipelineParts>>,
    consumers_states: Arc<Mutex<HashMap<Uuid, ConsumerState>>>,
}

enum ConsumerState {
    Consuming,
    ShutdownRequested,
}

const CASE_NAME: &'static str = "case_name";

#[tonic::async_trait]
impl GrpcKafkaService for KafkaService {
    async fn subscribe_for_kafka_topic(
        &self,
        request: Request<GrpcSubscribeForKafkaTopicRequest>,
    ) -> Result<Response<GrpcKafkaResult>, Status> {
        let consumer_uuid = Uuid::new_v4();
        let consumer_states = self.consumers_states.clone();
        let names_to_logs = self.names_to_logs.clone();
        let pipeline_parts = self.pipeline_parts.clone();

        Self::spawn_consumer(request, consumer_uuid, consumer_states, names_to_logs, pipeline_parts);

        Ok(Response::new(GrpcKafkaResult {
            result: Some(grpc_kafka_result::Result::Success(GrpcKafkaSuccessResult {
                subscription_id: Some(GrpcGuid {
                    guid: consumer_uuid.to_string(),
                }),
            })),
        }))
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
}

#[derive(Debug)]
enum XesFromBxesKafkaTraceCreatingError {
    CaseNameNotFound,
    CaseNameNotString,
    BxesToXexConversionError(BxesToXesReadError),
}

impl KafkaService {
    fn spawn_consumer(
        request: Request<GrpcSubscribeForKafkaTopicRequest>,
        consumer_uuid: Uuid,
        consumer_states: Arc<Mutex<HashMap<Uuid, ConsumerState>>>,
        names_to_logs: Arc<Mutex<HashMap<String, XesEventLogImpl>>>,
        pipeline_parts: Arc<Box<PipelineParts>>,
    ) {
        tokio::spawn(async move {
            let mut consumer = Self::create_consumer(&request);
            Self::subscribe(&mut consumer, consumer_uuid, consumer_states.clone());

            loop {
                let should_stop = Self::execute_consumer_routine(
                    &mut consumer,
                    &request,
                    consumer_uuid,
                    consumer_states.clone(),
                    names_to_logs.clone(),
                    pipeline_parts.clone(),
                );

                if should_stop {
                    return;
                }
            }
        });
    }

    fn create_consumer(request: &Request<GrpcSubscribeForKafkaTopicRequest>) -> BxesKafkaConsumer {
        let mut config = ClientConfig::new();

        for metadata_pair in &request.get_ref().metadata {
            config.set(metadata_pair.key.to_owned(), metadata_pair.value.to_owned());
        }

        let consumer = config.create().expect("Should create client config");

        BxesKafkaConsumer::new(request.get_ref().topic_name.to_owned(), consumer)
    }

    fn subscribe(consumer: &mut BxesKafkaConsumer, consumer_uuid: Uuid, consumer_states: Arc<Mutex<HashMap<Uuid, ConsumerState>>>) {
        match consumer.subscribe() {
            Ok(_) => {
                let mut states = consumer_states.lock().expect("Should take lock");
                if states.contains_key(&consumer_uuid) {
                    if let Some(ConsumerState::ShutdownRequested) = states.get(&consumer_uuid) {
                        consumer.unsubscribe();
                        return;
                    } else {
                        println!("Invalid state: consumer already in subscribed state");
                    }
                }

                states.insert(consumer_uuid.clone(), ConsumerState::Consuming);
            }
            Err(err) => println!("Failed to subscribe to topic: {:?}", err),
        };
    }

    fn execute_consumer_routine(
        consumer: &mut BxesKafkaConsumer,
        request: &Request<GrpcSubscribeForKafkaTopicRequest>,
        consumer_uuid: Uuid,
        consumer_states: Arc<Mutex<HashMap<Uuid, ConsumerState>>>,
        names_to_logs: Arc<Mutex<HashMap<String, XesEventLogImpl>>>,
        pipeline_parts: Arc<Box<PipelineParts>>,
    ) -> bool {
        if Self::unsubscribe_if_requested(consumer, consumer_states.clone(), consumer_uuid) {
            return true;
        }

        match consumer.consume() {
            Ok(trace) => match trace {
                Some(trace) => Self::process_kafka_trace(trace, &request, names_to_logs.clone(), pipeline_parts.clone()),
                None => {}
            },
            Err(err) => {
                print!("Failed to read messages from kafka: {:?}", err)
            }
        };

        false
    }

    fn unsubscribe_if_requested(
        consumer: &mut BxesKafkaConsumer,
        consumer_states: Arc<Mutex<HashMap<Uuid, ConsumerState>>>,
        consumer_uuid: Uuid,
    ) -> bool {
        let states = consumer_states.lock().expect("Should take lock");
        if let Some(ConsumerState::ShutdownRequested) = states.get(&consumer_uuid) {
            consumer.unsubscribe();
            true
        } else {
            false
        }
    }

    fn process_kafka_trace(
        trace: BxesKafkaTrace,
        request: &Request<GrpcSubscribeForKafkaTopicRequest>,
        names_to_logs: Arc<Mutex<HashMap<String, XesEventLogImpl>>>,
        pipeline_parts: Arc<Box<PipelineParts>>,
    ) {
        let pipeline_req = request.get_ref().pipeline_request.as_ref().expect("Pipeline should be supplied");
        let grpc_pipeline = pipeline_req.pipeline.as_ref().expect("Pipeline should be supplied");
        let events_handler = Arc::new(Box::new(KafkaEventsHandler::new()) as Box<dyn PipelineEventsHandler>);

        let context = ServicePipelineExecutionContext::new(
            grpc_pipeline,
            &pipeline_req.initial_context,
            pipeline_parts.clone(),
            events_handler.clone(),
        );

        let xes_log = match Self::update_log(names_to_logs.clone(), trace) {
            Ok(xes_log) => xes_log,
            Err(_) => return (),
        };

        let execution_result = context.execute_grpc_pipeline(move |context| {
            context.put_concrete(EVENT_LOG_KEY.key(), xes_log);
        });

        if let Err(err) = execution_result {
            let err = PipelineFinalResult::Error(err.to_string());
            events_handler.handle(PipelineEvent::FinalResult(err));
        }
    }

    fn update_log(
        names_to_logs: Arc<Mutex<HashMap<String, XesEventLogImpl>>>,
        trace: BxesKafkaTrace,
    ) -> Result<XesEventLogImpl, XesFromBxesKafkaTraceCreatingError> {
        let metadata = trace.metadata();
        let mut names_to_logs = names_to_logs.lock();
        let names_to_logs = match names_to_logs.as_mut() {
            Ok(names_to_logs) => names_to_logs,
            Err(_) => panic!("Failed to acquire a names_to_logs map from mutex"),
        };

        if let Some(case_name) = metadata.get(CASE_NAME) {
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

                Ok(existing_log.clone())
            } else {
                Err(XesFromBxesKafkaTraceCreatingError::CaseNameNotString)
            }
        } else {
            Err(XesFromBxesKafkaTraceCreatingError::CaseNameNotFound)
        }
    }
}
