use crate::event_log::bxes::bxes_to_xes_converter::BxesToXesReadError;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::grpc::events::events_handler::{CaseName, PipelineEventsHandler};
use crate::grpc::kafka::kafka_service::KafkaSubscription;
use crate::grpc::logs_handler::ConsoleLogMessageHandler;
use crate::pipelines::pipeline_parts::PipelineParts;
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub(super) const KAFKA_CASE_DISPLAY_NAME: &'static str = "case_display_name";
pub(super) const KAFKA_CASE_NAME_PARTS: &'static str = "case_name_parts";
pub(super) const KAFKA_CASE_NAME_PARTS_SEPARATOR: &'static str = ";";
pub(super) const KAFKA_PROCESS_NAME: &'static str = "process_name";

#[derive(Debug)]
pub(super) enum XesFromBxesKafkaTraceCreatingError {
    MetadataValueIsNotAString(String),
    MetadataValueNotFound(String),
    BxesToXexConversionError(BxesToXesReadError),
}

impl Display for XesFromBxesKafkaTraceCreatingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            XesFromBxesKafkaTraceCreatingError::BxesToXexConversionError(err) => err.to_string(),
            XesFromBxesKafkaTraceCreatingError::MetadataValueIsNotAString(key_name) => {
                format!("Value for key {} is not a String", key_name.to_owned())
            }
            XesFromBxesKafkaTraceCreatingError::MetadataValueNotFound(key_name) => {
                format!("The key {} is not found", key_name.to_string())
            }
        };

        write!(f, "{}", str)
    }
}

#[derive(Clone)]
pub(super) struct PipelineExecutionDto {
    pub(super) pipeline_parts: Arc<Box<PipelineParts>>,
    pub(super) events_handler: Arc<Box<dyn PipelineEventsHandler>>,
}

impl PipelineExecutionDto {
    pub fn new(pipeline_parts: Arc<Box<PipelineParts>>, events_handler: Arc<Box<dyn PipelineEventsHandler>>) -> Self {
        Self {
            pipeline_parts,
            events_handler,
        }
    }
}

#[derive(Clone)]
pub(super) struct KafkaConsumerCreationDto {
    pub uuid: Uuid,
    pub name: String,
    pub names_to_logs: Arc<Mutex<HashMap<String, XesEventLogImpl>>>,
    pub subscriptions_to_execution_requests: Arc<Mutex<HashMap<Uuid, KafkaSubscription>>>,
    pub logger: ConsoleLogMessageHandler,
}

impl KafkaConsumerCreationDto {
    pub fn new(
        name: String,
        names_to_logs: Arc<Mutex<HashMap<String, XesEventLogImpl>>>,
        subscriptions_to_execution_requests: Arc<Mutex<HashMap<Uuid, KafkaSubscription>>>,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name,
            names_to_logs,
            subscriptions_to_execution_requests,
            logger: ConsoleLogMessageHandler::new(),
        }
    }
}

#[derive(Clone)]
pub(super) struct LogUpdateResult {
    pub process_name: String,
    pub case_name: CaseName,
    pub new_log: XesEventLogImpl,
    pub unstructured_metadata: Vec<(String, String)>,
}
