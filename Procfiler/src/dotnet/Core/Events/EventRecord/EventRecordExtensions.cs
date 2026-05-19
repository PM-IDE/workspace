using System.Diagnostics.CodeAnalysis;
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
  extension(EventRecordWithMetadata eventRecord)
  {
    public bool IsTaskRelatedEvent() =>
      eventRecord.EventClass.StartsWith(TraceEventsConstants.TaskCommonPrefix);

    public bool IsTaskExecutionEvent() =>
      eventRecord.EventClass is TraceEventsConstants.TaskExecuteStart or TraceEventsConstants.TaskExecuteStop;

    public bool IsTaskExecuteStartEvent(out int taskId, out int originatingTaskId) =>
      eventRecord.IsTaskRelatedEvent(TraceEventsConstants.TaskExecuteStart, out taskId, out originatingTaskId);

    public bool IsTaskExecuteStopEvent(out int taskId, out int originatingTaskId) =>
      eventRecord.IsTaskRelatedEvent(TraceEventsConstants.TaskExecuteStop, out taskId, out originatingTaskId);

    public bool IsTaskWaitSendOrStopEvent() =>
      eventRecord.EventClass is TraceEventsConstants.TaskWaitSend or TraceEventsConstants.TaskWaitStop;

    public bool IsTaskWaitStopEvent(out int waitedTaskId, out int originatingTaskId) =>
      eventRecord.IsTaskRelatedEvent(TraceEventsConstants.TaskWaitStop, out waitedTaskId, out originatingTaskId);

    private bool IsTaskRelatedEvent(string eventClass, out int taskId, out int originatingTaskId)
    {
      taskId = -1;
      originatingTaskId = -1;

      if (eventRecord.EventClass != eventClass) return false;

      taskId = ExtractTaskId(eventRecord);
      originatingTaskId = ExtractOriginatingTaskId(eventRecord);

      return true;
    }
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

  extension(EventRecordWithMetadata eventRecord)
  {
    public TaskWaitSendEventData? IsTaskWaitSendEvent()
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

    public bool IsAwaitContinuationScheduled(out int scheduledTaskId)
    {
      scheduledTaskId = -1;
      if (eventRecord.EventClass is not TraceEventsConstants.AwaitTaskContinuationScheduledSend) return false;

      scheduledTaskId = int.Parse(eventRecord.Metadata[TraceEventsConstants.OriginatingTaskId]);
      return true;
    }

    public bool IsGcSampledObjectAlloc([NotNullWhen(true)] out string? typeName)
    {
      typeName = null;
      if (eventRecord.EventClass is not TraceEventsConstants.GcSampledObjectAllocation) return false;

      typeName = eventRecord.Metadata[TraceEventsConstants.CommonTypeName];
      return true;
    }

    public string GetAllocatedTypeNameOrThrow() =>
      eventRecord.Metadata[TraceEventsConstants.CommonTypeName];

    public bool IsMethodStartEndProvider() =>
      eventRecord.EventClass is not (TraceEventsConstants.GcSetGcHandle or TraceEventsConstants.GcDestroyGcHandle);

    public MethodIdToMethodInfo? TryGetMethodInfo()
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

    private (string? Name, string? Namespace, string? Signature) GetMethodFqnParts()
    {
      var name = eventRecord.Metadata.ValueOrDefault(TraceEventsConstants.MethodName);
      var methodNamespace = eventRecord.Metadata.ValueOrDefault(TraceEventsConstants.MethodNamespace);
      var signature = eventRecord.Metadata.ValueOrDefault(TraceEventsConstants.MethodSignature);

      return (name, methodNamespace, signature);
    }

    public ExtendedMethodIdToFqn? TryGetExtendedMethodInfo()
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

    public TypeIdToName? TryExtractTypeIdToName()
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
  }

  public static EventRecordTime ToTime(this TraceEvent traceEvent) => new()
  {
    LoggedAt = traceEvent.TimeStamp.ToUniversalTime(),
#pragma warning disable CS0618 // Type or member is obsolete
    QpcStamp = traceEvent.TimeStampQPC,
#pragma warning restore CS0618 // Type or member is obsolete
    RelativeStampMSec = traceEvent.TimeStampRelativeMSec
  };

  extension(EventRecordWithMetadata evt)
  {
    public bool IsOcelActivityBegin(out Guid id, out string name) =>
      evt.IsActivityStartOrEnd(TraceEventsConstants.OcelActivityBegin, out id, out name);

    private bool IsActivityStartOrEnd(string eventClass, out Guid id, out string name)
    {
      id = Guid.Empty;
      name = string.Empty;

      if (evt.EventClass != eventClass) return false;

      id = Guid.Parse(evt.Metadata[TraceEventsConstants.OcelActivityId]);
      name = evt.Metadata[TraceEventsConstants.OcelActivityName];

      return true;
    }

    public bool IsOcelActivityEnd(out Guid id, out string name) =>
      evt.IsActivityStartOrEnd(TraceEventsConstants.OcelActivityEnd, out id, out name);

    public bool IsOcelObjectEvent(out long objectId, out string? category)
    {
      objectId = -1;
      category = null;

      if (evt.EventClass is not TraceEventsConstants.OcelObjectAllocated) return false;

      objectId = int.Parse(evt.Metadata[TraceEventsConstants.OcelObjectId]);
      category = evt.Metadata[TraceEventsConstants.OcelObjectType];
      return true;
    }

    public bool IsOcelGlobalEvent(out long objectId, out string? activityName, out string? category)
    {
      objectId = -1;
      activityName = null;
      category = null;

      if (evt.EventClass is not TraceEventsConstants.OcelGlobalObjectEvent) return false;

      objectId = int.Parse(evt.Metadata[TraceEventsConstants.OcelObjectId]);
      category = evt.Metadata[TraceEventsConstants.OcelObjectType];
      activityName = evt.Metadata[TraceEventsConstants.OcelActivityName];

      return true;
    }

    private bool IsOcelActivitiesBatchEvent(string eventClass, out Guid batchId, out string[] names)
    {
      batchId = Guid.Empty;
      names = null!;

      if (evt.EventClass != eventClass) return false;

      batchId = Guid.Parse(evt.Metadata[TraceEventsConstants.OcelActivitiesBatchId]);
      names = evt.Metadata[TraceEventsConstants.OcelActivitiesBatchNames].Split(';');

      return true;
    }

    public bool IsOcelActivitiesBatchEnd(out Guid batchId, out string[] names) =>
      IsOcelActivitiesBatchEvent(evt, TraceEventsConstants.OcelBatchActivitiesEnd, out batchId, out names);
  }
}