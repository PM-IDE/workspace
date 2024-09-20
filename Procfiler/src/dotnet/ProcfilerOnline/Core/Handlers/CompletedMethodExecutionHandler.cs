using Core.Container;
using Core.Utils;
using Microsoft.Extensions.Logging;
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
  IBxesMethodsKafkaProducer producer,
  IProcfilerLogger logger
) : IEventPipeStreamEventHandler
{
  public void Handle(IEventPipeStreamEvent eventPipeStreamEvent)
  {
    if (!ProcfilerOnlineFeatures.ProduceEventsToKafka.IsEnabled()) return;
    if (eventPipeStreamEvent is not CompletedMethodExecutionEvent @event) return;

    if (@event.Frame.MethodInfo is null)
    {
      logger.LogWarning("Encountered an event without MethodInfo, will not send it");
      return;
    }

    var message = new BxesKafkaMethodsExecutionMessage
    {
      ProcessName = @event.ApplicationName,
      CaseName = @event.Frame.MethodInfo.Fqn,
      MethodInfo = @event.Frame.MethodInfo,
      Trace = @event.Frame.InnerEvents
    };

    producer.Produce(Guid.NewGuid(), message);
  }
}