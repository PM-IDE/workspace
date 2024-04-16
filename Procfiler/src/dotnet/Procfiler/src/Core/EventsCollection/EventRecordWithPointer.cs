using Procfiler.Core.EventRecord;

namespace Procfiler.Core.EventsCollection;

public readonly struct EventRecordWithPointer
{
  public required EventRecordWithMetadata Event { get; init; }
  public required EventPointer EventPointer { get; init; }

  public void Deconstruct(out EventPointer pointer, out EventRecordWithMetadata eventRecord)
  {
    pointer = EventPointer;
    eventRecord = Event;
  }
}