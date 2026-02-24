fn main() -> Result<(), Box<dyn std::error::Error>> {
  tonic_build::configure()
    .build_server(true)
    .compile(
      &[
        "../../../protos/backend_service.proto",
        "../../../protos/kafka_service.proto",
        "../../../protos/front_contract.proto",
        "../../../protos/context_values_service.proto",
      ],
      &["../../../protos"],
    )
    .ok();

  Ok(())
}
