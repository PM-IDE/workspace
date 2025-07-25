﻿using System.Diagnostics.CodeAnalysis;
using System.Runtime.CompilerServices;
using Core.Constants.TraceEvents;
using Core.Utils;
using Microsoft.Diagnostics.Tracing;

namespace Core.Events.EventRecord;

public readonly record struct MethodIdToMethodInfo(long Id, ExtendedMethodInfo Info);

public record ExtendedMethodInfo(string Name, string Namespace, string Signature)
{
  public string Fqn { get; } = MethodsUtil.ConcatenateMethodDetails(Name, Namespace, Signature);
}

public readonly record struct ExtendedMethodIdToFqn(long Id, ExtendedMethodInfo ExtendedMethodInfo);

public readonly record struct TypeIdToName(long Id, string Name);

public static class EventRecordExtensions
{
  public static bool IsTaskRelatedEvent(this EventRecordWithMetadata eventRecord) =>
    eventRecord.EventClass.StartsWith(TraceEventsConstants.TaskCommonPrefix);

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

  public static MethodIdToMethodInfo? TryGetMethodInfo(this EventRecordWithMetadata eventRecord)
  {
    if (eventRecord.EventName is not TraceEventsConstants.MethodLoadVerbose) return null;

    var methodId = eventRecord.Metadata.ValueOrDefault(TraceEventsConstants.MethodId);
    var (name, methodNamespace, signature) = eventRecord.GetMethodFqnParts();

    if (name is { } && methodNamespace is { } && signature is { } && methodId is { })
    {
      return new MethodIdToMethodInfo(methodId.ParseId(), new ExtendedMethodInfo(name, methodNamespace, signature));
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

  public static bool IsOcelActivityBegin(this EventRecordWithMetadata evt, out Guid id, out string name) =>
    evt.IsActivityStartOrEnd(TraceEventsConstants.OcelActivityBegin, out id, out name);

  private static bool IsActivityStartOrEnd(this EventRecordWithMetadata evt, string eventClass, out Guid id, out string name)
  {
    id = Guid.Empty;
    name = string.Empty;

    if (evt.EventClass != eventClass) return false;

    id = Guid.Parse(evt.Metadata[TraceEventsConstants.OcelActivityId]);
    name = evt.Metadata[TraceEventsConstants.OcelActivityName];

    return true;
  }

  public static bool IsOcelActivityEnd(this EventRecordWithMetadata evt, out Guid id, out string name) =>
    evt.IsActivityStartOrEnd(TraceEventsConstants.OcelActivityEnd, out id, out name);

  public static bool IsOcelObjectEvent(this EventRecordWithMetadata evt, out long objectId, out string? category)
  {
    objectId = -1;
    category = null;

    if (evt.EventClass is not TraceEventsConstants.OcelObjectEvent) return false;

    objectId = int.Parse(evt.Metadata[TraceEventsConstants.OcelObjectId]);
    category = evt.Metadata[TraceEventsConstants.OcelObjectCategory];
    return true;
  }

  public static bool IsOcelGlobalEvent(
    this EventRecordWithMetadata evt, out long objectId, out string activityName, out string? category)
  {
    objectId = -1;
    activityName = null;
    category = null;

    if (evt.EventClass is not TraceEventsConstants.OcelGlobalObjectEvent) return false;

    objectId = int.Parse(evt.Metadata[TraceEventsConstants.OcelObjectId]);
    category = evt.Metadata[TraceEventsConstants.OcelObjectCategory];
    activityName = evt.Metadata[TraceEventsConstants.OcelActivityName];

    return true;
  }

  public static bool IsOcelActivitiesBatchBegin(this EventRecordWithMetadata evt, out Guid batchId, out string[] names) =>
    IsOcelActivitiesBatchEvent(evt, TraceEventsConstants.OcelBatchActivitiesBegin, out batchId, out names);

  private static bool IsOcelActivitiesBatchEvent(this EventRecordWithMetadata evt, string eventClass, out Guid batchId, out string[] names)
  {
    batchId = Guid.Empty;
    names = null!;

    if (evt.EventClass != eventClass) return false;

    batchId = Guid.Parse(evt.Metadata[TraceEventsConstants.OcelActivitiesBatchId]);
    names = evt.Metadata[TraceEventsConstants.OcelActivitiesBatchNames].Split(';');

    return true;
  }

  public static bool IsOcelActivitiesBatchEnd(this EventRecordWithMetadata evt, out Guid batchId, out string[] names) =>
    IsOcelActivitiesBatchEvent(evt, TraceEventsConstants.OcelBatchActivitiesEnd, out batchId, out names);

  public static bool IsOcelBatchAttachedEvent(this EventRecordWithMetadata evt, out long objectId, out string activity, out string? category)
  {
    objectId = 0;
    activity = null;
    category = null;

    if (evt.EventClass is not TraceEventsConstants.OcelBatchObjectEvent) return false;

    objectId = int.Parse(evt.Metadata[TraceEventsConstants.OcelObjectId]);
    category = evt.Metadata[TraceEventsConstants.OcelObjectCategory];
    activity = evt.Metadata[TraceEventsConstants.OcelActivityName];

    return true;
  }
}