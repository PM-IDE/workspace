using Procfiler.Core.EventRecord.EventRecord;

namespace Procfiler.Core.Serialization.Core;

public interface IEventsSerializer
{
  void SerializeEvents(IEnumerable<EventRecordWithMetadata> events, string path);
}