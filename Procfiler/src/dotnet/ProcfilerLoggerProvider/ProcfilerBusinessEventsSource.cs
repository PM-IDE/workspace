using System.Diagnostics.Tracing;
using Microsoft.Extensions.Logging;

namespace ProcfilerLoggerProvider;

[EventSource(Name = $"{nameof(ProcfilerBusinessEventsSource)}")]
internal sealed class ProcfilerBusinessEventsSource : EventSource
{
  public const int ProcfilerBusinessEventId = 6000;


  public static ProcfilerBusinessEventsSource Instance { get; } = new();


  private ProcfilerBusinessEventsSource()
  {
  }


  [Event(ProcfilerBusinessEventId, Level = EventLevel.LogAlways)]
  public void WriteBusinessEvent(LogLevel level, EventId eventId, string message, List<(string, string)> attributes)
  {
    var values = attributes
      .SelectMany(p => new List<EventSourcePrimitive> { p.Item1, p.Item2 })
      .Prepend(message)
      .Prepend(eventId.Name)
      .Prepend(eventId.Id)
      .Prepend((int)level)
      .ToArray();

    WriteEvent(ProcfilerBusinessEventId, values);
  }
}