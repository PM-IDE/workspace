using Procfiler.Core.EventRecord.EventRecord;

namespace Procfiler.Core.EventRecord.EventsCollection;

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