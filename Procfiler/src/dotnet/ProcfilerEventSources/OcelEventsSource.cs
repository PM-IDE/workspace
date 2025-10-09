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
  private const int OcelMergedObjectAllocatedEventId = 7005;
  private const int OcelObjectConsumedWithProduceEventId = 7006;


  public static OcelEventsSource Instance { get; } = new();


  private OcelEventsSource()
  {
  }


  [Event(OcelObjectAllocatedEventId, Level = EventLevel.LogAlways)]
  public void ObjectAllocated(long objectId, string? objectCategory, string attributes) =>
    WriteEvent(OcelObjectAllocatedEventId, objectId, objectCategory, attributes);

  [Event(OcelObjectConsumedEventId, Level = EventLevel.LogAlways)]
  public void ObjectConsumed(long objectId, string? objectCategory, string attributes) =>
    WriteEvent(OcelObjectConsumedEventId, objectId, objectCategory, attributes);

  [Event(OcelObjectConsumedWithProduceEventId, Level = EventLevel.LogAlways)]
  public void ObjectConsumedWithProduce(long objectId, string? objectCategory, string relatedObjectIds, string attributes) =>
    WriteEvent(OcelObjectConsumedWithProduceEventId, objectId, objectCategory, relatedObjectIds, attributes);

  [Event(OcelMergedObjectAllocatedEventId, Level = EventLevel.LogAlways)]
  public void MergedObjectAllocated(long objectId, string? objectCategory, string relatedObjectIds, string attributes) =>
    WriteEvent(OcelMergedObjectAllocatedEventId, objectId, objectCategory, relatedObjectIds, attributes);

  [Event(OcelActivityBeginId, Level = EventLevel.LogAlways)]
  public void OcelActivityBegin(Guid activityId, string activity) => WriteEvent(OcelActivityBeginId, activityId, activity);

  [Event(OcelActivityEndId, Level = EventLevel.LogAlways)]
  public void OcelActivityEnd(Guid activityId, string activity) => WriteEvent(OcelActivityEndId, activityId, activity);

  [Event(OcelGloballyAttachedEventId, Level = EventLevel.LogAlways)]
  public void OcelGloballyAttachedEvent(long objectId, string activity, string? objectCategory, string attributes) =>
    WriteEvent(OcelGloballyAttachedEventId, objectId, activity, objectCategory, attributes);
}