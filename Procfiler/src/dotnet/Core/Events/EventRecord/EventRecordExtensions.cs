using System.Diagnostics.CodeAnalysis;
using System.Runtime.CompilerServices;
using Core.Constants.TraceEvents;
using Core.Utils;
using Microsoft.Diagnostics.Tracing;

namespace Core.Events.EventRecord;

public readonly record struct MethodIdToFqn(long Id, string Fqn);

public record ExtendedMethodInfo(string Name, string Namespace, string Signature)
{
  public string Fqn { get; } = MethodsUtil.ConcatenateMethodDetails(Name, Namespace, Signature);
}

public readonly record struct ExtendedMethodIdToFqn(long Id, ExtendedMethodInfo ExtendedMethodInfo);

public readonly record struct TypeIdToName(long Id, string Name);

public static class EventRecordExtensions
{
  public static bool IsTaskRelatedEvent(this EventRecordWithMetadata eventRecord) =>
    eventRecord.EventClass.StartsWith(TraceEventsConstants.TaskCommonPrefix) ||
    eventRecord.EventClass.StartsWith(TraceEventsConstants.AwaitCommonPrefix);

  public static bool IsTaskExecutionEvent(this EventRecordWithMetadata eventRecord) =>
    eventRecord.EventClass is TraceEventsConstants.TaskExecuteStart or TraceEventsConstants.TaskExecuteStop;

  public static bool IsTaskExecuteStartEvent(this EventRecordWithMetadata eventRecord, out int taskId, out int originatingTaskId) =>
    eventRecord.IsTaskRelatedEvent(TraceEventsConstants.TaskExecuteStart, out taskId, out originatingTaskId);

  public static bool IsTaskExecuteStopEvent(this EventRecordWithMetadata eventRecord, out int taskId, out int originatingTaskId) =>
    eventRecord.IsTaskRelatedEvent(TraceEventsConstants.TaskExecuteStop, out taskId, out originatingTaskId);

  public static bool IsTaskWaitSendOrStopEvent(this EventRecordWithMetadata eventRecord) =>
    eventRecord.EventClass is TraceEventsConstants.TaskWaitSend or TraceEventsConstants.TaskWaitStop;

  public static bool IsTaskWaitStopEvent(this EventRecordWithMetadata eventRecord, out int waitedTaskId, out int originatingTaskId) =>
    eventRecord.IsTaskRelatedEvent(TraceEventsConstants.TaskWaitStop, out waitedTaskId, out originatingTaskId);

  private static bool IsTaskRelatedEvent(
    this EventRecordWithMetadata eventRecord, string eventClass, out int taskId, out int originatingTaskId)
  {
    taskId = -1;
    originatingTaskId = -1;

    if (eventRecord.EventClass != eventClass) return false;

    taskId = ExtractTaskId(eventRecord);
    originatingTaskId = ExtractOriginatingTaskId(eventRecord);

    return true;
  }

  [MethodImpl(MethodImplOptions.AggressiveInlining)]
  private static int ExtractOriginatingTaskId(EventRecordWithMetadata eventRecord) =>
    int.Parse(eventRecord.Metadata[TraceEventsConstants.OriginatingTaskId]);

  [MethodImpl(MethodImplOptions.AggressiveInlining)]
  private static int ExtractTaskId(EventRecordWithMetadata eventRecord) =>
    int.Parse(eventRecord.Metadata[TraceEventsConstants.TaskId]);

  public readonly struct TaskWaitSendEventData
  {
    public required int TaskId { get; init; }
    public required int OriginatingTaskId { get; init; }
    public required int ContinueWithTaskId { get; init; }
    public required bool IsAsync { get; init; }
  }

  public static TaskWaitSendEventData? IsTaskWaitSendEvent(this EventRecordWithMetadata eventRecord)
  {
    if (!eventRecord.IsTaskRelatedEvent(TraceEventsConstants.TaskWaitSend, out var scheduledTaskId, out var originatingTaskId))
    {
      return null;
    }

    var continueWithTaskId = int.Parse(eventRecord.Metadata[TraceEventsConstants.ContinueWithTaskId]);
    var isAsync = eventRecord.Metadata[TraceEventsConstants.AsyncBehaviorAttribute] == TraceEventsConstants.AsyncBehaviour;

    return new TaskWaitSendEventData
    {
      IsAsync = isAsync,
      TaskId = scheduledTaskId,
      OriginatingTaskId = originatingTaskId,
      ContinueWithTaskId = continueWithTaskId
    };
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

  public static MethodIdToFqn? TryGetMethodInfo(this EventRecordWithMetadata eventRecord)
  {
    if (eventRecord.EventName is not TraceEventsConstants.MethodLoadVerbose) return null;

    var methodId = eventRecord.Metadata.ValueOrDefault(TraceEventsConstants.MethodId);
    var (name, methodNamespace, signature) = eventRecord.GetMethodFqnParts();

    if (name is { } && methodNamespace is { } && signature is { } && methodId is { })
    {
      var mergedName = MethodsUtil.ConcatenateMethodDetails(name, methodNamespace, signature);
      return new MethodIdToFqn(methodId.ParseId(), mergedName);
    }

    return null;
  }

  private static (string? Name, string? Namespace, string? Signature) GetMethodFqnParts(this EventRecordWithMetadata eventRecord)
  {
    var name = eventRecord.Metadata.ValueOrDefault(TraceEventsConstants.MethodName);
    var methodNamespace = eventRecord.Metadata.ValueOrDefault(TraceEventsConstants.MethodNamespace);
    var signature = eventRecord.Metadata.ValueOrDefault(TraceEventsConstants.MethodSignature);

    return (name, methodNamespace, signature);
  }

  public static ExtendedMethodIdToFqn? TryGetExtendedMethodInfo(this EventRecordWithMetadata eventRecord)
  {
    if (eventRecord.Metadata.ValueOrDefault(TraceEventsConstants.MethodId) is not { } methodId)
    {
      return null;
    }

    var (name, methodNamespace, signature) = eventRecord.GetMethodFqnParts();

    if (name is null || methodNamespace is null || signature is null)
    {
      return null;
    }

    return new ExtendedMethodIdToFqn(methodId.ParseId(), new ExtendedMethodInfo(name, methodNamespace, signature));
  }

  public static TypeIdToName? TryExtractTypeIdToName(this EventRecordWithMetadata eventRecord)
  {
    if (eventRecord.EventName is not TraceEventsConstants.TypeBulkType) return null;

    var id = eventRecord.Metadata.ValueOrDefault(TraceEventsConstants.TypeBulkTypeTypeId);
    var name = eventRecord.Metadata.ValueOrDefault(TraceEventsConstants.TypeBulkTypeTypeName);

    if (id is { } && name is { })
    {
      return new TypeIdToName(id.ParseId(), name);
    }

    return null;
  }

  public static EventRecordTime ToTime(this TraceEvent traceEvent) => new()
  {
    LoggedAt = traceEvent.TimeStamp.ToUniversalTime(),
    QpcStamp = traceEvent.TimeStampQPC,
    RelativeStampMSec = traceEvent.TimeStampRelativeMSec
  };
}