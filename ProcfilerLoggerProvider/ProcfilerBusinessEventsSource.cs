using System.Diagnostics.Tracing;
using Microsoft.Extensions.Logging;

namespace ProcfilerLoggerProvider;

[EventSource(Name = $"{nameof(ProcfilerBusinessEventsSource)}")]
internal sealed class ProcfilerBusinessEventsSource : EventSource
{
  public const int ProcfilerBusinessEventId = 6000;


  public static ProcfilerBusinessEventsSource Instance { get; } = new();


  [Event(ProcfilerBusinessEventId, Level = EventLevel.LogAlways)]
  public void WriteBusinessEvent(LogLevel level, EventId eventId)
  {
    WriteEvent(ProcfilerBusinessEventId, (int)level, eventId.Id, eventId.Name);
  }
}