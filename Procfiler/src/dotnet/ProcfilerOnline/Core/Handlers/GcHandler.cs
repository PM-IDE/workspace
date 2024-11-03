using Core.Container;
using Core.Events.EventRecord;
using Core.Utils;
using Microsoft.Extensions.Logging;
using ProcfilerOnline.Core.Features;
using ProcfilerOnline.Integrations.Kafka.Bxes;

namespace ProcfilerOnline.Core.Handlers;

public class GcEvent : IEventPipeStreamEvent
{
  public required string ApplicationName { get; init; }
  public required List<EventRecordWithMetadata> GcTrace { get; init; }
}

[AppComponent]
public class GcHandler(IProcfilerLogger logger, IBxesMethodsKafkaProducer producer)  : IEventPipeStreamEventHandler
{
  public void Handle(IEventPipeStreamEvent eventPipeStreamEvent)
  {
    if (eventPipeStreamEvent is not GcEvent gcEvent) return;
    if (!ProcfilerOnlineFeatures.ProduceEventsToKafka.IsEnabled()) return;

    if (!ProcfilerOnlineFeatures.ProduceBxesKafkaEvents.IsEnabled())
    {
      logger.LogError("Only bXES Kafka production is supported for GC traces");
      return;
    }

    const string GcCaseName = "GC";
    var message = new BxesKafkaTrace
    {
      Trace = gcEvent.GcTrace,
      ProcessName = gcEvent.ApplicationName,
      CaseName = GcCaseName,
      Metadata = []
    };

    producer.Produce(Guid.NewGuid(), message);
  }
}