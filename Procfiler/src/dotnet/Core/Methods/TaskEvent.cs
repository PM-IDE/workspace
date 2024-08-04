namespace Core.Methods;

public abstract class TaskEvent
{
  public required int TaskId { get; init; }
  public required int OriginatingTaskId { get; init; }

  public override string ToString() => $"{GetType().Name} TaskId: {TaskId}, OriginatingTaskId: {OriginatingTaskId}";
}

public sealed class TaskWaitSendEvent : TaskEvent
{
  public required int ContinueWithTaskId { get; init; }
  public required bool IsAsync { get; init; }

  public override string ToString() =>
    $"{base.ToString()}, {nameof(ContinueWithTaskId)}: {ContinueWithTaskId}, {nameof(IsAsync)}: {IsAsync}";
}

public sealed class TaskWaitStopEvent : TaskEvent;
