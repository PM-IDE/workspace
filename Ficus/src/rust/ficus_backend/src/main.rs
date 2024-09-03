use ficus::{ficus_proto::grpc_backend_service_server::GrpcBackendServiceServer, grpc::backend_service::FicusService};
use ficus::ficus_proto::grpc_kafka_service_server::GrpcKafkaServiceServer;
use ficus::grpc::kafka_service::KafkaService;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let backend_service = GrpcBackendServiceServer::new(FicusService::new());
    let kafka_service = GrpcKafkaServiceServer::new(KafkaService::new());

    Server::builder()
        .add_service(backend_service)
        .add_service(kafka_service)
        .serve("[::]:8080".parse()?)
        .await?;

    Ok(())
}
