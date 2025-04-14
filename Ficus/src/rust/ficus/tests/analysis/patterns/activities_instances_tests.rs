use crate::test_core::simple_events_logs_provider::{create_log_from_taxonomy_of_patterns, create_maximal_repeats_log};
use ficus::event_log::core::event::event::Event;
use ficus::event_log::xes::xes_event::XesEventImpl;
use ficus::event_log::xes::xes_event_log::XesEventLogImpl;
use ficus::features::analysis::patterns::activity_instances::{ActivityInTraceFilterKind, ActivityNarrowingKind};
use ficus::{
  event_log::core::{event::event_hasher::default_class_extractor, event_log::EventLog},
  features::analysis::patterns::{
    activity_instances::{create_activity_name, ActivityInTraceInfo, UndefActivityHandlingStrategy, UNDEF_ACTIVITY_NAME},
    contexts::{ActivitiesDiscoveryContext, ActivitiesInstancesDiscoveryContext, PatternsDiscoveryContext, PatternsDiscoveryStrategy},
    entry_points::{create_logs_for_activities, discover_activities_and_create_new_log, discover_activities_instances, PatternsKind},
  },
  vecs,
};
use std::{cell::RefCell, ops::Deref, rc::Rc};

#[test]
fn test_activity_instances() {
  let log = Rc::new(RefCell::new(create_log_from_taxonomy_of_patterns()));

  let patterns_context = PatternsDiscoveryContext::new(
    Rc::clone(&log),
    PatternsKind::PrimitiveTandemArrays(20),
    PatternsDiscoveryStrategy::FromAllTraces,
    default_class_extractor,
  );

  let context = ActivitiesDiscoveryContext::new(
    patterns_context,
    0,
    0,
    ActivityNarrowingKind::NarrowDown,
    ActivityInTraceFilterKind::DefaultFilter,
    |sub_array| create_activity_name(log.borrow().deref(), sub_array, None),
  );

  let activities = discover_activities_instances(&context);
  let activities = dump_activities(&activities);

  assert_eq!(activities, [[(2, 15), (17, 19)]]);
}

fn dump_activities(instances: &Vec<Vec<ActivityInTraceInfo>>) -> Vec<Vec<(usize, usize)>> {
  instances
    .into_iter()
    .map(|trace_instances| trace_instances.into_iter().map(|instance| instance.dump()).collect())
    .collect()
}

#[test]
fn test_activity_instances1() {
  let log = Rc::new(RefCell::new(create_maximal_repeats_log()));

  let patterns_context = PatternsDiscoveryContext::new(
    Rc::clone(&log),
    PatternsKind::PrimitiveTandemArrays(20),
    PatternsDiscoveryStrategy::FromAllTraces,
    default_class_extractor,
  );

  let context = ActivitiesDiscoveryContext::new(
    patterns_context,
    0,
    0,
    ActivityNarrowingKind::NarrowDown,
    ActivityInTraceFilterKind::DefaultFilter,
    |sub_array| create_activity_name(log.borrow().deref(), sub_array, None),
  );

  let activities = discover_activities_instances(&context);

  let activities = dump_activities(&activities);
  assert_eq!(
    activities,
    vec![
      vec![(0, 10)],
      vec![(0, 10)],
      vec![(0, 12)],
      vec![(0, 10)],
      vec![(0, 9), (10, 19), (20, 23)]
    ]
  );
}

#[test]
fn test_creating_new_log_from_activity_instances_insert_all_events() {
  execute_activities_discovery_test(
    create_log_from_taxonomy_of_patterns(),
    UndefActivityHandlingStrategy::<XesEventImpl>::InsertAllEvents,
    &vec![vec!["g", "d", "(a)::(b)::(c)", "f", "i", "(a)::(b)::(c)"]],
  );
}

fn execute_activities_discovery_test(
  log: XesEventLogImpl,
  strategy: UndefActivityHandlingStrategy<XesEventImpl>,
  expected: &Vec<Vec<&str>>,
) {
  let log = Rc::new(RefCell::new(log));

  let patterns_context = PatternsDiscoveryContext::new(
    Rc::clone(&log),
    PatternsKind::PrimitiveTandemArrays(20),
    PatternsDiscoveryStrategy::FromAllTraces,
    default_class_extractor,
  );

  let context = ActivitiesDiscoveryContext::new(
    patterns_context,
    0,
    0,
    ActivityNarrowingKind::NarrowDown,
    ActivityInTraceFilterKind::DefaultFilter,
    |sub_array| create_activity_name(log.borrow().deref(), sub_array, None),
  );

  let context = ActivitiesInstancesDiscoveryContext::new(context, strategy, |info, _| {
    Rc::new(RefCell::new(XesEventImpl::new_with_min_date(info.node().borrow().name().to_string())))
  });

  let new_log = discover_activities_and_create_new_log(&context);

  assert_eq!(new_log.to_raw_vector(), *expected);
}

#[test]
fn test_creating_new_log_from_activity_instances_insert_as_single_event() {
  execute_activities_discovery_test(
    create_log_from_taxonomy_of_patterns(),
    UndefActivityHandlingStrategy::InsertAsSingleEvent(Box::new(|| {
      Rc::new(RefCell::new(XesEventImpl::new_with_min_date(UNDEF_ACTIVITY_NAME.to_string())))
    })),
    &vec![vec![UNDEF_ACTIVITY_NAME, "(a)::(b)::(c)", UNDEF_ACTIVITY_NAME, "(a)::(b)::(c)"]],
  );
}

#[test]
fn test_creating_new_log_from_activity_instances_dont_insert() {
  execute_activities_discovery_test(
    create_log_from_taxonomy_of_patterns(),
    UndefActivityHandlingStrategy::<XesEventImpl>::DontInsert,
    &vec![vec!["(a)::(b)::(c)", "(a)::(b)::(c)"]],
  );
}

#[test]
fn test_creating_log_for_activities() {
  execute_activities_logs_creation_test(
    create_log_from_taxonomy_of_patterns(),
    PatternsKind::PrimitiveTandemArrays(20),
    vec![(
      "(a)::(b)::(c)".to_owned(),
      vec![
        vecs!["a", "b", "c", "a", "b", "c", "a", "b", "c", "a", "b", "c", "a"],
        vecs!["c", "a"],
      ],
    )],
  )
}

#[test]
fn test_creating_log_for_activities1() {
  execute_activities_logs_creation_test(
    create_maximal_repeats_log(),
    PatternsKind::MaximalRepeats,
    vec![
      ("(b)::(c)::(d)".to_owned(), vec![vecs!["b", "d", "c"]]),
      (
        "(d)::(a)::(b)::(c)".to_owned(),
        vec![
          vecs!["a", "a", "b", "c", "d", "b", "b", "c", "d", "a"],
          vecs!["d", "a", "b", "c", "d", "a", "b", "c", "b", "b"],
          vecs!["b", "b", "b", "c", "d", "b", "b", "b", "c", "c", "a", "a"],
          vecs!["a", "a", "a", "d", "a", "b", "b", "c", "c", "c"],
          vecs!["a", "a", "a", "c", "d", "c", "d", "c", "b"],
          vecs!["d", "b", "c", "c", "b", "a", "d", "b", "d"],
        ],
      ),
      ("(e)".to_owned(), vec![vecs!["e"], vecs!["e"]]),
    ],
  )
}

#[test]
fn test_creating_log_for_activities2() {
  execute_activities_logs_creation_test(
    create_maximal_repeats_log(),
    PatternsKind::NearSuperMaximalRepeats,
    vec![
      ("(b)::(c)::(d)".to_owned(), vec![vecs!["b", "d", "c"]]),
      (
        "(d)::(a)::(b)::(c)".to_owned(),
        vec![
          vecs!["a", "a", "b", "c", "d", "b", "b", "c", "d", "a"],
          vecs!["d", "a", "b", "c", "d", "a", "b", "c", "b", "b"],
          vecs!["b", "b", "b", "c", "d", "b", "b", "b", "c", "c", "a", "a"],
          vecs!["a", "a", "a", "d", "a", "b", "b", "c", "c", "c"],
          vecs!["a", "a", "a", "c", "d", "c", "d", "c", "b"],
          vecs!["d", "b", "c", "c", "b", "a", "d", "b", "d"],
        ],
      ),
      ("(e)".to_owned(), vec![vecs!["e"], vecs!["e"]]),
    ],
  )
}

#[test]
fn test_creating_log_for_activities3() {
  execute_activities_logs_creation_test(
    create_maximal_repeats_log(),
    PatternsKind::PrimitiveTandemArrays(20),
    vec![(
      "(d)::(a)::(b)::(c)".to_owned(),
      vec![
        vecs!["a", "a", "b", "c", "d", "b", "b", "c", "d", "a"],
        vecs!["d", "a", "b", "c", "d", "a", "b", "c", "b", "b"],
        vecs!["b", "b", "b", "c", "d", "b", "b", "b", "c", "c", "a", "a"],
        vecs!["a", "a", "a", "d", "a", "b", "b", "c", "c", "c"],
        vecs!["a", "a", "a", "c", "d", "c", "d", "c", "b"],
        vecs!["d", "b", "c", "c", "b", "a", "d", "b", "d"],
        vecs!["b", "d", "c"],
      ],
    )],
  )
}

#[test]
fn test_creating_log_for_activities4() {
  execute_activities_logs_creation_test(
    create_maximal_repeats_log(),
    PatternsKind::MaximalTandemArrays(20),
    vec![(
      "(d)::(a)::(b)::(c)".to_owned(),
      vec![
        vecs!["a", "a", "b", "c", "d", "b", "b", "c", "d", "a"],
        vecs!["d", "a", "b", "c", "d", "a", "b", "c", "b", "b"],
        vecs!["b", "b", "b", "c", "d", "b", "b", "b", "c", "c", "a", "a"],
        vecs!["a", "a", "a", "d", "a", "b", "b", "c", "c", "c"],
        vecs!["a", "a", "a", "c", "d", "c", "d", "c", "b"],
        vecs!["d", "b", "c", "c", "b", "a", "d", "b", "d"],
        vecs!["b", "d", "c"],
      ],
    )],
  )
}

fn execute_activities_logs_creation_test(log: XesEventLogImpl, pattern_kind: PatternsKind, expected: Vec<(String, Vec<Vec<String>>)>) {
  let log = Rc::new(RefCell::new(log));

  let patterns_context = PatternsDiscoveryContext::new(
    Rc::clone(&log),
    pattern_kind,
    PatternsDiscoveryStrategy::FromAllTraces,
    default_class_extractor,
  );

  let context = ActivitiesDiscoveryContext::new(
    patterns_context,
    0,
    0,
    ActivityNarrowingKind::NarrowDown,
    ActivityInTraceFilterKind::DefaultFilter,
    |sub_array| create_activity_name(log.borrow().deref(), sub_array, None),
  );

  let activities_logs = create_logs_for_activities(&context, 0);
  let mut activities_logs = activities_logs
    .iter()
    .map(|pair| (pair.0.to_owned(), pair.1.borrow().to_raw_vector()))
    .collect::<Vec<(String, Vec<Vec<String>>)>>();

  activities_logs.sort_by(|first, second| first.0.cmp(&second.0));
  assert_eq!(activities_logs, expected);
}
