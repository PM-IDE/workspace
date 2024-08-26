using Core.Container;
using ProcfilerOnline.Core.Features;
using ProcfilerOnline.Integrations.Kafka.Json;

namespace ProcfilerOnline.Core.Handlers;

public class CompletedMethodExecutionEvent : IEventPipeStreamEvent
{
  public required TargetMethodFrame Frame { get; init; }
}

[AppComponent]
public class CompletedMethodExecutionHandler(
  IJsonMethodsKafkaProducer producer
) : IEventPipeStreamEventHandler
{
  public void Handle(IEventPipeStreamEvent eventPipeStreamEvent)
  {
    if (!ProcfilerOnlineFeatures.ProduceEventsToKafka.IsEnabled()) return;
    if (eventPipeStreamEvent is not CompletedMethodExecutionEvent @event) return;
    if (@event.Frame.MethodFullName is not { } methodFullName) return;

    var message = new JsonMethodsExecutionKafkaMessage
    {
      Events = @event.Frame.InnerEvents.Select(JsonEventRecordWithMetadataKafkaDto.FromEventRecord).ToList(),
      MethodFullName = methodFullName,
    };

    producer.Produce(Guid.NewGuid(), message);
  }
}