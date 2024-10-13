use super::events_handler::{GetContextValuesEvent, PipelineEvent, PipelineEventsHandler, PipelineFinalResult, ProcessCaseMetadata};
use crate::ficus_proto::{
    GrpcGuid, GrpcKafkaConnectionMetadata, GrpcKafkaUpdate, GrpcPipelinePartInfo, GrpcProcessCaseMetadata, GrpcStringKeyValue,
};
use crate::grpc::events::utils::create_grpc_context_values;
use crate::grpc::logs_handler::ConsoleLogMessageHandler;
use crate::pipelines::context::LogMessageHandler;
use prost::Message;
use rdkafka::error::KafkaError;
use rdkafka::producer::{BaseProducer, BaseRecord};
use rdkafka::util::Timeout;
use rdkafka::ClientConfig;
use uuid::Uuid;

pub struct PipelineEventsProducer {
    topic_name: String,
    producer: BaseProducer,
}

impl PipelineEventsProducer {
    pub fn create(connection_metadata: &GrpcKafkaConnectionMetadata) -> Result<Self, KafkaError> {
        let mut config = ClientConfig::new();

        for kv_pair in &connection_metadata.metadata {
            config.set(kv_pair.key.clone(), kv_pair.value.clone());
        }

        let producer = match config.create() {
            Ok(producer) => producer,
            Err(err) => return Err(err),
        };

        Ok(Self {
            topic_name: connection_metadata.topic_name.to_owned(),
            producer,
        })
    }

    pub fn produce(&self, message: GrpcKafkaUpdate) -> Result<(), KafkaError> {
        let encoded_message = message.encode_to_vec();
        let message_id = Uuid::new_v4();
        let record: BaseRecord<[u8], Vec<u8>> = BaseRecord::to(self.topic_name.as_str())
            .key(message_id.as_bytes().as_slice())
            .payload(&encoded_message);

        let result = match self.producer.send(record) {
            Ok(_) => {
                self.producer.poll(Timeout::Never);
                Ok(())
            }
            Err(err) => Err(err.0),
        };

        result
    }
}

pub struct KafkaEventsHandler {
    producer: PipelineEventsProducer,
    console_logs_handler: ConsoleLogMessageHandler,
}

impl KafkaEventsHandler {
    pub fn new(producer: PipelineEventsProducer) -> Self {
        Self {
            producer,
            console_logs_handler: ConsoleLogMessageHandler::new(),
        }
    }
}

impl PipelineEventsHandler for KafkaEventsHandler {
    fn handle(&self, event: PipelineEvent) {
        match event {
            PipelineEvent::GetContextValuesEvent(event) => {
                let result = self.producer.produce(event.to_grpc_kafka_update());

                let message = match result {
                    Ok(_) => "Sent message to kafka".to_string(),
                    Err(err) => format!("Failed to produce event: {}", err.to_string()),
                };

                self.console_logs_handler.handle(message.as_str()).expect("Should log message");
            }
            PipelineEvent::LogMessage(_) => {}
            PipelineEvent::FinalResult(result) => match result {
                PipelineFinalResult::Success(_) => {}
                PipelineFinalResult::Error(err) => {
                    let message = format!("Received error as final result: {}", err);
                    self.console_logs_handler.handle(message.as_str()).expect("Should log message");
                }
            },
        };
    }

    fn is_alive(&self) -> bool {
        true
    }
}

impl GetContextValuesEvent<'_> {
    fn to_grpc_kafka_update(self) -> GrpcKafkaUpdate {
        GrpcKafkaUpdate {
            process_case_metadata: Some(self.process_case_metadata.to_grpc_process_case_metadata()),
            pipeline_part_info: Some(GrpcPipelinePartInfo {
                id: Some(GrpcGuid {
                    guid: self.uuid.to_string(),
                }),
                name: self.pipeline_part_name,
            }),
            context_values: create_grpc_context_values(&self.key_values),
        }
    }
}

impl ProcessCaseMetadata {
    fn to_grpc_process_case_metadata(self) -> GrpcProcessCaseMetadata {
        GrpcProcessCaseMetadata {
            case_name: self.case_name,
            process_name: self.process_name,
            metadata: self
                .metadata
                .into_iter()
                .map(|pair| GrpcStringKeyValue {
                    key: pair.0,
                    value: pair.1,
                })
                .collect(),
        }
    }
}
