use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
  event_log::xes::xes_event_log::XesEventLogImpl,
  features::analysis::patterns::{
    activity_instances::ActivityInTraceInfo,
    repeat_sets::{ActivityNode, SubArrayWithTraceIndex},
    tandem_arrays::SubArrayInTraceInfo,
  },
};

pub type TracesActivities = Vec<Vec<ActivityInTraceInfo>>;
pub type Activities = Vec<Rc<RefCell<ActivityNode>>>;
pub type RepeatSets = Vec<SubArrayWithTraceIndex>;
pub type Patterns = Vec<Vec<SubArrayInTraceInfo>>;
pub type ActivitiesToLogs = HashMap<String, XesEventLogImpl>;
