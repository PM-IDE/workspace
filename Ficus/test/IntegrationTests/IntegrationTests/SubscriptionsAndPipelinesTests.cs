using Ficus;
using Google.Protobuf.WellKnownTypes;

namespace IntegrationTests;

[TestFixture]
public class SubscriptionsAndPipelinesTests : TestWithFicusBackendBase
{
  [Test]
  public void TestAddRemoveSubscriptions()
  {
    const string TestSubscriptionName = nameof(TestSubscriptionName);
    var result = KafkaClient.SubscribeForKafkaTopic(new GrpcSubscribeToKafkaRequest
    {
      ConnectionMetadata = new GrpcKafkaConnectionMetadata
      {
        TopicName = ProducerSettings.Topic,
        Metadata =
        {
          new GrpcKafkaMetadata
          {
            Key = "bootstrap.servers",
            Value = ProducerSettings.BootstrapServers
          },
          new GrpcKafkaMetadata
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

    var allSubscriptions = KafkaClient.GetAllSubscriptionsAndPipelines(new Empty());

    Assert.Multiple(() =>
    {
      Assert.That(allSubscriptions.Subscriptions, Has.Count.EqualTo(1));
      Assert.That(
        allSubscriptions.Subscriptions.FirstOrDefault(s => s.Metadata.SubscriptionName == TestSubscriptionName),
        Is.Not.Null
      );
    });

    var unsubscribeResult = KafkaClient.UnsubscribeFromKafkaTopic(new GrpcUnsubscribeFromKafkaRequest
    {
      SubscriptionId = result.Success.Id
    });

    Assert.That(unsubscribeResult.ResultCase, Is.EqualTo(GrpcKafkaResult.ResultOneofCase.Success));

    var allSubscriptionsAfterUnsubscribe = KafkaClient.GetAllSubscriptionsAndPipelines(new Empty());

    Assert.Multiple(() =>
    {
      Assert.That(allSubscriptionsAfterUnsubscribe.Subscriptions, Has.Count.Zero);
      Assert.That(
        allSubscriptionsAfterUnsubscribe.Subscriptions.FirstOrDefault(s => s.Metadata.SubscriptionName == TestSubscriptionName),
        Is.Null
      );
    });
  }
}