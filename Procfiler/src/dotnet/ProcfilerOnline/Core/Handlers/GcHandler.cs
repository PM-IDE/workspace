using Core.Container;
using Core.Events.EventRecord;
using ProcfilerOnline.Core.Features;

namespace ProcfilerOnline.Core.Handlers;

public class GcEvent : IEventPipeStreamEvent
{
  public required List<EventRecordWithMetadata> GcTrace { get; init; }
}

[AppComponent]
public class GcHandler : IEventPipeStreamEventHandler
{
  public void Handle(IEventPipeStreamEvent eventPipeStreamEvent)
  {
    if (!ProcfilerOnlineFeatures.ProduceEventsToKafka.IsEnabled()) return;

    Console.WriteLine("Handled a completed GC");
  }
}