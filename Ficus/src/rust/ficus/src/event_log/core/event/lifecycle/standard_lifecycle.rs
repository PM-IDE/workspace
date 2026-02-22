use once_cell::sync::Lazy;
use std::{collections::HashMap, fmt::Display, ops::Deref, str::FromStr};

use crate::utils::hash_map_utils::reverse_map;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum XesStandardLifecycle {
  Unspecified = 0,
  Assign = 1,
  AteAbort = 2,
  Autoskip = 3,
  Complete = 4,
  ManualSkip = 5,
  PiAbort = 6,
  ReAssign = 7,
  Resume = 8,
  Schedule = 9,
  Start = 10,
  Suspend = 11,
  Unknown = 12,
  Withdraw = 13,
}

const SCHEDULE: &str = "schedule";
const START: &str = "start";
const COMPLETE: &str = "complete";
const UNKNOWN: &str = "unknown";
const UNSPECIFIED: &str = "unspecified";
const ASSIGN: &str = "assign";
const ATE_ABORT: &str = "ate_abort";
const AUTOSKIP: &str = "autoskip";
const MANUAL_SKIP: &str = "manualskip";
const PI_ABORT: &str = "pi_abort";
const RE_ASSIGN: &str = "reassign";
const RESUME: &str = "resume";
const SUSPEND: &str = "suspend";
const WITHDRAW: &str = "withdraw";

static STRINGS_TO_LIFECYCLE: Lazy<HashMap<&'static str, XesStandardLifecycle>> = Lazy::new(|| {
  HashMap::from_iter(vec![
    (SCHEDULE, XesStandardLifecycle::Schedule),
    (START, XesStandardLifecycle::Start),
    (COMPLETE, XesStandardLifecycle::Complete),
    (UNKNOWN, XesStandardLifecycle::Unknown),
    (UNSPECIFIED, XesStandardLifecycle::Unspecified),
    (ASSIGN, XesStandardLifecycle::Assign),
    (ATE_ABORT, XesStandardLifecycle::AteAbort),
    (AUTOSKIP, XesStandardLifecycle::Autoskip),
    (MANUAL_SKIP, XesStandardLifecycle::ManualSkip),
    (PI_ABORT, XesStandardLifecycle::PiAbort),
    (RE_ASSIGN, XesStandardLifecycle::ReAssign),
    (RESUME, XesStandardLifecycle::Resume),
    (SUSPEND, XesStandardLifecycle::Suspend),
    (WITHDRAW, XesStandardLifecycle::Withdraw),
  ])
});

static LIFECYCLE_TO_STRINGS: Lazy<HashMap<XesStandardLifecycle, &'static str>> = Lazy::new(|| reverse_map(STRINGS_TO_LIFECYCLE.deref()));

impl Display for XesStandardLifecycle {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", LIFECYCLE_TO_STRINGS.get(self).unwrap())
  }
}

impl FromStr for XesStandardLifecycle {
  type Err = ();

  fn from_str(s: &str) -> Result<XesStandardLifecycle, Self::Err> {
    if let Some(lifecycle) = STRINGS_TO_LIFECYCLE.get(s) {
      Ok(*lifecycle)
    } else {
      Err(())
    }
  }
}
