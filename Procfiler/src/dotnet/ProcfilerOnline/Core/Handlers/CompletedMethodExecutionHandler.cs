using Core.Container;
using ProcfilerOnline.Core.Features;
using ProcfilerOnline.Integrations.Kafka;

namespace ProcfilerOnline.Core.Handlers;

public class CompletedMethodExecutionEvent : IEventPipeStreamEvent
{
  public required TargetMethodFrame Frame { get; init; }
}

[AppComponent]
public class CompletedMethodExecutionHandler(
  IKafkaProducer<Guid, MethodsExecutionKafkaMessage> producer
) : IEventPipeStreamEventHandler
{
  public void Handle(IEventPipeStreamEvent eventPipeStreamEvent)
  {
    if (!ProcfilerOnlineFeatures.ProduceEventsToKafka.IsEnabled()) return;
    if (eventPipeStreamEvent is not CompletedMethodExecutionEvent @event) return;
    if (@event.Frame.MethodFullName is not { } methodFullName) return;

    var message = new MethodsExecutionKafkaMessage
    {
      Events = @event.Frame.InnerEvents,
      MethodFullName = methodFullName,
    };

    producer.Produce(Guid.NewGuid(), message);
  }
}