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
  public ExtendedMethodInfo? MethodInfo { get; init; }
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
        ProcessName = completedAsyncMethodEvent.ApplicationName,
        CaseName = completedAsyncMethodEvent.StateMachineName,
        Trace = methodTrace,
        MethodInfo = completedAsyncMethodEvent.MethodInfo
      };

      producer.Produce(Guid.NewGuid(), message);
    }
  }
}