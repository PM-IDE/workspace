using Core.Constants.TraceEvents;
using Core.Container;
using Core.Events.EventRecord;
using Core.EventsProcessing.Mutators;
using Core.Utils;
using Procfiler.Core.Collector;
using Procfiler.Core.CppProcfiler;
using Procfiler.Utils;

namespace Procfiler.Core.EventRecord;

public readonly record struct EventsCreationContext(EventRecordTime Time, long ManagedThreadId, long NativeThreadId)
{
  public static EventsCreationContext CreateWithUndefinedStackTrace(global::Core.Events.EventRecord.EventRecord record) =>
    new(record.Time, record.ManagedThreadId, record.NativeThreadId);
}

public readonly ref struct FromFrameInfoCreationContext
{
  public required FrameInfo FrameInfo { get; init; }
  public required IGlobalDataWithStacks GlobalData { get; init; }
  public required long ManagedThreadId { get; init; }
  public required long NativeThreadId { get; init; }
}

public interface IProcfilerEventsFactory
{
  EventRecordWithMetadata CreateMethodStartEvent(EventsCreationContext context, string methodName);
  EventRecordWithMetadata CreateMethodEndEvent(EventsCreationContext context, string methodName);
  EventRecordWithMetadata CreateMethodExecutionEvent(EventsCreationContext context, string methodName);
  EventRecordWithMetadata CreateMethodEvent(FromFrameInfoCreationContext context);

  void FillExistingEventWith(FromFrameInfoCreationContext context, EventRecordWithMetadata existingEvent);
}

[AppComponent]
public class ProcfilerEventsFactory(IProcfilerLogger logger) : IProcfilerEventsFactory
{
  private readonly Dictionary<string, string> myStartMethodsNames = new();
  private readonly Dictionary<string, string> myEndMethodsNames = new();
  private readonly Dictionary<string, string> myMethodExecutionNames = new();


  public EventRecordWithMetadata CreateMethodStartEvent(EventsCreationContext context, string methodName) =>
    CreateMethodStartOrEndEvent(context, TraceEventsConstants.ProcfilerMethodStart, methodName);

  private EventRecordWithMetadata CreateMethodStartOrEndEvent(
    EventsCreationContext context, string eventClass, string methodName)
  {
    var (stamp, managedThreadId, nativeThreadId) = context;
    var metadata = CreateMethodEventMetadata(methodName);

    return new EventRecordWithMetadata(stamp, eventClass, managedThreadId, nativeThreadId, -1, metadata)
    {
      EventName = CreateMethodStartOrEndEventName(eventClass, methodName)
    };
  }

  private string CreateMethodStartOrEndEventName(string eventClass, string fqn)
  {
    var map = eventClass switch
    {
      TraceEventsConstants.ProcfilerMethodStart => myStartMethodsNames,
      TraceEventsConstants.ProcfilerMethodEnd => myEndMethodsNames,
      _ => throw new ArgumentOutOfRangeException(nameof(eventClass), eventClass, null)
    };

    return map.GetOrCreate(fqn, () => eventClass + "_{" + MutatorsUtil.TransformMethodLikeNameForEventNameConcatenation(fqn) + "}");
  }

  private static void SetMethodNameInMetadata(IEventMetadata metadata, string fqn)
  {
    metadata[TraceEventsConstants.ProcfilerMethodName] = fqn;
  }

  private static IEventMetadata CreateMethodEventMetadata(string fqn)
  {
    var metadata = new EventMetadata();
    SetMethodNameInMetadata(metadata, fqn);
    return metadata;
  }

  public EventRecordWithMetadata CreateMethodEndEvent(EventsCreationContext context, string methodName) =>
    CreateMethodStartOrEndEvent(context, TraceEventsConstants.ProcfilerMethodEnd, methodName);

  public EventRecordWithMetadata CreateMethodExecutionEvent(EventsCreationContext context, string methodName)
  {
    var (stamp, managedThreadId, nativeThreadId) = context;
    var metadata = CreateMethodEventMetadata(methodName);
    var name = CreateEventNameForMethodExecutionEvent(methodName);
    return new EventRecordWithMetadata(stamp, name, managedThreadId, nativeThreadId, -1, metadata);
  }

  private string CreateEventNameForMethodExecutionEvent(string fqn) =>
    myMethodExecutionNames.GetOrCreate(fqn, () => $"{TraceEventsConstants.ProcfilerMethodExecution}_{fqn}");

  public EventRecordWithMetadata CreateMethodEvent(FromFrameInfoCreationContext context)
  {
    var fqn = ExtractMethodName(context);
    var time = new EventRecordTime
    {
      QpcStamp = context.FrameInfo.QpcTimeStamp,
      LoggedAt = QpcUtil.ConvertQpcTimeToDateTimeUtc(context.FrameInfo.QpcTimeStamp, context.GlobalData)
    };

    var creationContext = new EventsCreationContext(time, context.ManagedThreadId, context.NativeThreadId);

    return context.FrameInfo.IsStart switch
    {
      true => CreateMethodStartEvent(creationContext, fqn),
      false => CreateMethodEndEvent(creationContext, fqn)
    };
  }

  private string ExtractMethodName(FromFrameInfoCreationContext context)
  {
    var methodId = context.FrameInfo.FunctionId;
    if (context.GlobalData.FindMethodName(methodId) is not { } fqn)
    {
      logger.LogTrace("Failed to get fqn for {FunctionId}", methodId);
      fqn = $"System.Undefined.{methodId}[instance.void..()]";
    }

    return fqn;
  }

  public void FillExistingEventWith(FromFrameInfoCreationContext context, EventRecordWithMetadata existingEvent)
  {
    existingEvent.UpdateWith(new FromMethodEventRecordUpdateDto
    {
      IsStart = context.FrameInfo.IsStart,
      LoggedAt = QpcUtil.ConvertQpcTimeToDateTimeUtc(context.FrameInfo.QpcTimeStamp, context.GlobalData),
      QpcStamp = context.FrameInfo.QpcTimeStamp,
      ManagedThreadId = context.ManagedThreadId,
      NativeThreadId = context.NativeThreadId
    });

    var fqn = ExtractMethodName(context);
    existingEvent.EventName = CreateMethodStartOrEndEventName(existingEvent.EventClass, fqn);
    SetMethodNameInMetadata(existingEvent.Metadata, fqn);
  }
}