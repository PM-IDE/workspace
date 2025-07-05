using System.Diagnostics.Tracing;

namespace ProcfilerEventSources;

[EventSource(Name = $"{nameof(OcelEventsSource)}")]
public class OcelEventsSource : EventSource
{
  private const int OcelEventId = 7000;
  private const int OcelActivityBeginId = 7001;
  private const int OcelActivityEndId = 7002;
  private const int OcelActivitiesStartId = 7003;
  private const int OcelActivitiesEndId = 7004;
  private const int OcelAttachedToActivityEventId = 7005;


  public static OcelEventsSource Instance { get; } = new();


  private OcelEventsSource()
  {
  }


  [Event(OcelEventId, Level = EventLevel.LogAlways)]
  public void OcelEvent(int objectId, string? objectCategory, string attributes) =>
    WriteEvent(OcelEventId, objectId, objectCategory, attributes);

  [Event(OcelActivityBeginId, Level = EventLevel.LogAlways)]
  public void OcelActivityBegin(Guid activityId, string activity) => WriteEvent(OcelActivityBeginId, activityId, activity);

  [Event(OcelActivityEndId, Level = EventLevel.LogAlways)]
  public void OcelActivityEnd(Guid activityId, string activity) => WriteEvent(OcelActivityEndId, activityId, activity);

  [Event(OcelActivitiesStartId, Level = EventLevel.LogAlways)]
  public void OcelActivitiesBegin(Guid activitiesBatchId, string names) => WriteEvent(OcelActivityEndId, activitiesBatchId, names);

  [Event(OcelActivitiesEndId, Level = EventLevel.LogAlways)]
  public void OcelActivitiesEnd(Guid activitiesBatchId, string names) => WriteEvent(OcelActivityEndId, activitiesBatchId, names);

  [Event(OcelAttachedToActivityEventId, Level = EventLevel.LogAlways)]
  public void OcelAttachedToActivityEvent(int objectId, string activityName, string? objectCategory, string attributes) =>
    WriteEvent(OcelEventId, objectId, activityName, objectCategory, attributes);
}