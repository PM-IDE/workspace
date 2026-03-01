use super::events_handler::{GetContextValuesEvent, PipelineEvent, PipelineEventsHandler, PipelineFinalResult};
use crate::{
  ficus_proto::{
    GrpcCaseName, GrpcGuid, GrpcKafkaConnectionMetadata, GrpcKafkaUpdate, GrpcPipelinePartInfo, GrpcProcessCaseMetadata, GrpcStringKeyValue,
  },
  grpc::{events::utils::create_grpc_context_values, logs_handler::ConsoleLogMessageHandler},
};
use ficus::{features::cases::CaseName, pipelines::context::LogMessageHandler};
use prost::Message;
use rdkafka::{
  error::KafkaError,
  producer::{BaseProducer, BaseRecord},
  util::Timeout,
  ClientConfig,
};
use std::rc::Rc;
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

    let producer = config.create()?;

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
  fn handle(&self, event: &PipelineEvent) {
    match event {
      PipelineEvent::GetContextValuesEvent(event) => {
        let result = self.producer.produce(event.to_grpc_kafka_update());

        let message = match result {
          Ok(_) => "Sent message to kafka".to_string(),
          Err(err) => format!("Failed to produce event: {}", err),
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
  fn to_grpc_kafka_update(&self) -> GrpcKafkaUpdate {
    GrpcKafkaUpdate {
      process_case_metadata: Some(self.process_case_metadata.to_grpc_process_case_metadata()),
      pipeline_part_info: Some(GrpcPipelinePartInfo {
        id: Some(GrpcGuid {
          guid: self.pipeline_part_id.to_string(),
        }),
        execution_id: Some(GrpcGuid {
          guid: self.execution_id.to_string(),
        }),
        name: self.pipeline_part_name.clone(),
      }),
      context_values: create_grpc_context_values(&self.key_values),
    }
  }
}

pub struct ProcessCaseMetadata {
  pub case_name: CaseName,
  pub process_name: Rc<str>,
  pub subscription_id: Option<Uuid>,
  pub subscription_name: Option<Rc<str>>,
  pub pipeline_id: Option<Uuid>,
  pub pipeline_name: Option<Rc<str>>,
  pub metadata: Vec<(Rc<str>, Rc<str>)>,
}

impl ProcessCaseMetadata {
  fn to_grpc_process_case_metadata(&self) -> GrpcProcessCaseMetadata {
    GrpcProcessCaseMetadata {
      case_name: Some(GrpcCaseName {
        display_name: self.case_name.display_name.to_string(),
        full_name_parts: self.case_name.name_parts.iter().map(|p| p.to_string()).collect(),
      }),
      process_name: self.process_name.to_string(),

      subscription_id: self.subscription_id.map(GrpcGuid::from),
      subscription_name: self.subscription_name.clone().map_or("".to_string(), |name| name.to_string()),
      pipeline_id: self.pipeline_id.map(GrpcGuid::from),
      pipeline_name: self.pipeline_name.clone().map_or("".to_string(), |name| name.to_string()),

      metadata: self
        .metadata
        .iter()
        .map(|pair| GrpcStringKeyValue {
          key: pair.0.to_string(),
          value: pair.1.to_string(),
        })
        .collect(),
    }
  }
}
