using Core.Constants.TraceEvents;
using Microsoft.Diagnostics.Tracing;

namespace Core.Events.EventRecord;

public struct EventRecordTime
{
  public static EventRecordTime Default { get; } = new()
  {
    LoggedAt = DateTime.UnixEpoch,
    QpcStamp = 0,
    RelativeStampMSec = 0
  };


  public required long QpcStamp { get; init; }
  public required DateTime LoggedAt { get; init; }
  public double? RelativeStampMSec { get; init; }
}

public class EventRecord
{
  public EventRecordTime Time { get; private set; }
  public string EventClass { get; set; }
  public long ManagedThreadId { get; private set; }
  public long NativeThreadId { get; private set; }
  public Guid ActivityId { get; }
  public string EventName { get; set; }
  public int StackTraceId { get; }


  public EventRecord(EventRecordTime time, string eventClass, long managedThreadId, long nativeThreadId, Guid activityId,
    int stackTraceId)
  {
    Time = time;
    ActivityId = activityId;
    EventClass = eventClass;
    ManagedThreadId = managedThreadId;
    EventName = EventClass;
    StackTraceId = stackTraceId;
    NativeThreadId = nativeThreadId;
  }

  public EventRecord(TraceEvent @event, long managedThreadId, long nativeThreadId, int stackTraceId)
    : this(@event.ToTime(), @event.EventName, managedThreadId, nativeThreadId, @event.ActivityID, stackTraceId)
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
    NativeThreadId = other.NativeThreadId;
  }

  public void UpdateWith(FromMethodEventRecordUpdateDto updateDto)
  {
    Time = new EventRecordTime
    {
      QpcStamp = updateDto.QpcStamp,
      LoggedAt = updateDto.LoggedAt,
      RelativeStampMSec = null
    };

    ManagedThreadId = updateDto.ManagedThreadId;
    NativeThreadId = updateDto.NativeThreadId;

    EventClass = updateDto.IsStart switch
    {
      true => TraceEventsConstants.ProcfilerMethodStart,
      false => TraceEventsConstants.ProcfilerMethodEnd
    };
  }
}

public readonly ref struct FromMethodEventRecordUpdateDto
{
  public required long QpcStamp { get; init; }
  public required DateTime LoggedAt { get; init; }
  public required long ManagedThreadId { get; init; }
  public required long NativeThreadId { get; init; }
  public required bool IsStart { get; init; }
}

public class EventRecordWithMetadata : EventRecord
{
  public static EventRecordWithMetadata CreateUninitialized() =>
    new(EventRecordTime.Default, string.Empty, -1, -1, -1, new EventMetadata());


  public IEventMetadata Metadata { get; }


  public EventRecordWithMetadata(TraceEvent @event, long managedThreadId, long nativeThreadId, int stackTraceId)
    : base(@event, managedThreadId, nativeThreadId, stackTraceId)
  {
    Metadata = new EventMetadata(@event);
  }

  public EventRecordWithMetadata(
    EventRecordTime time, string eventClass, long managedThreadId, long nativeThreadId, int stackTraceId, IEventMetadata metadata)
    : base(time, eventClass, managedThreadId, nativeThreadId, Guid.Empty, stackTraceId)
  {
    Metadata = metadata;
  }

  private EventRecordWithMetadata(EventRecordWithMetadata other) : base(other)
  {
    Metadata = new EventMetadata(other.Metadata);
  }

  public EventRecordWithMetadata DeepClone() => new(this);
}