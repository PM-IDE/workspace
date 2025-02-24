use std::{collections::HashMap, ops::Deref, str::FromStr};

use once_cell::sync::Lazy;

use crate::utils::hash_map_utils::reverse_map;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum XesBrafLifecycle {
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

const UNSPECIFIED: &'static str = "Unspecified";
const CLOSED: &'static str = "Closed";
const CLOSED_CANCELLED: &'static str = "Closed.Cancelled";
const CLOSED_CANCELLED_ABORTED: &'static str = "Closed.Cancelled.Aborted";
const CLOSED_CANCELLED_ERROR: &'static str = "Closed.Cancelled.Error";
const CLOSED_CANCELLED_EXITED: &'static str = "Closed.Cancelled.Exited";
const CLOSED_CANCELLED_OBSOLETE: &'static str = "Closed.Cancelled.Obsolete";
const CLOSED_CANCELLED_TERMINATED: &'static str = "Closed.Cancelled.Terminated";
const COMPLETED: &'static str = "Completed";
const COMPLETED_FAILED: &'static str = "Completed.Failed";
const COMPLETED_SUCCESS: &'static str = "Completed.Success";
const OPEN: &'static str = "Open";
const OPEN_NOTRUNNING: &'static str = "Open.NotRunning";
const OPEN_NOTRUNNING_ASSIGNED: &'static str = "Open.NotRunning.Assigned";
const OPEN_NOTRUNNING_RESERVED: &'static str = "Open.NotRunning.Reserved";
const OPEN_NOTRUNNING_SUSPENDED_ASSIGNED: &'static str = "Open.NotRunning.Suspended.Assigned";
const OPEN_NOTRUNNING_SUSPENDED_RESERVED: &'static str = "Open.NotRunning.Suspended.Reserved";
const OPEN_RUNNING: &'static str = "Open.Running";
const OPEN_RUNNING_INPROGRESS: &'static str = "Open.Running.InProgress";
const OPEN_RUNNING_SUSPENDED: &'static str = "Open.Running.Suspended";

static strings_to_lifecycle: Lazy<HashMap<&'static str, XesBrafLifecycle>> = Lazy::new(|| {
  HashMap::from_iter(vec![
    (UNSPECIFIED, XesBrafLifecycle::Unspecified),
    (CLOSED, XesBrafLifecycle::Closed),
    (CLOSED_CANCELLED, XesBrafLifecycle::ClosedCancelled),
    (CLOSED_CANCELLED_ABORTED, XesBrafLifecycle::ClosedCancelledAborted),
    (CLOSED_CANCELLED_ERROR, XesBrafLifecycle::ClosedCancelledError),
    (CLOSED_CANCELLED_EXITED, XesBrafLifecycle::ClosedCancelledExited),
    (CLOSED_CANCELLED_OBSOLETE, XesBrafLifecycle::ClosedCancelledObsolete),
    (CLOSED_CANCELLED_TERMINATED, XesBrafLifecycle::ClosedCancelledTerminated),
    (COMPLETED, XesBrafLifecycle::Completed),
    (COMPLETED_FAILED, XesBrafLifecycle::CompletedFailed),
    (COMPLETED_SUCCESS, XesBrafLifecycle::CompletedSuccess),
    (OPEN, XesBrafLifecycle::Open),
    (OPEN_NOTRUNNING, XesBrafLifecycle::OpenNotRunning),
    (OPEN_NOTRUNNING_ASSIGNED, XesBrafLifecycle::OpenNotRunningAssigned),
    (OPEN_NOTRUNNING_RESERVED, XesBrafLifecycle::OpenNotRunningReserved),
    (
      OPEN_NOTRUNNING_SUSPENDED_ASSIGNED,
      XesBrafLifecycle::OpenNotRunningSuspendedAssigned,
    ),
    (
      OPEN_NOTRUNNING_SUSPENDED_RESERVED,
      XesBrafLifecycle::OpenNotRunningSuspendedReserved,
    ),
    (OPEN_RUNNING, XesBrafLifecycle::OpenRunning),
    (OPEN_RUNNING_INPROGRESS, XesBrafLifecycle::OpenRunningInProgress),
    (OPEN_RUNNING_SUSPENDED, XesBrafLifecycle::OpenRunningSuspended),
  ])
});

static lifecycle_to_strings: Lazy<HashMap<XesBrafLifecycle, &'static str>> = Lazy::new(|| reverse_map(strings_to_lifecycle.deref()));

impl ToString for XesBrafLifecycle {
  fn to_string(&self) -> String {
    lifecycle_to_strings.get(&self).unwrap().to_string()
  }
}

impl FromStr for XesBrafLifecycle {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if let Some(lifecycle) = strings_to_lifecycle.get(s) {
      Ok(*lifecycle)
    } else {
      Err(())
    }
  }
}
