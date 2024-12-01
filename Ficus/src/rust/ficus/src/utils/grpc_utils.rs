use crate::ficus_proto::{grpc_kafka_result, GrpcGuid, GrpcKafkaResult, GrpcKafkaSuccessResult};
use std::str::FromStr;
use tonic::Status;
use uuid::Uuid;

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
