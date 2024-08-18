using Core.Container;
using Core.Events.EventRecord;
using Microsoft.Extensions.Options;
using ProcfilerOnline.Core.Settings;
using ProcfilerOnline.Integrations.Kafka;

namespace ProcfilerOnline.Core.Handlers;

public class CompletedAsyncMethodEvent : IEventPipeStreamEvent
{
  public required string StateMachineName { get; init; }
  public required List<List<EventRecordWithMetadata>> MethodTraces { get; init; }
}

[AppComponent]
public class CompletedAsyncMethodHandler(
  IOptions<OnlineProcfilerSettings> settings,
  IKafkaProducer<Guid, MethodsExecutionKafkaMessage> producer
) : IEventPipeStreamEventHandler
{
  public void Handle(IEventPipeStreamEvent eventPipeStreamEvent)
  {
    if (!settings.Value.ProduceEventsToKafka) return;
    if (eventPipeStreamEvent is not CompletedAsyncMethodEvent completedAsyncMethodEvent) return;

    foreach (var methodTrace in completedAsyncMethodEvent.MethodTraces)
    {
      var message = new MethodsExecutionKafkaMessage
      {
        MethodFullName = completedAsyncMethodEvent.StateMachineName,
        Events = methodTrace
      };

      producer.Produce(Guid.NewGuid(), message);
    }
  }
}