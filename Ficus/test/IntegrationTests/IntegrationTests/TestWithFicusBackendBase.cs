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


  [OneTimeSetUp]
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

public abstract class TestWithFicusBackendOneKafkaSubscription : TestWithFicusBackendBase
{
  private GrpcGuid? mySubscriptionId;


  [OneTimeSetUp]
  public void Setup()
  {
    mySubscriptionId = CreateFicusKafkaSubscription();
  }

  [OneTimeTearDown]
  public void Teardown()
  {
    KafkaClient.UnsubscribeFromKafkaTopic(new GrpcUnsubscribeFromKafkaRequest
    {
      SubscriptionId = mySubscriptionId
    });
  }

  private GrpcGuid CreateFicusKafkaSubscription()
  {
    var subscribeRequest = GrpcRequestsCreator.CreateSubscribeToKafkaRequest(TestsSettings);
    var subscriptionResult = KafkaClient.SubscribeForKafkaTopic(subscribeRequest);

    Assert.That(subscriptionResult.ResultCase, Is.EqualTo(GrpcKafkaResult.ResultOneofCase.Success));

    var subscriptionId = subscriptionResult.Success.Id;
    var addPipelineRequest = GrpcRequestsCreator.CreateAddGetNamesLogPipelineRequest(subscriptionId, TestsSettings);

    var pipelineAdditionResult = KafkaClient.AddPipelineToSubscription(addPipelineRequest);

    Assert.That(pipelineAdditionResult.ResultCase, Is.EqualTo(GrpcKafkaResult.ResultOneofCase.Success));

    return subscriptionResult.Success.Id;
  }
}