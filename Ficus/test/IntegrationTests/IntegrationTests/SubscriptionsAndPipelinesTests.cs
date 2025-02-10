using Ficus;
using Google.Protobuf.WellKnownTypes;
using IntegrationTests.Base;

namespace IntegrationTests;

[TestFixture, FixtureLifeCycle(LifeCycle.SingleInstance)]
public class SubscriptionsAndPipelinesTests : TestWithFicusBackendBase
{
  private const string TestSubscriptionName = nameof(TestSubscriptionName);
  private const string TestPipelineName = nameof(TestPipelineName);

  [Test]
  public void TestAddRemoveSubscriptions() => ExecuteTestWithSingleSubscription(_ => { });

  [Test]
  public void TestAddRemovePipelinesInSubscription() => ExecuteTestWithSingleSubscription(subscription =>
  {
    var pipelineId = AddPipelineToSubscription(subscription.Id);
    AssertSinglePipeline(subscription.Id, pipelineId, TestPipelineName);
    RemovePipelineFromSubscription(subscription.Id, pipelineId);
    AssertNoPipelinesInSubscription(subscription.Id);
  });

  private GrpcGuid AddPipelineToSubscription(GrpcGuid subscriptionId)
  {
    var pipelineRequest = GrpcRequestsCreator.CreateAddGetNamesLogPipelineRequest(subscriptionId, TestsSettings, TestPipelineName);

    var pipelineAdditionResult = KafkaClient.AddPipelineToSubscription(pipelineRequest);
    Assert.That(pipelineAdditionResult.ResultCase, Is.EqualTo(GrpcKafkaResult.ResultOneofCase.Success));

    return pipelineAdditionResult.Success.Id;
  }

  private void RemovePipelineFromSubscription(GrpcGuid subscriptionId, GrpcGuid pipelineId)
  {
    KafkaClient.RemovePipelineSubscription(new GrpcRemovePipelineRequest
    {
      SubscriptionId = subscriptionId,
      PipelineId = pipelineId
    });
  }

  private void AssertNoPipelinesInSubscription(GrpcGuid subscriptionId)
  {
    var subscription = FindSubscription(subscriptionId);
    Assert.That(subscription.Pipelines, Has.Count.Zero);
  }

  private void ExecuteTestWithSingleSubscription(Action<GrpcKafkaSubscription> testAction)
  {
    var subscriptionId = CreateKafkaSubscription();

    try
    {
      AssertSingleSubscription(subscriptionId, TestSubscriptionName);
      var subscription = FindSubscription(subscriptionId);

      testAction(subscription);
    }
    finally
    {
      UnsubscribeFromKafka(subscriptionId);

      AssertNoSubscriptions();
    }
  }

  private void AssertSinglePipeline(GrpcGuid subscriptionId, GrpcGuid pipelineId, string name)
  {
    var subscription = FindSubscription(subscriptionId);
    var pipeline = subscription.Pipelines.FirstOrDefault(p => p.Id.Guid == pipelineId.Guid);

    Assert.That(pipeline, Is.Not.Null);
    Assert.That(pipeline.Metadata.Name, Is.EqualTo(name));
  }

  private GrpcKafkaSubscription FindSubscription(GrpcGuid subscriptionId)
  {
    var allSubscriptions = KafkaClient.GetAllSubscriptionsAndPipelines(new Empty());
    var subscription = allSubscriptions.Subscriptions.FirstOrDefault(s => s.Id.Equals(subscriptionId));
    Assert.That(subscription, Is.Not.Null);

    return subscription;
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
    var request = GrpcRequestsCreator.CreateSubscribeToKafkaRequest(TestsSettings, TestSubscriptionName);
    var result = KafkaClient.SubscribeForKafkaTopic(request);

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