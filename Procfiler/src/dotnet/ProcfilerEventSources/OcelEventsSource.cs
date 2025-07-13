using System.Diagnostics.Tracing;

namespace ProcfilerEventSources;

[EventSource(Name = nameof(OcelEventsSource))]
public class OcelEventsSource : EventSource
{
  private const int OcelEventId = 7000;
  private const int OcelActivityBeginId = 7001;
  private const int OcelActivityEndId = 7002;
  private const int OcelGloballyAttachedEventId = 7003;


  public static OcelEventsSource Instance { get; } = new();


  private OcelEventsSource()
  {
  }


  [Event(OcelEventId, Level = EventLevel.LogAlways)]
  public void OcelEvent(long objectId, string? objectCategory, string attributes) =>
    WriteEvent(OcelEventId, objectId, objectCategory, attributes);

  [Event(OcelActivityBeginId, Level = EventLevel.LogAlways)]
  public void OcelActivityBegin(Guid activityId, string activity) => WriteEvent(OcelActivityBeginId, activityId, activity);

  [Event(OcelActivityEndId, Level = EventLevel.LogAlways)]
  public void OcelActivityEnd(Guid activityId, string activity) => WriteEvent(OcelActivityEndId, activityId, activity);

  [Event(OcelGloballyAttachedEventId, Level = EventLevel.LogAlways)]
  public void OcelGloballyAttachedEvent(long objectId, string activity, string? objectCategory, string attributes) =>
    WriteEvent(OcelGloballyAttachedEventId, objectId, activity, objectCategory, attributes);
}