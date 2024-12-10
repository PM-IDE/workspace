using Ficus;
using FicusKafkaIntegration;
using Grpc.Net.Client;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.Configuration.EnvironmentVariables;

namespace IntegrationTests;

public class FicusKafkaProducerSettings
{
  public required string Topic { get; init; }
  public required string BootstrapServers { get; init; }
}

public class FicusIntegrationTestsSettings
{
  public required string ConsumerBootstrapServers { get; init; }
  public required string ConsumerTopic { get; init; }
  public required string ConsumerGroup { get; init; }
  
  public required string ProducerBootstrapServers { get; init; }
  public required string ProducerTopic { get; init; }

  public required string FicusBackendAddress { get; init; }
}

public abstract class TestWithFicusBackendBase
{
  protected IConfiguration Configuration;
  protected GrpcKafkaService.GrpcKafkaServiceClient KafkaClient;
  protected FicusKafkaProducerSettings ProducerSettings;
  protected PipelinePartsUpdateKafkaSettings PipelinePartsSettings;
  protected FicusIntegrationTestsSettings TestsSettings;


  [SetUp]
  public void InitConfiguration()
  {
    Configuration = new ConfigurationBuilder().Add(new EnvironmentVariablesConfigurationSource()).Build();

    ProducerSettings = Configuration.GetSection(nameof(FicusKafkaProducerSettings)).Get<FicusKafkaProducerSettings>()!;
    PipelinePartsSettings = Configuration.GetSection(nameof(PipelinePartsUpdateKafkaSettings)).Get<PipelinePartsUpdateKafkaSettings>()!;
    TestsSettings = Configuration.GetSection(nameof(FicusIntegrationTestsSettings)).Get<FicusIntegrationTestsSettings>()!;

    var channel = GrpcChannel.ForAddress($"http://{TestsSettings.FicusBackendAddress}");
    KafkaClient = new GrpcKafkaService.GrpcKafkaServiceClient(channel);
  }
}