using System.Diagnostics.CodeAnalysis;
using System.Runtime.CompilerServices;
using Procfiler.Core.Constants.TraceEvents;
using Procfiler.Utils;

namespace Procfiler.Core.EventRecord;

public struct EventRecordTime
{
  public static EventRecordTime Default { get; } = new()
  {
    LoggedAt = DateTime.UnixEpoch,
    QpcStamp = 0,
    RelativeStampMSec = 0
  };

  public static EventRecordTime QpcOnly(long qpcStamp) => new()
  {
    QpcStamp = qpcStamp,
    LoggedAt = null,
    RelativeStampMSec = null
  };


  public DateTime? LoggedAt { get; init; }
  public double? RelativeStampMSec { get; init; }
  public required long QpcStamp { get; init; }
}

public class EventRecord
{
  public EventRecordTime Time { get; private set; }
  public string EventClass { get; set; }
  public long ManagedThreadId { get; private set; }
  public Guid ActivityId { get; }
  public string EventName { get; set; }
  public int StackTraceId { get; }


  public EventRecord(EventRecordTime time, string eventClass, long managedThreadId, Guid activityId, int stackTraceId)
  {
    Time = time;
    ActivityId = activityId;
    EventClass = eventClass;
    ManagedThreadId = managedThreadId;
    EventName = EventClass;
    StackTraceId = stackTraceId;
  }

  public EventRecord(TraceEvent @event, long managedThreadId, int stackTraceId)
    : this(@event.ToTime(), @event.EventName, managedThreadId, @event.ActivityID, stackTraceId)
  {
  }

  public EventRecord(EventRecord other)
  {
    Time = other.Time;
    EventClass = other.EventClass;
    ManagedThreadId = other.ManagedThreadId;
    ActivityId = other.ActivityId;
    EventName = other.EventName;
    StackTraceId = other.StackTraceId;
  }


  public void UpdateWith(FromFrameInfoCreationContext context)
  {
    Time = new EventRecordTime
    {
      QpcStamp = context.FrameInfo.QpcTimeStamp,
      LoggedAt = null,
      RelativeStampMSec = null
    };

    ManagedThreadId = context.ManagedThreadId;
    EventClass = context.FrameInfo.IsStart switch
    {
      true => TraceEventsConstants.ProcfilerMethodStart,
      false => TraceEventsConstants.ProcfilerMethodEnd
    };
  }
}

public class EventRecordWithMetadata : EventRecord
{
  public static EventRecordWithMetadata CreateUninitialized() => new(EventRecordTime.Default, string.Empty, -1, -1, new EventMetadata());


  public IEventMetadata Metadata { get; }


  public EventRecordWithMetadata(TraceEvent @event, long managedThreadId, int stackTraceId)
    : base(@event, managedThreadId, stackTraceId)
  {
    Metadata = new EventMetadata(@event);
  }

  public EventRecordWithMetadata(
    EventRecordTime time, string eventClass, long managedThreadId, int stackTraceId, IEventMetadata metadata)
    : base(time, eventClass, managedThreadId, Guid.Empty, stackTraceId)
  {
    Metadata = metadata;
  }

  public EventRecordWithMetadata(EventRecordWithMetadata other) : base(other)
  {
    Metadata = new EventMetadata(other.Metadata);
  }

  public EventRecordWithMetadata DeepClone() => new(this);
}

public static class EventRecordExtensions
{
  public readonly record struct MethodStartEndEventInfo(string Frame, bool IsStart);


  public static bool IsMethodStartOrEndEvent(this EventRecordWithMetadata eventRecord) =>
    eventRecord.EventClass is TraceEventsConstants.ProcfilerMethodStart or TraceEventsConstants.ProcfilerMethodEnd;


  public static MethodStartEndEventInfo GetMethodStartEndEventInfo(this EventRecordWithMetadata eventRecord)
    => eventRecord.TryGetMethodStartEndEventInfo() ?? throw new ArgumentOutOfRangeException();

  public static MethodStartEndEventInfo? TryGetMethodStartEndEventInfo(this EventRecordWithMetadata eventRecord)
  {
    if (IsMethodStartOrEndEvent(eventRecord))
    {
      return new MethodStartEndEventInfo(
        eventRecord.Metadata[TraceEventsConstants.ProcfilerMethodName],
        eventRecord.EventClass is TraceEventsConstants.ProcfilerMethodStart
      );
    }

    return null;
  }

  public static bool IsTaskRelatedEvent(this EventRecordWithMetadata eventRecord) =>
    eventRecord.EventClass.StartsWith(TraceEventsConstants.TaskCommonPrefix) ||
    eventRecord.EventClass.StartsWith(TraceEventsConstants.AwaitCommonPrefix);

  public static bool IsTaskWaitSendOrStopEvent(this EventRecordWithMetadata eventRecord) =>
    eventRecord.EventClass is TraceEventsConstants.TaskWaitSend or TraceEventsConstants.TaskWaitStop;

  public static bool IsTaskWaitStopEvent(this EventRecordWithMetadata eventRecord, out int waitedTaskId)
  {
    waitedTaskId = -1;

    if (eventRecord.EventClass is not TraceEventsConstants.TaskWaitStop) return false;

    waitedTaskId = ExtractTaskId(eventRecord);
    return true;
  }

  [MethodImpl(MethodImplOptions.AggressiveInlining)]
  private static int ExtractTaskId(EventRecordWithMetadata eventRecord) => int.Parse(eventRecord.Metadata[TraceEventsConstants.TaskId]);

  public static bool IsTaskWaitSendEvent(this EventRecordWithMetadata eventRecord, out int scheduledTaskId)
  {
    scheduledTaskId = -1;
    if (eventRecord.EventClass is not TraceEventsConstants.TaskWaitSend) return false;

    scheduledTaskId = ExtractTaskId(eventRecord);
    return true;
  }

  public static bool IsAwaitContinuationScheduled(this EventRecordWithMetadata eventRecord, out int scheduledTaskId)
  {
    scheduledTaskId = -1;
    if (eventRecord.EventClass is not TraceEventsConstants.AwaitTaskContinuationScheduledSend) return false;

    scheduledTaskId = int.Parse(eventRecord.Metadata[TraceEventsConstants.OriginatingTaskId]);
    return true;
  }

  public static bool IsGcSampledObjectAlloc(
    this EventRecordWithMetadata eventRecord, [NotNullWhen(true)] out string? typeName)
  {
    typeName = null;
    if (eventRecord.EventClass is not TraceEventsConstants.GcSampledObjectAllocation) return false;

    typeName = eventRecord.Metadata[TraceEventsConstants.CommonTypeName];
    return true;
  }

  public static string GetAllocatedTypeNameOrThrow(this EventRecordWithMetadata eventRecord) =>
    eventRecord.Metadata[TraceEventsConstants.CommonTypeName];

  public static bool IsMethodStartEndProvider(this EventRecordWithMetadata eventRecord) =>
    eventRecord.EventClass is not (TraceEventsConstants.GcSetGcHandle or TraceEventsConstants.GcDestroyGcHandle);
}