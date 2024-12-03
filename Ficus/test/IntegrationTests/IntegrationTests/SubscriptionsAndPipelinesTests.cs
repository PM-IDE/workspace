using Ficus;
using Google.Protobuf.WellKnownTypes;
using NUnit.Framework.Internal;

namespace IntegrationTests;

[TestFixture]
public class SubscriptionsAndPipelinesTests : TestWithFicusBackendBase
{
  private const string TestSubscriptionName = nameof(TestSubscriptionName);
  
  [Test]
  public void TestAddRemoveSubscriptions()
  {
    var subscriptionId = CreateKafkaSubscription();

    AssertSingleSubscription(subscriptionId, TestSubscriptionName);

    UnsubscribeFromKafka(subscriptionId);

    AssertNoSubscriptions();
  }

  private void AssertSingleSubscription(GrpcGuid subscriptionId, string subscriptionName)
  {
    var allSubscriptions = KafkaClient.GetAllSubscriptionsAndPipelines(new Empty());

    Assert.Multiple(() =>
    {
      Assert.That(allSubscriptions.Subscriptions, Has.Count.EqualTo(1));
      Assert.That(
        allSubscriptions.Subscriptions.FirstOrDefault(s => s.Metadata.SubscriptionName == subscriptionName && 
                                                           s.Id.Equals(subscriptionId)),
        Is.Not.Null
      );
    });
  }

  private void AssertNoSubscriptions()
  {    
    var allSubscriptions = KafkaClient.GetAllSubscriptionsAndPipelines(new Empty());
    Assert.That(allSubscriptions.Subscriptions, Has.Count.Zero);
  }

  private GrpcGuid CreateKafkaSubscription()
  {
    var result = KafkaClient.SubscribeForKafkaTopic(GrpcRequestsCreator.CreateSubscribeToKafkaRequest(TestsSettings, TestSubscriptionName));

    Assert.That(result.ResultCase, Is.EqualTo(GrpcKafkaResult.ResultOneofCase.Success));

    return result.Success.Id;
  }

  private void UnsubscribeFromKafka(GrpcGuid subscriptionId)
  {
    var unsubscribeResult = KafkaClient.UnsubscribeFromKafkaTopic(new GrpcUnsubscribeFromKafkaRequest
    {
      SubscriptionId = subscriptionId
    });

    Assert.That(unsubscribeResult.ResultCase, Is.EqualTo(GrpcKafkaResult.ResultOneofCase.Success));
  }
}