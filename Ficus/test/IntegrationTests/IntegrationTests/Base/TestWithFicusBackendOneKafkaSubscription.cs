using Ficus;

namespace IntegrationTests.Base;

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