﻿using System.Diagnostics.CodeAnalysis;
using System.Runtime.CompilerServices;
using Core.Constants.TraceEvents;
using Core.Utils;
using Microsoft.Diagnostics.Tracing;

namespace Core.Events.EventRecord;

public readonly record struct MethodIdToFqn(long Id, string Fqn);
public readonly record struct TypeIdToName(long Id, string Name);

public static class EventRecordExtensions
{
  public static bool IsTaskRelatedEvent(this EventRecordWithMetadata eventRecord) =>
    eventRecord.EventClass.StartsWith(TraceEventsConstants.TaskCommonPrefix) ||
    eventRecord.EventClass.StartsWith(TraceEventsConstants.AwaitCommonPrefix);

  public static bool IsTaskExecutionEvent(this EventRecordWithMetadata eventRecord)
  {
    return eventRecord.EventClass is TraceEventsConstants.TaskExecuteStart or TraceEventsConstants.TaskExecuteStop;
  }

  public static bool IsTaskExecuteStartEvent(this EventRecordWithMetadata eventRecord, out int taskId)
  {
    return IsTaskExecutionStartStopEvent(eventRecord, TraceEventsConstants.TaskExecuteStart, out taskId);
  }

  public static bool IsTaskExecuteStopEvent(this EventRecordWithMetadata eventRecord, out int taskId)
  {
    return IsTaskExecutionStartStopEvent(eventRecord, TraceEventsConstants.TaskExecuteStop, out taskId);
  }

  private static bool IsTaskExecutionStartStopEvent(this EventRecordWithMetadata eventRecord, string eventClass, out int executedTaskId)
  {
    executedTaskId = -1;

    if (eventRecord.EventClass != eventClass) return false;

    executedTaskId = ExtractTaskId(eventRecord);

    return true;
  }

  public static bool IsTaskWaitSendOrStopEvent(this EventRecordWithMetadata eventRecord) =>
    eventRecord.EventClass is TraceEventsConstants.TaskWaitSend or TraceEventsConstants.TaskWaitStop;

  public static bool IsTaskWaitStopEvent(this EventRecordWithMetadata eventRecord, out int waitedTaskId, out int originatingTaskId)
  {
    return eventRecord.IsTaskWaitStopOrSendEventImpl(TraceEventsConstants.TaskWaitStop, out waitedTaskId, out originatingTaskId);
  }

  private static bool IsTaskWaitStopOrSendEventImpl(this EventRecordWithMetadata eventRecord, string eventClass, out int taskId, out int originatingTaskId)
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

  public static bool IsTaskWaitSendEvent(this EventRecordWithMetadata eventRecord, out int scheduledTaskId, out int originatingTaskId)
  {
    return eventRecord.IsTaskWaitStopOrSendEventImpl(TraceEventsConstants.TaskWaitSend, out scheduledTaskId, out originatingTaskId);
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

  public static TypeIdToName? TryExtractTypeIdToName(this EventRecordWithMetadata eventRecord)
  {
    if (eventRecord.EventName is not TraceEventsConstants.TypeBulkType) return null;

    var id = eventRecord.Metadata.GetValueOrDefault(TraceEventsConstants.TypeBulkTypeTypeId);
    var name = eventRecord.Metadata.GetValueOrDefault(TraceEventsConstants.TypeBulkTypeTypeName);

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