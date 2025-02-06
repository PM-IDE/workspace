using Autofac;
using Core.Container;
using Core.Events.EventRecord;
using ProcfilerOnline.Core.Features;
using ProcfilerOnline.Integrations.Kafka.Bxes;
using ProcfilerOnline.Integrations.Kafka.Json;

namespace ProcfilerOnline.Core.Handlers;

public class CompletedAsyncMethodEvent : IEventPipeStreamEvent
{
  public required string ApplicationName { get; init; }
  public required string StateMachineName { get; init; }
  public required Guid AsyncMethodCaseId { get; init; }
  public required List<List<EventRecordWithMetadata>> MethodTraces { get; init; }
  public ExtendedMethodInfo? MethodInfo { get; init; }
}

[AppComponent]
public class CompletedAsyncMethodHandler(
  IComponentContext container
) : IEventPipeStreamEventHandler
{
  public void Handle(IEventPipeStreamEvent eventPipeStreamEvent)
  {
    if (!ProcfilerOnlineFeatures.ProduceEventsToKafka.IsEnabled()) return;
    if (eventPipeStreamEvent is not CompletedAsyncMethodEvent completedAsyncMethodEvent) return;

    if (ProcfilerOnlineFeatures.ProduceBxesKafkaEvents.IsEnabled())
    {
      ProduceBxesKafkaMessage(completedAsyncMethodEvent);
      return;
    }

    ProduceJsonKafkaMessage(completedAsyncMethodEvent);
  }

  private void ProduceBxesKafkaMessage(CompletedAsyncMethodEvent completedAsyncMethodEvent)
  {
    var producer = container.Resolve<IBxesMethodsKafkaProducer>();

    foreach (var methodTrace in completedAsyncMethodEvent.MethodTraces)
    {
      var message = new BxesKafkaTrace
      {
        ProcessName = completedAsyncMethodEvent.ApplicationName,
        CaseName = new BxesKafkaCaseName
        {
          DisplayName = completedAsyncMethodEvent.StateMachineName,
          NameParts = [completedAsyncMethodEvent.StateMachineName]
        },
        Trace = methodTrace,
        CaseId = completedAsyncMethodEvent.AsyncMethodCaseId
      };

      completedAsyncMethodEvent.MethodInfo.AddToMetadata(message.Metadata);

      producer.Produce(Guid.NewGuid(), message);
    }
  }

  private void ProduceJsonKafkaMessage(CompletedAsyncMethodEvent completedAsyncMethodEvent)
  {
    var producer = container.Resolve<IJsonMethodsKafkaProducer>();
    foreach (var methodTrace in completedAsyncMethodEvent.MethodTraces)
    {
      var message = new JsonMethodsExecutionKafkaMessage
      {
        MethodFullName = completedAsyncMethodEvent.StateMachineName,
        Events = methodTrace.Select(JsonEventRecordWithMetadataKafkaDto.FromEventRecord).ToList()
      };

      producer.Produce(Guid.NewGuid(), message);
    }
  }
}