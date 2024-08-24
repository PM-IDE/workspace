using Core.Constants.TraceEvents;
using Core.Container;

namespace Procfiler.Core.Collector.CustomTraceEvents;

public interface ICustomClrEventsFactory
{
  bool NeedToCreateCustomWrapper(int originalEventId);
  TraceEvent CreateWrapperEvent(TraceEvent rawEvent);
}

[AppComponent]
public class CustomClrEventsFactoryImpl : ICustomClrEventsFactory
{
  private static readonly IReadOnlySet<int> ourSupportedOriginalEvents = new HashSet<int>
  {
    UnknownEventsIds.GcLohCompactId
  };


  public bool NeedToCreateCustomWrapper(int originalEventId) => ourSupportedOriginalEvents.Contains(originalEventId);

  public TraceEvent CreateWrapperEvent(TraceEvent rawEvent) => (int)rawEvent.ID switch
  {
    UnknownEventsIds.GcLohCompactId => CreateGcLohCompactTraceData(rawEvent),
    _ => throw new ArgumentOutOfRangeException()
  };

  private static GcLohCompactTraceData CreateGcLohCompactTraceData(TraceEvent rawEvent) => new(rawEvent);
}