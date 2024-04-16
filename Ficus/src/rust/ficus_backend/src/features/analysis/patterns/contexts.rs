use std::rc::Rc;
use std::{cell::RefCell, str::FromStr};

use crate::event_log::core::{event_log::EventLog, trace::trace::Trace};
use crate::features::analysis::patterns::activity_instances::{ActivityInTraceFilterKind, ActivityNarrowingKind};

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
                processed_trace.push((&class_extractor)(&event.borrow()));
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

pub struct ActivitiesDiscoveryContext<TClassExtractor, TLog, TNameCreator>
where
    TLog: EventLog,
    TClassExtractor: Fn(&TLog::TEvent) -> u64,
    TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
    pub patterns_context: PatternsDiscoveryContext<TClassExtractor, TLog>,
    pub activity_level: usize,
    pub name_creator: TNameCreator,
    pub min_events_in_activity: usize,
    pub narrow_kind: ActivityNarrowingKind,
    pub activity_filter_kind: ActivityInTraceFilterKind,
}

impl<TClassExtractor, TLog, TNameCreator> ActivitiesDiscoveryContext<TClassExtractor, TLog, TNameCreator>
where
    TLog: EventLog,
    TClassExtractor: Fn(&TLog::TEvent) -> u64,
    TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
    pub fn new(
        patterns_context: PatternsDiscoveryContext<TClassExtractor, TLog>,
        activity_level: usize,
        min_events_in_activity: usize,
        narrow_kind: ActivityNarrowingKind,
        activity_filter_kind: ActivityInTraceFilterKind,
        name_creator: TNameCreator,
    ) -> Self {
        Self {
            patterns_context,
            activity_level,
            name_creator,
            min_events_in_activity,
            narrow_kind,
            activity_filter_kind,
        }
    }
}

pub struct ActivitiesInstancesDiscoveryContext<TClassExtractor, TLog, TNameCreator, TEvtFactory>
where
    TLog: EventLog,
    TClassExtractor: Fn(&TLog::TEvent) -> u64,
    TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
    TEvtFactory: Fn(&ActivityInTraceInfo) -> Rc<RefCell<TLog::TEvent>>,
{
    pub activities_context: ActivitiesDiscoveryContext<TClassExtractor, TLog, TNameCreator>,
    pub undef_events_handling_strategy: UndefActivityHandlingStrategy<TLog::TEvent>,
    pub high_level_event_factory: TEvtFactory,
}

impl<TClassExtractor, TLog, TNameCreator, TEvtFactory> ActivitiesInstancesDiscoveryContext<TClassExtractor, TLog, TNameCreator, TEvtFactory>
where
    TLog: EventLog,
    TClassExtractor: Fn(&TLog::TEvent) -> u64,
    TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
    TEvtFactory: Fn(&ActivityInTraceInfo) -> Rc<RefCell<TLog::TEvent>>,
{
    pub fn new(
        activities_context: ActivitiesDiscoveryContext<TClassExtractor, TLog, TNameCreator>,
        strategy: UndefActivityHandlingStrategy<TLog::TEvent>,
        high_level_event_factory: TEvtFactory,
    ) -> Self {
        Self {
            activities_context,
            undef_events_handling_strategy: strategy,
            high_level_event_factory,
        }
    }
}
