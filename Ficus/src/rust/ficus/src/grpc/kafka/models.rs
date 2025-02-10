use crate::event_log::bxes::bxes_to_xes_converter::BxesToXesReadError;
use crate::grpc::events::events_handler::PipelineEventsHandler;
use crate::grpc::kafka::kafka_service::KafkaSubscription;
use crate::grpc::logs_handler::ConsoleLogMessageHandler;
use crate::pipelines::errors::pipeline_errors::PipelinePartExecutionError;
use crate::pipelines::pipeline_parts::PipelineParts;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub(super) const KAFKA_CASE_DISPLAY_NAME: &'static str = "case_display_name";
pub(super) const KAFKA_CASE_NAME_PARTS: &'static str = "case_name_parts";
pub(super) const KAFKA_CASE_ID: &'static str = "case_id";
pub(super) const KAFKA_CASE_NAME_PARTS_SEPARATOR: &'static str = ";";
pub(super) const KAFKA_PROCESS_NAME: &'static str = "process_name";
pub(super) const KAFKA_TRACE_ID: &'static str = "trace_id";

#[derive(Debug)]
pub enum KafkaTraceProcessingError {
    XesFromBxesTraceCreationError(XesFromBxesKafkaTraceCreatingError),
    FailedToPreprocessTrace(PipelinePartExecutionError),
}

impl Display for KafkaTraceProcessingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            KafkaTraceProcessingError::XesFromBxesTraceCreationError(e) => Display::fmt(e, f),
            KafkaTraceProcessingError::FailedToPreprocessTrace(e) => Display::fmt(e, f),
        }
    }
}

#[derive(Debug)]
pub enum XesFromBxesKafkaTraceCreatingError {
    MetadataValueIsNotAString(String),
    MetadataValueNotFound(String),
    BxesToXexConversionError(BxesToXesReadError),
    TraceIdIsNotUuid,
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
            XesFromBxesKafkaTraceCreatingError::TraceIdIsNotUuid => "Trace id was not of type uuid ".to_string(),
        };

        write!(f, "{}", str)
    }
}

#[derive(Clone)]
pub struct PipelineExecutionDto {
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
pub struct KafkaConsumerCreationDto {
    pub uuid: Uuid,
    pub name: String,
    pub subscriptions_to_execution_requests: Arc<Mutex<HashMap<Uuid, KafkaSubscription>>>,
    pub logger: ConsoleLogMessageHandler,
}

impl KafkaConsumerCreationDto {
    pub fn new(name: String, subscriptions_to_execution_requests: Arc<Mutex<HashMap<Uuid, KafkaSubscription>>>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name,
            subscriptions_to_execution_requests,
            logger: ConsoleLogMessageHandler::new(),
        }
    }
}
