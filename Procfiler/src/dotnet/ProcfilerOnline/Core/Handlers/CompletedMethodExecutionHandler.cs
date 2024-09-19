using Core.Container;
using ProcfilerOnline.Core.Features;
using ProcfilerOnline.Integrations.Kafka.Bxes;

namespace ProcfilerOnline.Core.Handlers;

public class CompletedMethodExecutionEvent : IEventPipeStreamEvent
{
  public required string ApplicationName { get; init; }
  public required TargetMethodFrame Frame { get; init; }
}

[AppComponent]
public class CompletedMethodExecutionHandler(
  IBxesMethodsKafkaProducer producer
) : IEventPipeStreamEventHandler
{
  public void Handle(IEventPipeStreamEvent eventPipeStreamEvent)
  {
    if (!ProcfilerOnlineFeatures.ProduceEventsToKafka.IsEnabled()) return;
    if (eventPipeStreamEvent is not CompletedMethodExecutionEvent @event) return;

    var message = new BxesKafkaMethodsExecutionMessage
    {
      ApplicationNamne = @event.ApplicationName,
      MethodName = @event.Frame.MethodFullName ?? "UNRESOLVED",
      Trace = @event.Frame.InnerEvents
    };

    producer.Produce(Guid.NewGuid(), message);
  }
}