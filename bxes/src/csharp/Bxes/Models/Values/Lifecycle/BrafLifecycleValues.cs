namespace Bxes.Models.Values.Lifecycle;

public enum BrafLifecycleValues : byte
{
  Unspecified = 0,
  Closed = 1,
  ClosedCancelled = 2,
  ClosedCancelledAborted = 3,
  ClosedCancelledError = 4,
  ClosedCancelledExited = 5,
  ClosedCancelledObsolete = 6,
  ClosedCancelledTerminated = 7,
  Completed = 8,
  CompletedFailed = 9,
  CompletedSuccess = 10,
  Open = 11,
  OpenNotRunning = 12,
  OpenNotRunningAssigned = 13,
  OpenNotRunningReserved = 14,
  OpenNotRunningSuspendedAssigned = 15,
  OpenNotRunningSuspendedReserved = 16,
  OpenRunning = 17,
  OpenRunningInProgress = 18,
  OpenRunningSuspended = 19,
}

public static class BrafLifecycleValuesUtil 
{
  public static BrafLifecycleValues? TryParse(string value) => value switch
  {
    "Unspecified" => BrafLifecycleValues.Unspecified,
    "Closed" => BrafLifecycleValues.Closed,
    "Closed.Cancelled" => BrafLifecycleValues.ClosedCancelled,
    "Closed.Cancelled.Aborted" => BrafLifecycleValues.ClosedCancelledAborted,
    "Closed.Cancelled.Error" => BrafLifecycleValues.ClosedCancelledError,
    "Closed.Cancelled.Exited" => BrafLifecycleValues.ClosedCancelledExited,
    "Closed.Cancelled.Obsolete" => BrafLifecycleValues.ClosedCancelledObsolete,
    "Closed.Cancelled.Terminated" => BrafLifecycleValues.ClosedCancelledTerminated,
    "Completed" => BrafLifecycleValues.Completed,
    "Completed.Failed" => BrafLifecycleValues.CompletedFailed,
    "Completed.Success" => BrafLifecycleValues.CompletedSuccess,
    "Open" => BrafLifecycleValues.Open,
    "Open.NotRunning" => BrafLifecycleValues.OpenNotRunning,
    "Open.NotRunning.Assigned" => BrafLifecycleValues.OpenNotRunningAssigned,
    "Open.NotRunning.Reserved" => BrafLifecycleValues.OpenNotRunningReserved,
    "Open.NotRunning.Suspended.Assigned" => BrafLifecycleValues.OpenNotRunningSuspendedAssigned,
    "Open.NotRunning.Suspended.Reserved" => BrafLifecycleValues.OpenNotRunningSuspendedReserved,
    "Open.Running" => BrafLifecycleValues.OpenRunning,
    "Open.Running.InProgress" => BrafLifecycleValues.OpenRunningInProgress,
    "Open.Running.Suspended" => BrafLifecycleValues.OpenRunningSuspended,
    _ => null
  };
}