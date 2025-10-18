using System.Diagnostics.Tracing;

namespace ProcfilerEventSources;

[EventSource(Name = nameof(OcelEventsSource))]
public class OcelEventsSource : EventSource
{
  private const int OcelObjectAllocatedEventId = 7000;
  private const int OcelActivityBeginId = 7001;
  private const int OcelActivityEndId = 7002;
  private const int OcelGloballyAttachedEventId = 7003;
  private const int OcelObjectConsumedEventId = 7004;
  private const int OcelObjectMergeAllocateEventId = 7005;
  private const int OcelObjectConsumeProduceEventId = 7006;


  public static OcelEventsSource Instance { get; } = new();


  private OcelEventsSource()
  {
  }


  [Event(OcelObjectAllocatedEventId, Level = EventLevel.LogAlways)]
  public void OcelObjectAllocated(long objectId, string? type, string attributes) =>
    WriteEvent(OcelObjectAllocatedEventId, objectId, type, attributes);

  [Event(OcelObjectConsumedEventId, Level = EventLevel.LogAlways)]
  public void OcelObjectConsumed(long objectId, string? type, string attributes) =>
    WriteEvent(OcelObjectConsumedEventId, objectId, type, attributes);

  [Event(OcelObjectConsumeProduceEventId, Level = EventLevel.LogAlways)]
  public void OcelConsumeProduce(long objectId, string relatedObjectsIds, string relatedObjectsTypes, string attributes) =>
    WriteEvent(OcelObjectConsumeProduceEventId, objectId, relatedObjectsIds, relatedObjectsTypes, attributes);

  [Event(OcelObjectMergeAllocateEventId, Level = EventLevel.LogAlways)]
  public void OcelMergeAllocate(long objectId, string? type, string relatedObjectIds, string attributes) =>
    WriteEvent(OcelObjectMergeAllocateEventId, objectId, type, relatedObjectIds, attributes);

  [Event(OcelActivityBeginId, Level = EventLevel.LogAlways)]
  public void OcelActivityBegin(Guid activityId, string activity) => WriteEvent(OcelActivityBeginId, activityId, activity);

  [Event(OcelActivityEndId, Level = EventLevel.LogAlways)]
  public void OcelActivityEnd(Guid activityId, string activity) => WriteEvent(OcelActivityEndId, activityId, activity);

  [Event(OcelGloballyAttachedEventId, Level = EventLevel.LogAlways)]
  public void OcelGloballyAttachedEvent(long objectId, string activity, string? objectCategory, string attributes) =>
    WriteEvent(OcelGloballyAttachedEventId, objectId, activity, objectCategory, attributes);
}