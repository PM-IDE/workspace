use ficus::ficus_proto::grpc_context_values_service_server::GrpcContextValuesServiceServer;
use ficus::ficus_proto::grpc_kafka_service_server::GrpcKafkaServiceServer;
use ficus::grpc::context_values_service::{ContextValueService, GrpcContextValueService};
use ficus::grpc::kafka::grpc_kafka_service::GrpcKafkaServiceImpl;
use ficus::{
    ficus_proto::grpc_backend_service_server::GrpcBackendServiceServer,
    grpc::backend_service::FicusService,
};
use std::sync::{Arc, Mutex};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cv_service = Arc::new(Mutex::new(ContextValueService::new()));
    let grpc_cv_service =
        GrpcContextValuesServiceServer::new(GrpcContextValueService::new(cv_service.clone()));
    let backend_service = GrpcBackendServiceServer::new(FicusService::new(cv_service.clone()));
    let kafka_service = GrpcKafkaServiceServer::new(GrpcKafkaServiceImpl::new(cv_service.clone()));

    Server::builder()
        .add_service(grpc_cv_service)
        .add_service(backend_service)
        .add_service(kafka_service)
        .serve("[::]:8080".parse()?)
        .await?;

    Ok(())
}
