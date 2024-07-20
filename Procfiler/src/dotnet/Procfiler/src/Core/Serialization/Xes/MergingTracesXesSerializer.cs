using Core.Utils;
using Procfiler.Core.Collector;

namespace Procfiler.Core.Serialization.Xes;

public class MergingTracesXesSerializer(IXesEventsSessionSerializer sessionSerializer, IProcfilerLogger logger, bool writeAllEventData)
{
  private readonly Dictionary<string, List<EventSessionInfo>> myDocuments = new();


  public void AddTrace(string path, EventSessionInfo sessionInfo)
  {
    myDocuments.GetOrCreate(path, static () => []).Add(sessionInfo);
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