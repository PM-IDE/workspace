use crate::event_log::bxes::bxes_to_xes_converter::{read_bxes_events, BxesToXesReadError};
use crate::event_log::core::event_log::EventLog;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::ficus_proto::grpc_kafka_service_server::GrpcKafkaService;
use crate::ficus_proto::{GrpcKafkaResult, GrpcSubscribeForKafkaTopicRequest, GrpcUnsubscribeFromKafkaRequest};
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
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};

use super::events::events_handler::{PipelineEventsHandler, PipelineFinalResult};

pub struct KafkaService {
    names_to_logs: Arc<Mutex<HashMap<String, Arc<Mutex<XesEventLogImpl>>>>>,
    pipeline_parts: Arc<Box<PipelineParts>>,
}

const CASE_NAME: &'static str = "case_name";

#[tonic::async_trait]
impl GrpcKafkaService for KafkaService {
    async fn subscribe_for_kafka_topic(
        &self,
        request: Request<GrpcSubscribeForKafkaTopicRequest>,
    ) -> Result<Response<GrpcKafkaResult>, Status> {
        let names_to_logs = self.names_to_logs.clone();
        let pipeline_parts = self.pipeline_parts.clone();

        tokio::task::spawn(async move {
            let request = request.get_ref();

            let mut config = ClientConfig::new();

            for metadata_pair in &request.metadata {
                config.set(metadata_pair.key.to_owned(), metadata_pair.value.to_owned());
            }

            let consumer = config.create().expect("Should create client config");
            let pipeline_req = request.pipeline_request.as_ref().expect("Pipeline should be supplied");
            let mut consumer = BxesKafkaConsumer::new(request.topic_name.to_owned(), consumer);

            let result = consumer.consume(|trace| {
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
                    events_handler.handle(PipelineEvent::FinalResult(PipelineFinalResult::Error(err.to_string())));
                }
            });

            if let Err(kafka_error) = result {
                print!("Failed to read messsages from kafka: {:?}", kafka_error)
            }
        });

        todo!();
    }

    async fn unsubscribe_from_kafka_topic(
        &self,
        request: Request<GrpcUnsubscribeFromKafkaRequest>,
    ) -> Result<Response<GrpcKafkaResult>, Status> {
        todo!()
    }
}

#[derive(Debug)]
enum XesFromBxesKafkaTraceCreatingError {
    CaseNameNotFound,
    CaseNameNotString,
    BxesToXexConversionError(BxesToXesReadError),
}

impl KafkaService {
    fn update_log(
        names_to_logs: Arc<Mutex<HashMap<String, Arc<Mutex<XesEventLogImpl>>>>>,
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
                    let new_log = Arc::new(Mutex::new(XesEventLogImpl::empty()));
                    names_to_logs.insert(case_name.to_owned(), new_log);
                }

                let mut existing_log = names_to_logs.get(case_name).expect("Log should be present").lock();

                let existing_log = existing_log.as_mut().ok().expect("Should take the lock on the log");

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
