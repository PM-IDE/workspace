use crate::{
  ficus_proto::{
    grpc_backend_service_server::GrpcBackendServiceServer, grpc_context_values_service_server::GrpcContextValuesServiceServer,
    grpc_kafka_service_server::GrpcKafkaServiceServer,
  },
  grpc::{
    backend_service::FicusService,
    context_values_service::{ContextValueService, GrpcContextValueService},
    kafka::grpc_kafka_service::GrpcKafkaServiceImpl,
  },
};
use log::{LevelFilter, info};
use std::sync::Arc;
use tonic::transport::Server;

pub mod ficus_proto {
  tonic::include_proto!("ficus");
}

mod grpc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  colog::basic_builder().filter_level(LevelFilter::Info).init();

  let cv_service = Arc::new(ContextValueService::new());
  let grpc_cv_service = GrpcContextValuesServiceServer::new(GrpcContextValueService::new(cv_service.clone()));

  let backend_service = GrpcBackendServiceServer::new(FicusService::new(cv_service.clone()));
  let kafka_service = GrpcKafkaServiceServer::new(GrpcKafkaServiceImpl::new(cv_service.clone()));

  info!("Starting server");

  Server::builder()
    .add_service(grpc_cv_service)
    .add_service(backend_service)
    .add_service(kafka_service)
    .serve("[::]:8080".parse()?)
    .await?;

  Ok(())
}
