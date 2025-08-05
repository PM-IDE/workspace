use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{
  activity_instances::{self, create_new_log_from_activities_instances, extract_activities_instances, ActivityInTraceInfo},
  contexts::{ActivitiesDiscoveryContext, ActivitiesInstancesDiscoveryContext, PatternsDiscoveryContext},
  repeat_sets::{build_repeat_set_tree_from_repeats, build_repeat_sets, ActivityNode, SubArrayWithTraceIndex},
  repeats::{find_maximal_repeats, find_near_super_maximal_repeats, find_super_maximal_repeats},
  tandem_arrays::{find_maximal_tandem_arrays, find_primitive_tandem_arrays, SubArrayInTraceInfo},
};
use crate::event_log::core::event_log::EventLog;
use crate::features::analysis::patterns::activity_instances::{extract_activities_instances_strict, ActivitiesLogSource};
use crate::features::analysis::patterns::pattern_info::UnderlyingPatternKind;

#[derive(Clone, Copy)]
pub enum PatternsKind {
  PrimitiveTandemArrays(usize),
  MaximalTandemArrays(usize),

  MaximalRepeats,
  SuperMaximalRepeats,
  NearSuperMaximalRepeats,
}

pub fn find_patterns<TClassExtractor, TLog>(context: &PatternsDiscoveryContext<TClassExtractor, TLog>) -> Vec<Vec<SubArrayInTraceInfo>>
where
  TLog: EventLog,
  TClassExtractor: Fn(&TLog::TEvent) -> u64,
{
  let log = context.get_processed_log();
  match &context.pattern_kind {
    PatternsKind::MaximalRepeats => find_maximal_repeats(log, &context.strategy),
    PatternsKind::SuperMaximalRepeats => find_super_maximal_repeats(log, &context.strategy),
    PatternsKind::NearSuperMaximalRepeats => find_near_super_maximal_repeats(log, &context.strategy),
    PatternsKind::PrimitiveTandemArrays(length) => find_primitive_tandem_arrays(log, *length, false),
    PatternsKind::MaximalTandemArrays(length) => find_maximal_tandem_arrays(log, *length, false),
  }
}

pub fn find_repeats<TClassExtractor, TLog>(context: &PatternsDiscoveryContext<TClassExtractor, TLog>) -> Vec<SubArrayWithTraceIndex>
where
  TLog: EventLog,
  TClassExtractor: Fn(&TLog::TEvent) -> u64,
{
  let patterns = find_patterns(context);
  build_repeat_sets(context.get_processed_log(), &patterns)
}

pub fn build_repeat_set_tree<TClassExtractor, TLog, TNameCreator>(
  activities_context: &ActivitiesDiscoveryContext<TClassExtractor, TLog, TNameCreator>,
) -> Vec<Rc<RefCell<ActivityNode>>>
where
  TLog: EventLog,
  TClassExtractor: Fn(&TLog::TEvent) -> u64,
  TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
  let repeats = find_repeats(&activities_context.patterns_context);
  build_repeat_set_tree_from_repeats(
    activities_context.patterns_context.get_processed_log(),
    &repeats,
    activities_context.activity_level,
    UnderlyingPatternKind::from(activities_context.patterns_context.pattern_kind),
    &activities_context.name_creator,
  )
}

pub fn discover_activities_instances<TClassExtractor, TLog, TNameCreator>(
  activities_context: &ActivitiesDiscoveryContext<TClassExtractor, TLog, TNameCreator>,
) -> Vec<Vec<ActivityInTraceInfo>>
where
  TLog: EventLog,
  TClassExtractor: Fn(&TLog::TEvent) -> u64,
  TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
  let mut repeat_set_tree = build_repeat_set_tree(activities_context);

  match activities_context.extract_activities_strict {
    true => extract_activities_instances_strict(activities_context.patterns_context.get_processed_log(), &repeat_set_tree),
    false => extract_activities_instances(
      activities_context.patterns_context.get_processed_log(),
      &mut repeat_set_tree,
      &activities_context.narrow_kind,
      activities_context.min_events_in_activity,
      &activities_context.activity_filter_kind,
    ),
  }
}

pub fn discover_activities_and_create_new_log<TClassExtractor, TLog, TNameCreator, TEvtFactory>(
  context: &ActivitiesInstancesDiscoveryContext<TClassExtractor, TLog, TNameCreator, TEvtFactory>,
) -> TLog
where
  TLog: EventLog,
  TLog::TEvent: 'static,
  TClassExtractor: Fn(&TLog::TEvent) -> u64,
  TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
  TEvtFactory: Fn(&ActivityInTraceInfo, &[Rc<RefCell<TLog::TEvent>>]) -> Rc<RefCell<TLog::TEvent>>,
{
  let activity_instances = discover_activities_instances(&context.activities_context);

  create_new_log_from_activities_instances(
    &context.activities_context.patterns_context.log.borrow(),
    &activity_instances,
    &context.undef_events_handling_strategy,
    &context.high_level_event_factory,
  )
}

pub fn create_logs_for_activities<TClassExtractor, TLog, TNameCreator>(
  context: &ActivitiesDiscoveryContext<TClassExtractor, TLog, TNameCreator>,
  activity_level: usize,
) -> HashMap<String, Rc<RefCell<TLog>>>
where
  TLog: EventLog,
  TLog::TEvent: 'static,
  TClassExtractor: Fn(&TLog::TEvent) -> u64,
  TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
  let activity_instances = discover_activities_instances(&context);

  activity_instances::create_logs_for_activities(&ActivitiesLogSource::TracesActivities(
    &context.patterns_context.log.borrow(),
    &activity_instances,
    activity_level,
  ))
}
