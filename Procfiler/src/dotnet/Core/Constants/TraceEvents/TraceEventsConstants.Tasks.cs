namespace Core.Constants.TraceEvents;

public static partial class TraceEventsConstants
{
  public const string TaskCommonPrefix = "Task";
  public const string AwaitCommonPrefix = "Await";

  public const string TaskExecuteStart = "TaskExecute/Start";
  public const string TaskExecuteStop = "TaskExecute/Stop";
  public const string TaskWaitSend = "TaskWait/Send";
  public const string TaskWaitStop = "TaskWait/Stop";
  public const string TaskScheduledSend = "TaskScheduled/Send";
  public const string TaskWaitContinuationStarted = "TaskWaitContinuationStarted";
  public const string TaskWaitContinuationComplete = "TaskWaitContinuationComplete";
  public const string AwaitTaskContinuationScheduledSend = "AwaitTaskContinuationScheduled/Send";
  public const string IncompleteAsyncMethod = "IncompleteAsyncMethod";
  public const string ThreadPoolDequeueWork = "ThreadPoolDequeueWork";
  public const string ThreadPoolEnqueueWork = "ThreadPoolEnqueueWork";

  public const string TaskId = "TaskID";
  public const string ContinueWithTaskId = "ContinuationId";
  public const string OriginatingTaskId = "OriginatingTaskID";
  public const string OriginatingTaskSchedulerId = "OriginatingTaskSchedulerID";
}