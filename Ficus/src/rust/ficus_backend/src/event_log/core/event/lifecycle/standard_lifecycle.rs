use std::{collections::HashMap, ops::Deref, str::FromStr};

use once_cell::sync::Lazy;

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

const SCHEDULE: &'static str = "schedule";
const START: &'static str = "start";
const COMPLETE: &'static str = "complete";
const UNKNOWN: &'static str = "unknown";
const UNSPECIFIED: &'static str = "unspecified";
const ASSIGN: &'static str = "assign";
const ATE_ABORT: &'static str = "ate_abort";
const AUTOSKIP: &'static str = "autoskip";
const MANUAL_SKIP: &'static str = "manualskip";
const PI_ABORT: &'static str = "pi_abort";
const RE_ASSIGN: &'static str = "reassign";
const RESUME: &'static str = "resume";
const SUSPEND: &'static str = "suspend";
const WITHDRAW: &'static str = "withdraw";

static strings_to_lifecycle: Lazy<HashMap<&'static str, XesStandardLifecycle>> = Lazy::new(|| {
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

static lifecycle_to_strings: Lazy<HashMap<XesStandardLifecycle, &'static str>> = Lazy::new(|| reverse_map(strings_to_lifecycle.deref()));

impl ToString for XesStandardLifecycle {
    fn to_string(&self) -> String {
        lifecycle_to_strings.get(&self).unwrap().to_string()
    }
}

impl FromStr for XesStandardLifecycle {
    type Err = ();

    fn from_str(s: &str) -> Result<XesStandardLifecycle, Self::Err> {
        if let Some(lifecycle) = strings_to_lifecycle.get(s) {
            Ok(*lifecycle)
        } else {
            Err(())
        }
    }
}
