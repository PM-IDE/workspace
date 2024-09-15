use super::events_handler::{PipelineEvent, PipelineEventsHandler};
use crate::ficus_proto::{GrpcGuid, GrpcKafkaConnectionMetadata, GrpcKafkaUpdate};
use crate::grpc::events::utils::create_grpc_context_values;
use crate::grpc::logs_handler::ConsoleLogMessageHandler;
use crate::pipelines::context::LogMessageHandler;
use prost::Message;
use rdkafka::error::KafkaError;
use rdkafka::producer::{BaseProducer, BaseRecord};
use rdkafka::ClientConfig;
use uuid::Uuid;

pub struct PipelineEventsProducer {
    topic_name: String,
    producer: BaseProducer
}

impl PipelineEventsProducer {
    pub fn create(connection_metadata: &GrpcKafkaConnectionMetadata) -> Result<Self, KafkaError> {
        let mut config = ClientConfig::new();

        for kv_pair in &connection_metadata.metadata {
            config.set(kv_pair.key.clone(), kv_pair.value.clone());
        }

        let producer = match config.create() {
            Ok(producer) => producer,
            Err(err) => return Err(err)
        };

        Ok(Self {
            topic_name: connection_metadata.topic_name.to_owned(),
            producer
        })
    }

    pub fn produce(&self, message: GrpcKafkaUpdate) -> Result<(), KafkaError> {
        let encoded_message = message.encode_to_vec();
        let message_id = Uuid::new_v4();
        let record: BaseRecord<[u8], Vec<u8>> = BaseRecord::to(self.topic_name.as_str())
            .key(message_id.as_bytes().as_slice())
            .payload(&encoded_message);

        match self.producer.send(record) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.0)
        }
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
                let result = self.producer.produce(GrpcKafkaUpdate {
                    case_name: event.case_name,
                    pipeline_part_guid: Some(GrpcGuid {
                        guid: event.uuid.to_string()
                    }),
                    context_values: create_grpc_context_values(&event.key_values),
                });

                if result.is_err() {
                    let message = format!("Failed to produce event: {}", result.err().unwrap().to_string());
                    self.console_logs_handler.handle(message.as_str()).expect("Should log message");
                }
            },
            PipelineEvent::LogMessage(_) => {}
            PipelineEvent::FinalResult(_) => {}
        };
    }

    fn is_alive(&self) -> bool {
        true
    }
}
