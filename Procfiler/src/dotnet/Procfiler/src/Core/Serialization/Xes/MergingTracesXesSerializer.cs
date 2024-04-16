using Procfiler.Core.Collector;
using Procfiler.Utils;

namespace Procfiler.Core.Serialization.Xes;

public class MergingTracesXesSerializer(IXesEventsSessionSerializer sessionSerializer, IProcfilerLogger logger, bool writeAllEventData)
{
  private readonly Dictionary<string, List<EventSessionInfo>> myDocuments = new();


  public void AddTrace(string path, EventSessionInfo sessionInfo)
  {
    myDocuments.GetOrCreate(path, static () => new List<EventSessionInfo>()).Add(sessionInfo);
  }

  public void SerializeAll()
  {
    using var _ = new PerformanceCookie($"{GetType()}::{nameof(SerializeAll)}", logger);
    foreach (var (path, sessions) in myDocuments)
    {
      sessionSerializer.SerializeEvents(sessions, path, writeAllEventData);
    }
  }
}