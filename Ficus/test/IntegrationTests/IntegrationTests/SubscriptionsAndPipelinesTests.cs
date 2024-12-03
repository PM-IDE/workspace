using Ficus;
using Google.Protobuf.WellKnownTypes;
using Grpc.Net.Client;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.Configuration.EnvironmentVariables;

namespace IntegrationTests;

[TestFixture]
public class SubscriptionsAndPipelinesTests
{
  private IConfiguration myConfiguration;
  private GrpcKafkaService.GrpcKafkaServiceClient myKafkaClient;
  private FicusKafkaProducerSettings myProducerSettings;


  [SetUp]
  public void InitConfiguration()
  {
    myConfiguration = new ConfigurationBuilder().Add(new EnvironmentVariablesConfigurationSource()).Build();
    
    var channel = GrpcChannel.ForAddress(Environment.GetEnvironmentVariable("FicusBackendAddr")!);
    myKafkaClient = new GrpcKafkaService.GrpcKafkaServiceClient(channel);
    myProducerSettings = myConfiguration.GetSection(nameof(FicusKafkaProducerSettings)).Get<FicusKafkaProducerSettings>()!;
  }

  [Test]
  public void TestAddRemoveSubscriptions()
  {
    const string TestSubscriptionName = nameof(TestSubscriptionName);
    var result = myKafkaClient.SubscribeForKafkaTopic(new GrpcSubscribeToKafkaRequest
    {
      ConnectionMetadata = new GrpcKafkaConnectionMetadata
      {
        TopicName = myProducerSettings.Topic,
        Metadata =
        {
          new GrpcKafkaConsumerMetadata
          {
            Key = "bootstrap.servers",
            Value = myProducerSettings.BootstrapServers
          },
          new GrpcKafkaConsumerMetadata
          {
            Key = "group.id",
            Value = "xdxd"
          }
        }
      },
      SubscriptionMetadata = new GrpcKafkaSubscriptionMetadata
      {
        SubscriptionName = TestSubscriptionName
      }
    });

    Assert.That(result.ResultCase, Is.EqualTo(GrpcKafkaResult.ResultOneofCase.Success));

    var allSubscriptions = myKafkaClient.GetAllSubscriptionsAndPipelines(new Empty());

    Assert.Multiple(() =>
    {
      Assert.That(allSubscriptions.Subscriptions, Has.Count.EqualTo(2));
      Assert.That(
        allSubscriptions.Subscriptions.FirstOrDefault(s => s.Metadata.SubscriptionName == TestSubscriptionName),
        Is.Not.Null
      );
    });

    var unsubscribeResult = myKafkaClient.UnsubscribeFromKafkaTopic(new GrpcUnsubscribeFromKafkaRequest
    {
      SubscriptionId = result.Success.Id
    });

    Assert.That(unsubscribeResult.ResultCase, Is.EqualTo(GrpcKafkaResult.ResultOneofCase.Success));

    var allSubscriptionsAfterUnsubscribe = myKafkaClient.GetAllSubscriptionsAndPipelines(new Empty());

    Assert.Multiple(() =>
    {
      Assert.That(allSubscriptionsAfterUnsubscribe.Subscriptions, Has.Count.EqualTo(1));
      Assert.That(
        allSubscriptionsAfterUnsubscribe.Subscriptions.FirstOrDefault(s => s.Metadata.SubscriptionName == TestSubscriptionName),
        Is.Null
      );
    });
  }
}