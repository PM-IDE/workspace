using System.Diagnostics.Tracing;

namespace ProcfilerEventSources;

[EventSource(Name = $"{nameof(ProcfilerBusinessEventsSource)}")]
public class ProcfilerBusinessEventsSource : EventSource
{
  private const int ProcfilerBusinessEventId = 6000;


  public static ProcfilerBusinessEventsSource Instance { get; } = new();

  private ProcfilerBusinessEventsSource()
  {
  }


  [Event(ProcfilerBusinessEventId, Level = EventLevel.LogAlways)]
  public void BusinessEvent(int level, string message, string attributes) =>
    WriteEvent(ProcfilerBusinessEventId, level, message, attributes);
}