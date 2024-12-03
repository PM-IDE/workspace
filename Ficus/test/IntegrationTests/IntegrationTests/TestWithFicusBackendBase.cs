using Ficus;
using FicusKafkaIntegration;
using Grpc.Net.Client;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.Configuration.EnvironmentVariables;

namespace IntegrationTests;

public abstract class TestWithFicusBackendBase
{
  protected IConfiguration Configuration;
  protected GrpcKafkaService.GrpcKafkaServiceClient KafkaClient;
  protected FicusKafkaProducerSettings ProducerSettings;
  protected PipelinePartsUpdateKafkaSettings PipelinePartsSettings;


  [SetUp]
  public void InitConfiguration()
  {
    Configuration = new ConfigurationBuilder().Add(new EnvironmentVariablesConfigurationSource()).Build();

    var channel = GrpcChannel.ForAddress(Environment.GetEnvironmentVariable("FicusBackendAddr")!);
    KafkaClient = new GrpcKafkaService.GrpcKafkaServiceClient(channel);

    ProducerSettings = Configuration.GetSection(nameof(FicusKafkaProducerSettings)).Get<FicusKafkaProducerSettings>()!;
    PipelinePartsSettings = Configuration.GetSection(nameof(PipelinePartsUpdateKafkaSettings)).Get<PipelinePartsUpdateKafkaSettings>()!;
  }
}