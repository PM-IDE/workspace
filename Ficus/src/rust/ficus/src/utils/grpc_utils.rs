use crate::ficus_proto::{grpc_kafka_result, GrpcAddPipelineRequest, GrpcAddPipelineStreamRequest, GrpcGuid, GrpcKafkaPipelineExecutionRequest, GrpcKafkaResult, GrpcKafkaSuccessResult, GrpcPipelineExecutionRequest, GrpcPipelineStreamingConfiguration};
use std::str::FromStr;
use tonic::Status;
use uuid::Uuid;
use crate::grpc::kafka::streaming_configs::StreamingConfiguration;

impl GrpcGuid {
    pub fn to_uuid(&self) -> Result<Uuid, Status> {
        match Uuid::from_str(&self.guid) {
            Ok(uuid) => Ok(uuid),
            Err(_) => Err(Status::invalid_argument("Invalid uuid")),
        }
    }
}

impl From<Uuid> for GrpcGuid {
    fn from(value: Uuid) -> Self {
        GrpcGuid { guid: value.to_string() }
    }
}

impl GrpcKafkaResult {
    pub fn success(uuid: Uuid) -> GrpcKafkaResult {
        GrpcKafkaResult {
            result: Some(grpc_kafka_result::Result::Success(GrpcKafkaSuccessResult {
                id: Some(GrpcGuid::from(uuid)),
            })),
        }
    }
}

pub struct PipelineExecutionDto {
    pub name: String,
    pub request: GrpcPipelineExecutionRequest,
    pub streaming_configuration: GrpcPipelineStreamingConfiguration,
    pub subscription_id: Uuid,
}

impl GrpcAddPipelineStreamRequest {
    pub fn to_dto(&self) -> PipelineExecutionDto {
        self.pipeline_request.as_ref().unwrap().to_dto()
    }
}

impl GrpcAddPipelineRequest {
    pub fn to_dto(&self) -> PipelineExecutionDto {
        self.pipeline_request.as_ref().unwrap().to_dto()
    }
}

impl GrpcKafkaPipelineExecutionRequest {
    pub fn to_dto(&self) -> PipelineExecutionDto {
        PipelineExecutionDto {
            name: self.pipeline_metadata.as_ref().unwrap().name.clone(),
            request: self.pipeline_request.as_ref().unwrap().clone(),
            subscription_id: self.subscription_id.as_ref().unwrap().to_uuid().unwrap(),
            streaming_configuration: self.streaming_configuration.as_ref().unwrap().clone(),
        }
    }
}
