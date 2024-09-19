using Core.Container;
using Core.Events.EventRecord;
using ProcfilerOnline.Core.Features;
using ProcfilerOnline.Integrations.Kafka.Bxes;

namespace ProcfilerOnline.Core.Handlers;

public class CompletedAsyncMethodEvent : IEventPipeStreamEvent
{
  public required string ApplicationName { get; init; }
  public required string StateMachineName { get; init; }
  public required List<List<EventRecordWithMetadata>> MethodTraces { get; init; }
}

[AppComponent]
public class CompletedAsyncMethodHandler(
  IBxesMethodsKafkaProducer producer
) : IEventPipeStreamEventHandler
{
  public void Handle(IEventPipeStreamEvent eventPipeStreamEvent)
  {
    if (!ProcfilerOnlineFeatures.ProduceEventsToKafka.IsEnabled()) return;
    if (eventPipeStreamEvent is not CompletedAsyncMethodEvent completedAsyncMethodEvent) return;

    foreach (var methodTrace in completedAsyncMethodEvent.MethodTraces)
    {
      var message = new BxesKafkaMethodsExecutionMessage
      {
        ApplicationName = completedAsyncMethodEvent.ApplicationName,
        MethodName = completedAsyncMethodEvent.StateMachineName,
        Trace = methodTrace
      };

      producer.Produce(Guid.NewGuid(), message);
    }
  }
}