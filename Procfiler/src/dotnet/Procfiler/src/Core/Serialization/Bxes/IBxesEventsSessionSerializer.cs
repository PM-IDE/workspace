using Procfiler.Core.Collector;
using Procfiler.Core.Serialization.Core;
using Procfiler.Utils;
using Procfiler.Utils.Container;

namespace Procfiler.Core.Serialization.Bxes;

public interface IBxesEventsSessionSerializer : IEventsSessionSerializer;

[AppComponent]
public class BxesEventsSessionSerializer(IProcfilerLogger logger) : IBxesEventsSessionSerializer
{
  public void SerializeEvents(IEnumerable<EventSessionInfo> eventsTraces, string path, bool writeAllMetadata)
  {
    var serializer = new NotStoringMergingTraceBxesSerializer(logger, writeAllMetadata);

    foreach (var sessionInfo in eventsTraces)
    {
      serializer.WriteTrace(path, sessionInfo);
    }
  }
}