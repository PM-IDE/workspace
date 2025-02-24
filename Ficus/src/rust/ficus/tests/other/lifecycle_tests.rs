use std::str::FromStr;

use ficus::event_log::core::event::lifecycle::{braf_lifecycle::XesBrafLifecycle, standard_lifecycle::XesStandardLifecycle};

#[test]
pub fn test_xes_braf_lifecycle_constants() {
  assert_eq!(XesBrafLifecycle::Unspecified, XesBrafLifecycle::from_str("Unspecified").unwrap());
  assert_eq!(XesBrafLifecycle::Closed, XesBrafLifecycle::from_str("Closed").unwrap());
  assert_eq!(
    XesBrafLifecycle::ClosedCancelled,
    XesBrafLifecycle::from_str("Closed.Cancelled").unwrap()
  );
  assert_eq!(
    XesBrafLifecycle::ClosedCancelledAborted,
    XesBrafLifecycle::from_str("Closed.Cancelled.Aborted").unwrap()
  );
  assert_eq!(
    XesBrafLifecycle::ClosedCancelledError,
    XesBrafLifecycle::from_str("Closed.Cancelled.Error").unwrap()
  );
  assert_eq!(
    XesBrafLifecycle::ClosedCancelledExited,
    XesBrafLifecycle::from_str("Closed.Cancelled.Exited").unwrap()
  );
  assert_eq!(
    XesBrafLifecycle::ClosedCancelledObsolete,
    XesBrafLifecycle::from_str("Closed.Cancelled.Obsolete").unwrap()
  );
  assert_eq!(
    XesBrafLifecycle::ClosedCancelledTerminated,
    XesBrafLifecycle::from_str("Closed.Cancelled.Terminated").unwrap()
  );
  assert_eq!(XesBrafLifecycle::Completed, XesBrafLifecycle::from_str("Completed").unwrap());
  assert_eq!(
    XesBrafLifecycle::CompletedFailed,
    XesBrafLifecycle::from_str("Completed.Failed").unwrap()
  );
  assert_eq!(
    XesBrafLifecycle::CompletedSuccess,
    XesBrafLifecycle::from_str("Completed.Success").unwrap()
  );
  assert_eq!(XesBrafLifecycle::Open, XesBrafLifecycle::from_str("Open").unwrap());
  assert_eq!(
    XesBrafLifecycle::OpenNotRunning,
    XesBrafLifecycle::from_str("Open.NotRunning").unwrap()
  );
  assert_eq!(
    XesBrafLifecycle::OpenNotRunningAssigned,
    XesBrafLifecycle::from_str("Open.NotRunning.Assigned").unwrap()
  );
  assert_eq!(
    XesBrafLifecycle::OpenNotRunningReserved,
    XesBrafLifecycle::from_str("Open.NotRunning.Reserved").unwrap()
  );
  assert_eq!(
    XesBrafLifecycle::OpenNotRunningSuspendedAssigned,
    XesBrafLifecycle::from_str("Open.NotRunning.Suspended.Assigned").unwrap()
  );
  assert_eq!(
    XesBrafLifecycle::OpenNotRunningSuspendedReserved,
    XesBrafLifecycle::from_str("Open.NotRunning.Suspended.Reserved").unwrap()
  );
  assert_eq!(
    XesBrafLifecycle::OpenRunningInProgress,
    XesBrafLifecycle::from_str("Open.Running.InProgress").unwrap()
  );
  assert_eq!(
    XesBrafLifecycle::OpenRunningSuspended,
    XesBrafLifecycle::from_str("Open.Running.Suspended").unwrap()
  );
}

#[test]
pub fn test_xes_standard_lifecycle_constants() {
  assert_eq!(
    XesStandardLifecycle::Unspecified,
    XesStandardLifecycle::from_str("unspecified").unwrap()
  );
  assert_eq!(XesStandardLifecycle::Assign, XesStandardLifecycle::from_str("assign").unwrap());
  assert_eq!(XesStandardLifecycle::AteAbort, XesStandardLifecycle::from_str("ate_abort").unwrap());
  assert_eq!(XesStandardLifecycle::Autoskip, XesStandardLifecycle::from_str("autoskip").unwrap());
  assert_eq!(XesStandardLifecycle::Complete, XesStandardLifecycle::from_str("complete").unwrap());
  assert_eq!(
    XesStandardLifecycle::ManualSkip,
    XesStandardLifecycle::from_str("manualskip").unwrap()
  );
  assert_eq!(XesStandardLifecycle::PiAbort, XesStandardLifecycle::from_str("pi_abort").unwrap());
  assert_eq!(XesStandardLifecycle::ReAssign, XesStandardLifecycle::from_str("reassign").unwrap());
  assert_eq!(XesStandardLifecycle::Resume, XesStandardLifecycle::from_str("resume").unwrap());
  assert_eq!(XesStandardLifecycle::Schedule, XesStandardLifecycle::from_str("schedule").unwrap());
  assert_eq!(XesStandardLifecycle::Start, XesStandardLifecycle::from_str("start").unwrap());
  assert_eq!(XesStandardLifecycle::Suspend, XesStandardLifecycle::from_str("suspend").unwrap());
  assert_eq!(XesStandardLifecycle::Unknown, XesStandardLifecycle::from_str("unknown").unwrap());
  assert_eq!(XesStandardLifecycle::Withdraw, XesStandardLifecycle::from_str("withdraw").unwrap());
}
