using System.Diagnostics.Tracing;

namespace ProcfilerEventSources;

[EventSource(Name = $"{nameof(OcelEventsSource)}")]
public class OcelEventsSource : EventSource
{
  private const int OcelEventId = 7000;
  private const int OcelActivityStartId = 7001;
  private const int OcelActivityEndId = 7002;


  public static OcelEventsSource Instance { get; } = new();


  private OcelEventsSource()
  {
  }


  [Event(OcelEventId, Level = EventLevel.LogAlways)]
  public void OcelEvent(int objectId, string? objectCategory, string attributes) =>
    WriteEvent(OcelEventId, objectId, objectCategory, attributes);

  [Event(OcelActivityStartId, Level = EventLevel.LogAlways)]
  public void OcelActivityBegin(Guid activityId, string activity) => WriteEvent(OcelActivityStartId, activityId, activity);

  [Event(OcelActivityEndId, Level = EventLevel.LogAlways)]
  public void OcelActivityEnd(Guid activityId, string activity) => WriteEvent(OcelActivityEndId, activityId, activity);
}