use ficus::{ficus_proto::grpc_backend_service_server::GrpcBackendServiceServer, grpc::backend_service::FicusService};

use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ficus_service = FicusService::new();
    let service = GrpcBackendServiceServer::new(ficus_service);
    Server::builder().add_service(service).serve("[::]:8080".parse()?).await?;

    Ok(())
}
