using Procfiler.Core.Collector;

namespace Procfiler.Core.Serialization.Core;

public interface IEventsSessionSerializer
{
  void SerializeEvents(IEnumerable<EventSessionInfo> eventsTraces, string path, bool writeAllMetadata);
}