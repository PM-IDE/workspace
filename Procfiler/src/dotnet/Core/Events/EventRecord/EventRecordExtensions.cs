using System.Diagnostics.CodeAnalysis;
using System.Runtime.CompilerServices;
using Core.Constants.TraceEvents;
using Core.Utils;
using Microsoft.Diagnostics.Tracing;

namespace Core.Events.EventRecord;

public readonly record struct MethodIdToFqn(long Id, string Fqn);

public static class EventRecordExtensions
{
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

  public static MethodIdToFqn? TryGetMethodInfo(this EventRecordWithMetadata eventRecord)
  {
    if (eventRecord.EventName is not TraceEventsConstants.MethodLoadVerbose) return null;

    var methodId = eventRecord.Metadata.GetValueOrDefault(TraceEventsConstants.MethodId);
    var name = eventRecord.Metadata.GetValueOrDefault(TraceEventsConstants.MethodName);
    var methodNamespace = eventRecord.Metadata.GetValueOrDefault(TraceEventsConstants.MethodNamespace);
    var signature = eventRecord.Metadata.GetValueOrDefault(TraceEventsConstants.MethodSignature);

    if (name is { } && methodNamespace is { } && signature is { } && methodId is { })
    {
      var mergedName = MethodsUtil.ConcatenateMethodDetails(name, methodNamespace, signature);
      return new MethodIdToFqn(methodId.ParseId(), mergedName);
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