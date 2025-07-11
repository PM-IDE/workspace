﻿using Core.Events.EventRecord;

namespace Core.Methods;

public abstract class TaskEvent
{
  public required int TaskId { get; init; }
  public required int OriginatingTaskId { get; init; }

  public override string ToString() => $"{GetType().Name} TaskId: {TaskId}, OriginatingTaskId: {OriginatingTaskId}";
}

public sealed class UnknownTaskEvent : TaskEvent;

public abstract class TaskWaitEvent : TaskEvent;

public sealed class TaskWaitSendEvent : TaskWaitEvent
{
  public required int ContinueWithTaskId { get; init; }
  public required bool IsAsync { get; init; }

  public override string ToString() =>
    $"{base.ToString()}, {nameof(ContinueWithTaskId)}: {ContinueWithTaskId}, {nameof(IsAsync)}: {IsAsync}";
}

public sealed class TaskWaitStopEvent : TaskWaitEvent;

public abstract class TaskExecuteEvent : TaskEvent;

public sealed class TaskExecuteStartEvent : TaskExecuteEvent;

public sealed class TaskExecuteStopEvent : TaskExecuteEvent;

public static class TaskEventExtensions
{
  public static TaskEvent? ToTaskEvent(this EventRecordWithMetadata eventRecord)
  {
    if (eventRecord.IsTaskWaitSendEvent() is { } taskData)
    {
      return new TaskWaitSendEvent
      {
        TaskId = taskData.TaskId,
        OriginatingTaskId = taskData.OriginatingTaskId,
        ContinueWithTaskId = taskData.ContinueWithTaskId,
        IsAsync = taskData.IsAsync
      };
    }

    if (eventRecord.IsTaskWaitStopEvent(out var taskId, out var originatingTaskId))
    {
      return new TaskWaitStopEvent { TaskId = taskId, OriginatingTaskId = originatingTaskId };
    }

    if (eventRecord.IsTaskExecuteStartEvent(out taskId, out originatingTaskId))
    {
      return new TaskExecuteStartEvent
      {
        TaskId = taskId,
        OriginatingTaskId = originatingTaskId
      };
    }

    if (eventRecord.IsTaskExecuteStopEvent(out taskId, out originatingTaskId))
    {
      return new TaskExecuteStopEvent
      {
        TaskId = taskId,
        OriginatingTaskId = originatingTaskId
      };
    }

    if (eventRecord.IsTaskRelatedEvent())
    {
      return new UnknownTaskEvent
      {
        TaskId = 0,
        OriginatingTaskId = 0
      };
    }

    return null;
  }
}