using Core.Container;
using Core.Events.EventRecord;
using Microsoft.Extensions.Options;
using ProcfilerOnline.Core.Features;
using ProcfilerOnline.Core.Settings;
using ProcfilerOnline.Integrations.Kafka;
using ProcfilerOnline.Integrations.Kafka.Json;

namespace ProcfilerOnline.Core.Handlers;

public class CompletedAsyncMethodEvent : IEventPipeStreamEvent
{
  public required string StateMachineName { get; init; }
  public required List<List<EventRecordWithMetadata>> MethodTraces { get; init; }
}

[AppComponent]
public class CompletedAsyncMethodHandler(
  IJsonMethodsKafkaProducer producer
) : IEventPipeStreamEventHandler
{
  public void Handle(IEventPipeStreamEvent eventPipeStreamEvent)
  {
    if (!ProcfilerOnlineFeatures.ProduceEventsToKafka.IsEnabled()) return;
    if (eventPipeStreamEvent is not CompletedAsyncMethodEvent completedAsyncMethodEvent) return;

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