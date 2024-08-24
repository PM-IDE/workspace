namespace Core.Constants.TraceEvents;

public static partial class TraceEventsConstants
{
  public const string ThreadPoolWorkerThreadStart = "ThreadPoolWorkerThread/Start";
  public const string ThreadPoolWorkerThreadStop = "ThreadPoolWorkerThread/Stop";
  public const string ThreadPoolWorkerThreadWait = "ThreadPoolWorkerThread/Wait";

  public const string ThreadPoolWorkerThreadRetirementStart = "ThreadPoolWorkerThreadRetirement/Start";
  public const string ThreadPoolWorkerThreadRetirementStop = "ThreadPoolWorkerThreadRetirement/Stop";
  public const string ThreadPoolWorkerThreadAdjustmentSample = "ThreadPoolWorkerThreadAdjustment/Sample";
  public const string ThreadPoolWorkerThreadAdjustmentAdjustment = "ThreadPoolWorkerThreadAdjustment/Adjustment";
  public const string ThreadPoolWorkerThreadAdjustmentStats = "ThreadPoolWorkerThreadAdjustment/Stats";
  public const string IoThreadCreate = "IOThread/Create";
  public const string IoThreadTerminate = "IOThread/Terminate";
  public const string IoThreadRetire = "IOThread/Retire";
  public const string IoThreadUnRetire = "IOThread/Unretire";
}