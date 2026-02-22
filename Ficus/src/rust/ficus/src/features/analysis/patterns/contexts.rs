use std::{cell::RefCell, rc::Rc, str::FromStr};

use crate::{
  event_log::{
    core::{event_log::EventLog, trace::trace::Trace},
    xes::{xes_event::XesEventImpl, xes_event_log::XesEventLogImpl},
  },
  features::analysis::patterns::activity_instances::{ActivityInTraceFilterKind, ActivityNarrowingKind},
};

use super::{
  activity_instances::{ActivityInTraceInfo, UndefActivityHandlingStrategy},
  entry_points::PatternsKind,
  repeat_sets::SubArrayWithTraceIndex,
};

#[derive(Clone, Copy)]
pub enum PatternsDiscoveryStrategy {
  FromAllTraces,
  FromSingleMergedTrace,
}

impl FromStr for PatternsDiscoveryStrategy {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "FromAllTraces" => Ok(PatternsDiscoveryStrategy::FromAllTraces),
      "FromSingleMergedTrace" => Ok(PatternsDiscoveryStrategy::FromSingleMergedTrace),
      _ => Err(()),
    }
  }
}

pub struct PatternsDiscoveryContext<TClassExtractor, TLog>
where
  TLog: EventLog,
  TClassExtractor: Fn(&TLog::TEvent) -> u64,
{
  pub log: Rc<RefCell<TLog>>,
  pub pattern_kind: PatternsKind,
  pub class_extractor: TClassExtractor,
  pub strategy: PatternsDiscoveryStrategy,

  processed_log: Vec<Vec<u64>>,
}

impl<TClassExtractor, TLog> PatternsDiscoveryContext<TClassExtractor, TLog>
where
  TLog: EventLog,
  TClassExtractor: Fn(&TLog::TEvent) -> u64,
{
  pub fn get_processed_log(&self) -> &Vec<Vec<u64>> {
    &self.processed_log
  }

  pub fn new(
    log: Rc<RefCell<TLog>>,
    pattern_kind: PatternsKind,
    strategy: PatternsDiscoveryStrategy,
    class_extractor: TClassExtractor,
  ) -> Self {
    let mut processed_log = vec![];
    for trace in log.borrow().traces() {
      let mut processed_trace = vec![];
      for event in trace.borrow().events() {
        processed_trace.push(class_extractor(&event.borrow()));
      }

      processed_log.push(processed_trace);
    }

    Self {
      log,
      pattern_kind,
      class_extractor,
      strategy,
      processed_log,
    }
  }
}

pub struct ActivitiesDiscoveryContext<TClassExtractor, TNameCreator>
where
  TClassExtractor: Fn(&XesEventImpl) -> u64,
  TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
  pub patterns_context: PatternsDiscoveryContext<TClassExtractor, XesEventLogImpl>,
  pub activity_level: usize,
  pub name_creator: TNameCreator,
  pub min_events_in_activity: usize,
  pub narrow_kind: ActivityNarrowingKind,
  pub activity_filter_kind: ActivityInTraceFilterKind,
  pub extract_activities_strict: bool,
}

impl<TClassExtractor, TNameCreator> ActivitiesDiscoveryContext<TClassExtractor, TNameCreator>
where
  TClassExtractor: Fn(&XesEventImpl) -> u64,
  TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
  pub fn new(
    patterns_context: PatternsDiscoveryContext<TClassExtractor, XesEventLogImpl>,
    activity_level: usize,
    min_events_in_activity: usize,
    narrow_kind: ActivityNarrowingKind,
    activity_filter_kind: ActivityInTraceFilterKind,
    name_creator: TNameCreator,
    extract_activities_strict: bool,
  ) -> Self {
    Self {
      patterns_context,
      activity_level,
      name_creator,
      min_events_in_activity,
      narrow_kind,
      activity_filter_kind,
      extract_activities_strict,
    }
  }
}

pub struct ActivitiesInstancesDiscoveryContext<TClassExtractor, TNameCreator, TEvtFactory>
where
  TClassExtractor: Fn(&XesEventImpl) -> u64,
  TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
  TEvtFactory: Fn(&ActivityInTraceInfo, &[Rc<RefCell<XesEventImpl>>]) -> Rc<RefCell<XesEventImpl>>,
{
  pub activities_context: ActivitiesDiscoveryContext<TClassExtractor, TNameCreator>,
  pub undef_events_handling_strategy: UndefActivityHandlingStrategy<XesEventImpl>,
  pub high_level_event_factory: TEvtFactory,
}

impl<TClassExtractor, TNameCreator, TEvtFactory> ActivitiesInstancesDiscoveryContext<TClassExtractor, TNameCreator, TEvtFactory>
where
  TClassExtractor: Fn(&XesEventImpl) -> u64,
  TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
  TEvtFactory: Fn(&ActivityInTraceInfo, &[Rc<RefCell<XesEventImpl>>]) -> Rc<RefCell<XesEventImpl>>,
{
  pub fn new(
    activities_context: ActivitiesDiscoveryContext<TClassExtractor, TNameCreator>,
    strategy: UndefActivityHandlingStrategy<XesEventImpl>,
    high_level_event_factory: TEvtFactory,
  ) -> Self {
    Self {
      activities_context,
      undef_events_handling_strategy: strategy,
      high_level_event_factory,
    }
  }
}
