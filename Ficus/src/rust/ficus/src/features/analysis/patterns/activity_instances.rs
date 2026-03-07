use super::repeat_sets::{ActivityNode, SubArrayWithTraceIndex};
use crate::{
  context_key,
  event_log::{
    core::{
      event::event::{Event, EventPayloadValue},
      event_log::EventLog,
      trace::trace::Trace,
    },
    xes::{xes_event::XesEventImpl, xes_event_log::XesEventLogImpl, xes_trace::XesTraceImpl},
  },
  features::analysis::patterns::pattern_info::UNDERLYING_PATTERN_KIND_KEY,
  pipelines::aliases::TracesActivities,
  utils::user_data::user_data::{UserData, UserDataOwner},
};
use derive_new::new;
use fancy_regex::Regex;
use getset::{Getters, MutGetters};
use lazy_static::lazy_static;
use std::{
  cell::RefCell,
  collections::{HashMap, HashSet, VecDeque},
  ops::DerefMut,
  rc::Rc,
  str::FromStr,
};

#[derive(Debug, Clone, Getters, MutGetters, new)]
pub struct ActivityInTraceInfo {
  #[getset(get = "pub")]
  node: Rc<RefCell<ActivityNode>>,
  #[getset(get = "pub")]
  start_pos: usize,
  #[getset(get = "pub", get_mut = "pub")]
  length: usize,
}

pub const UNATTACHED_SUB_TRACE_NAME: &str = "UndefinedActivity";

pub enum SubTraceKind<'a> {
  Attached(&'a ActivityInTraceInfo),
  Unattached(usize, usize),
}

#[derive(PartialOrd, PartialEq, Copy, Clone)]
pub enum ActivityNarrowingKind {
  DontNarrow,
  StayTheSame,
  NarrowUp,
  NarrowDown,
}

impl FromStr for ActivityNarrowingKind {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "DontNarrow" => Ok(ActivityNarrowingKind::DontNarrow),
      "StayTheSame" => Ok(ActivityNarrowingKind::StayTheSame),
      "NarrowUp" => Ok(ActivityNarrowingKind::NarrowUp),
      "NarrowDown" => Ok(ActivityNarrowingKind::NarrowDown),
      _ => Err(()),
    }
  }
}

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum ActivityInTraceFilterKind {
  NoFilter,
  DefaultFilter,
}

impl FromStr for ActivityInTraceFilterKind {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "NoFilter" => Ok(ActivityInTraceFilterKind::NoFilter),
      "DefaultFilter" => Ok(ActivityInTraceFilterKind::DefaultFilter),
      _ => Err(()),
    }
  }
}

impl ActivityInTraceInfo {
  pub fn dump(&self) -> (usize, usize) {
    (self.start_pos, self.start_pos + self.length)
  }
}

pub fn extract_activities_instances_strict(
  log: &Vec<Vec<u64>>,
  activities: &Vec<Rc<RefCell<ActivityNode>>>,
) -> Vec<Vec<ActivityInTraceInfo>> {
  let mut result = vec![];

  let mut suitable_activities = get_all_child_activities(activities)
    .into_iter()
    .filter_map(|a| a.borrow().repeat_set().as_ref().map(|_| a.clone()))
    .collect::<Vec<Rc<RefCell<ActivityNode>>>>();

  suitable_activities.sort_by_key(|a| std::cmp::Reverse(a.borrow().repeat_set().unwrap().len()));

  for trace in log {
    let mut index = 0;
    let mut trace_instances = vec![];

    'this_loop: loop {
      if index >= trace.len() {
        break;
      }

      for suitable_activity in &suitable_activities {
        let activity = suitable_activity.borrow();
        let repeat_set = activity.repeat_set().as_ref().unwrap();
        let sub_array = repeat_set.sub_array;

        if index + sub_array.length >= trace.len() {
          continue;
        }

        let repeat_set_slice = &log[repeat_set.trace_index][sub_array.start_index..sub_array.start_index + sub_array.length];

        let mut found_pattern = true;
        for i in 0..repeat_set_slice.len() {
          if repeat_set_slice[i] != trace[index + i] {
            found_pattern = false;
            break;
          }
        }

        if found_pattern {
          trace_instances.push(ActivityInTraceInfo::new(
            (*suitable_activity).clone(),
            index,
            repeat_set_slice.len(),
          ));
          index += repeat_set_slice.len();
          continue 'this_loop;
        }
      }

      index += 1;
    }

    result.push(trace_instances);
  }

  result
}

fn get_all_child_activities(activities: &[Rc<RefCell<ActivityNode>>]) -> Vec<Rc<RefCell<ActivityNode>>> {
  let mut result = vec![];

  let mut queue = VecDeque::from_iter(activities.iter().cloned());
  while !queue.is_empty() {
    let current = queue.pop_front().unwrap();

    for child in current.borrow().children() {
      queue.push_back(child.clone());
    }

    result.push(current);
  }

  result
}

pub fn extract_activities_instances(
  log: &Vec<Vec<u64>>,
  activities: &mut [Rc<RefCell<ActivityNode>>],
  narrow_kind: &ActivityNarrowingKind,
  min_events_in_activity: usize,
  filtering_kind: &ActivityInTraceFilterKind,
) -> Vec<Vec<ActivityInTraceInfo>> {
  let activities_by_size = split_activities_nodes_by_size(activities);
  let mut result = vec![];

  for trace in log {
    let mut trace_activities = vec![];
    let mut index = None;
    let mut current_activity = None;
    let mut last_activity_start_index = None;
    let mut current_event_classes = HashSet::new();

    while index.is_none() || index.unwrap() < trace.len() {
      if let Some(index) = &mut index {
        *index += 1;
      } else {
        index = Some(0);
      }

      if index.unwrap() >= trace.len() {
        break;
      }

      let event_hash = trace[index.unwrap()];
      if current_activity.is_none() {
        let mut found_activity = false;
        for activities in activities_by_size.iter() {
          for activity in activities {
            if activity.borrow().event_classes().contains(&event_hash) {
              current_activity = Some(Rc::clone(activity));
              last_activity_start_index = Some(index.unwrap());
              found_activity = true;
              break;
            }
          }

          if found_activity {
            current_event_classes.clear();
            current_event_classes.insert(event_hash);
            break;
          }
        }

        continue;
      }

      if !current_activity.as_ref().unwrap().borrow().event_classes().contains(&event_hash) {
        let mut new_set = current_event_classes.clone();
        new_set.insert(event_hash);

        let mut found_new_set = false;
        for activities_set in activities_by_size.iter() {
          if activities_set.is_empty() || activities_set[0].borrow().len() < current_activity.as_ref().unwrap().borrow().len() {
            continue;
          }

          for activity in activities_set {
            if new_set.is_subset(activity.borrow().event_classes()) {
              current_activity = Some(Rc::clone(activity));
              found_new_set = true;
              break;
            }
          }

          if found_new_set {
            current_event_classes.insert(event_hash);
            break;
          }
        }

        if !found_new_set {
          let activity = narrow_activity(current_activity.as_ref().unwrap(), &current_event_classes, narrow_kind);

          current_activity = Some(activity);

          let activity_instance = ActivityInTraceInfo {
            node: Rc::clone(current_activity.as_ref().unwrap()),
            start_pos: last_activity_start_index.unwrap(),
            length: index.unwrap() - last_activity_start_index.unwrap(),
          };

          if is_suitable_activity_instance(&activity_instance, min_events_in_activity, filtering_kind) {
            trace_activities.push(activity_instance);
          }

          current_activity = None;
          current_event_classes.clear();
          last_activity_start_index = None;
          *index.as_mut().unwrap() -= 1;
        }
      } else {
        current_event_classes.insert(event_hash);
      }
    }

    if let Some(last_activity_start_index) = last_activity_start_index {
      let activity = narrow_activity(current_activity.as_ref().unwrap(), &current_event_classes, narrow_kind);
      current_activity = Some(activity);

      let activity_instance = ActivityInTraceInfo {
        node: Rc::clone(current_activity.as_ref().unwrap()),
        start_pos: last_activity_start_index,
        length: index.unwrap() - last_activity_start_index,
      };

      if is_suitable_activity_instance(&activity_instance, min_events_in_activity, filtering_kind) {
        trace_activities.push(activity_instance);
      }
    }

    result.push(trace_activities);
  }

  result
}

fn is_suitable_activity_instance(
  instance: &ActivityInTraceInfo,
  min_events_in_activity: usize,
  filtering_kind: &ActivityInTraceFilterKind,
) -> bool {
  if filtering_kind == &ActivityInTraceFilterKind::NoFilter {
    true
  } else if instance.node.borrow().len() < min_events_in_activity {
    false
  } else {
    instance.length > instance.node.borrow().len() / 2
  }
}

fn split_activities_nodes_by_size(activities: &mut [Rc<RefCell<ActivityNode>>]) -> Vec<Vec<Rc<RefCell<ActivityNode>>>> {
  if activities.is_empty() {
    return vec![];
  }

  activities.sort_by_key(|first| first.borrow().len());
  let mut current_length = activities[0].borrow().len();
  let mut result = vec![vec![Rc::clone(activities.first().unwrap())]];

  for activity in activities.iter() {
    if activity.borrow().len() != current_length {
      result.push(vec![]);
      current_length = activity.borrow().len();
    }

    result.last_mut().unwrap().push(Rc::clone(activity));
  }

  for i in 0..result.len() {
    result
      .get_mut(i)
      .unwrap()
      .sort_by(|first, second| first.borrow().name().cmp(second.borrow().name()));
  }

  result
}

fn narrow_activity(
  node_ptr: &Rc<RefCell<ActivityNode>>,
  activities_set: &HashSet<u64>,
  narrow_kind: &ActivityNarrowingKind,
) -> Rc<RefCell<ActivityNode>> {
  if narrow_kind == &ActivityNarrowingKind::DontNarrow || narrow_kind == &ActivityNarrowingKind::StayTheSame {
    return node_ptr.clone();
  }

  let mut q = VecDeque::new();
  let node = node_ptr.borrow();
  for child in node.children() {
    q.push_back(Rc::clone(child));
  }

  let mut result = vec![];
  while !q.is_empty() {
    let current_activity_ptr = q.pop_front().unwrap();
    let current_activity = current_activity_ptr.borrow();

    if current_activity.event_classes().is_superset(activities_set) {
      result.push(Rc::clone(&current_activity_ptr));
      for child_node in current_activity.children() {
        q.push_back(Rc::clone(child_node));
      }
    }
  }

  if result.is_empty() {
    return Rc::clone(node_ptr);
  }

  let result = result.iter();
  let result = match narrow_kind {
    ActivityNarrowingKind::NarrowUp => result.min_by(|first, second| first.borrow().len().cmp(&second.borrow().len())),
    ActivityNarrowingKind::NarrowDown => result.min_by(|first, second| first.borrow().len().cmp(&second.borrow().len())),
    _ => panic!("Should not be reached"),
  };

  Rc::clone(result.unwrap())
}

pub fn process_activities_in_trace<TUndefActivityHandleFunc, TActivityHandleFunc>(
  trace_length: usize,
  activities_instances: &Vec<ActivityInTraceInfo>,
  mut undefined_activity_func: TUndefActivityHandleFunc,
  mut activity_func: TActivityHandleFunc,
) where
  TUndefActivityHandleFunc: FnMut(usize, usize),
  TActivityHandleFunc: FnMut(&ActivityInTraceInfo),
{
  let mut index = 0;
  for instance in activities_instances {
    if index < instance.start_pos {
      undefined_activity_func(index, instance.start_pos);
    }

    activity_func(instance);
    index = instance.start_pos + instance.length;
  }

  if index < trace_length {
    undefined_activity_func(index, trace_length);
  }
}

pub enum UndefActivityHandlingStrategy<TEvent> {
  DontInsert,
  InsertAsSingleEvent(Box<dyn Fn() -> Rc<RefCell<TEvent>>>),
  InsertAllEvents,
}

#[derive(PartialEq, Clone, Copy)]
pub enum AdjustingMode {
  FromAllLog,
  FromUnattachedSubTraces,
}

impl FromStr for AdjustingMode {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "FromAllLog" => Ok(AdjustingMode::FromAllLog),
      "FromUnattachedSubTraces" => Ok(AdjustingMode::FromUnattachedSubTraces),
      _ => Err(()),
    }
  }
}

pub const UNDEF_ACTIVITY_NAME: &str = "UNDEFINED_ACTIVITY";

pub struct UnderlyingEventsInfo<T> {
  base_pattern: Option<Vec<Rc<RefCell<T>>>>,
  underlying_events: Vec<Rc<RefCell<T>>>,
}

impl<T> UnderlyingEventsInfo<T> {
  pub fn new(base_pattern: Option<Vec<Rc<RefCell<T>>>>, underlying_events: Vec<Rc<RefCell<T>>>) -> Self {
    Self {
      underlying_events,
      base_pattern,
    }
  }

  pub fn base_pattern(&self) -> Option<&Vec<Rc<RefCell<T>>>> {
    self.base_pattern.as_ref()
  }

  pub fn underlying_events(&self) -> &Vec<Rc<RefCell<T>>> {
    &self.underlying_events
  }
}

const HIERARCHY_LEVEL: &str = "HIERARCHY_LEVEL";
const UNDERLYING_EVENTS: &str = "UNDERLYING_EVENTS";

context_key! { HIERARCHY_LEVEL, usize }
context_key! { UNDERLYING_EVENTS, UnderlyingEventsInfo<XesEventImpl> }

pub fn create_new_log_from_activities_instances<TEventFactory>(
  log: &XesEventLogImpl,
  instances: &[Vec<ActivityInTraceInfo>],
  strategy: &UndefActivityHandlingStrategy<XesEventImpl>,
  event_from_activity_factory: &TEventFactory,
) -> XesEventLogImpl
where
  TEventFactory: Fn(&ActivityInTraceInfo, &[Rc<RefCell<XesEventImpl>>]) -> Rc<RefCell<XesEventImpl>>,
{
  let level = log.user_data().get(HIERARCHY_LEVEL_KEY.key()).unwrap_or(&0usize);
  let mut new_log = XesEventLogImpl::default();

  for (instances, trace) in instances.iter().zip(log.traces()) {
    let trace = trace.borrow();
    let new_trace_ptr = Rc::new(RefCell::new(XesTraceImpl::default()));

    let undef_activity_func = |start_index: usize, end_index: usize| match strategy {
      UndefActivityHandlingStrategy::DontInsert => (),
      UndefActivityHandlingStrategy::InsertAsSingleEvent(factory) => {
        new_trace_ptr.borrow_mut().push(factory());
      }
      UndefActivityHandlingStrategy::InsertAllEvents => {
        for i in start_index..end_index {
          let event = trace.events()[i].borrow().clone();
          new_trace_ptr.borrow_mut().push(Rc::new(RefCell::new(event)));
        }
      }
    };

    let activity_func = |activity: &ActivityInTraceInfo| {
      let instance_events = &trace.events()[activity.start_pos..activity.start_pos + activity.length];

      let ptr = event_from_activity_factory(activity, instance_events);

      new_trace_ptr.borrow_mut().push(Rc::clone(&ptr));

      let mut underlying_events = vec![];
      for i in activity.start_pos..(activity.start_pos + activity.length) {
        underlying_events.push(Rc::clone(&trace.events()[i]));
      }

      let mut event = ptr.borrow_mut();
      let user_data = event.user_data_mut();

      for event in &underlying_events {
        execute_with_underlying_events(event, &mut |event| {
          let payload_value = EventPayloadValue::String(activity.node.borrow().id().clone());
          let key = Rc::from(format!("hierarchy_level_{}", level));
          event.add_or_update_payload(key, payload_value);
        })
      }

      user_data.put_concrete(UNDERLYING_PATTERN_KIND_KEY.key(), *activity.node.borrow().pattern_kind());

      let base_pattern = if let Some(repeat_set) = activity.node.borrow().repeat_set() {
        let trace = log.traces().get(repeat_set.trace_index).unwrap();
        let sub_array = repeat_set.sub_array;
        Some(trace.borrow().events()[sub_array.start_index..sub_array.start_index + sub_array.length].to_vec())
      } else {
        None
      };

      let info = UnderlyingEventsInfo::new(base_pattern, underlying_events);

      user_data.put_concrete(UNDERLYING_EVENTS_KEY.key(), info);
    };

    process_activities_in_trace(trace.events().len(), instances, undef_activity_func, activity_func);

    new_log.push(new_trace_ptr)
  }

  new_log.user_data_mut().put_concrete(HIERARCHY_LEVEL_KEY.key(), level + 1);
  new_log
}

pub fn add_unattached_activities(
  log: &Vec<Vec<u64>>,
  activities: &mut [Rc<RefCell<ActivityNode>>],
  existing_instances: &[Vec<ActivityInTraceInfo>],
  min_numbers_of_events: usize,
  should_narrow: &ActivityNarrowingKind,
  min_events_in_activity: usize,
  activity_filter_kind: &ActivityInTraceFilterKind,
) -> Vec<Vec<ActivityInTraceInfo>> {
  let mut new_activities = vec![];

  for (trace_activities, trace) in existing_instances.iter().zip(log) {
    let mut new_trace_activities = vec![];

    let handle_unattached_events = |start_index: usize, end_index: usize| {
      if end_index - start_index < min_numbers_of_events {
        return;
      }

      let activities = extract_activities_instances(
        &vec![trace[start_index..end_index].to_vec()],
        activities,
        should_narrow,
        min_events_in_activity,
        activity_filter_kind,
      );

      new_trace_activities.extend(
        activities[0]
          .iter()
          .map(|instance| ActivityInTraceInfo {
            node: Rc::clone(&instance.node),
            start_pos: start_index + instance.start_pos,
            length: instance.length,
          })
          .collect::<Vec<ActivityInTraceInfo>>(),
      );
    };

    let length = trace.len();
    process_activities_in_trace(length, trace_activities, handle_unattached_events, |_| {});

    new_trace_activities.extend(trace_activities.iter().cloned());
    new_trace_activities.sort_by(|first, second| first.start_pos.cmp(&second.start_pos));

    new_activities.push(new_trace_activities);
  }

  new_activities
}

pub enum ActivitiesLogSource<'a, TLog>
where
  TLog: EventLog,
{
  Log(&'a TLog),
  TracesActivities(&'a TLog, &'a Vec<Vec<ActivityInTraceInfo>>, usize),
}

pub fn create_logs_for_activities(
  activities_source: &ActivitiesLogSource<XesEventLogImpl>,
) -> HashMap<Rc<str>, Rc<RefCell<XesEventLogImpl>>> {
  match activities_source {
    ActivitiesLogSource::Log(log) => create_activities_logs_from_log(log),
    ActivitiesLogSource::TracesActivities(log, activities, level) => create_log_from_traces_activities(log, activities, *level),
  }
}

fn create_activities_logs_from_log(log: &XesEventLogImpl) -> HashMap<Rc<str>, Rc<RefCell<XesEventLogImpl>>> {
  let mut activities_to_logs: HashMap<Rc<str>, Rc<RefCell<XesEventLogImpl>>> = HashMap::new();

  for trace in log.traces() {
    for event in trace.borrow().events() {
      if event
        .borrow_mut()
        .user_data_mut()
        .concrete_mut(UNDERLYING_EVENTS_KEY.key())
        .is_some()
      {
        let name = event.borrow().name_pointer().clone();
        let mut new_trace = XesTraceImpl::default();
        substitute_underlying_events(event, &mut new_trace);

        if let Some(existing_log) = activities_to_logs.get_mut(name.as_ref()) {
          existing_log.borrow_mut().push(Rc::new(RefCell::new(new_trace)));
        } else {
          let mut new_log = XesEventLogImpl::default();
          new_log.push(Rc::new(RefCell::new(new_trace)));
          activities_to_logs.insert(name, Rc::new(RefCell::new(new_log)));
        }
      }
    }
  }

  activities_to_logs
}

fn create_log_from_traces_activities<TLog: EventLog>(
  log: &TLog,
  activities: &[Vec<ActivityInTraceInfo>],
  activity_level: usize,
) -> HashMap<Rc<str>, Rc<RefCell<TLog>>> {
  let mut activities_to_logs: HashMap<Rc<str>, Rc<RefCell<TLog>>> = HashMap::new();
  for (trace_activities, trace) in activities.iter().zip(log.traces()) {
    let activity_handler = |activity_info: &ActivityInTraceInfo| {
      if activity_level != *activity_info.node.borrow().level() {
        return;
      }

      let new_trace_ptr = Rc::new(RefCell::new(TLog::TTrace::default()));
      let mut new_trace = new_trace_ptr.borrow_mut();

      let start = activity_info.start_pos;
      let end = start + activity_info.length;

      let trace = trace.borrow();
      let events = trace.events();

      for event in events.iter().take(end).skip(start) {
        new_trace.push(Rc::new(RefCell::new(event.borrow().clone())));
      }

      let name = activity_info.node.borrow().name().clone();
      if let Some(activity_log) = activities_to_logs.get_mut(name.as_ref()) {
        activity_log.borrow_mut().push(Rc::clone(&new_trace_ptr));
      } else {
        let log = Rc::new(RefCell::new(TLog::default()));
        log.borrow_mut().push(Rc::clone(&new_trace_ptr));

        activities_to_logs.insert(name, log);
      }
    };

    let length = trace.borrow().events().len();
    process_activities_in_trace(length, trace_activities, |_, _| {}, activity_handler);
  }

  activities_to_logs
}

pub fn create_activity_name<TLog>(log: &TLog, sub_array: &SubArrayWithTraceIndex, class_extractor: Option<&str>) -> String
where
  TLog: EventLog,
{
  let mut name = String::new();

  let left = sub_array.sub_array.start_index;
  let right = left + sub_array.sub_array.length;
  let trace = log.traces().get(sub_array.trace_index).unwrap().borrow();
  let events = trace.events();

  let regex = class_extractor.map(|extractor| Regex::new(extractor).unwrap());

  for (index, event) in events.iter().enumerate().take(right).skip(left) {
    name.push('(');

    let event = event.borrow();
    let event_name = event.name();

    let event_name = match regex.as_ref() {
      Some(regex) => match regex.find(event_name) {
        Ok(Some(m)) => {
          if m.start() == 0 {
            &event_name[0..m.end()]
          } else {
            event_name
          }
        }
        _ => event_name,
      },
      None => event_name,
    };

    name.push_str(event_name);
    name.push(')');

    if index != right - 1 {
      name.push_str("::");
    }
  }

  name
}

pub fn count_underlying_events(log: &XesEventLogImpl) -> usize {
  let mut count = 0usize;
  for trace in log.traces() {
    let mut trace_count = 0usize;
    for event in trace.borrow().events() {
      trace_count += count_underlying_events_for_event(event.borrow_mut().deref_mut());
    }

    count += trace_count;
  }

  count
}

fn count_underlying_events_for_event(event: &mut XesEventImpl) -> usize {
  if let Some(underlying_events) = event.user_data_mut().concrete_mut(UNDERLYING_EVENTS_KEY.key()) {
    let mut result = 0usize;
    for underlying_event in underlying_events.underlying_events() {
      result += count_underlying_events_for_event(underlying_event.borrow_mut().deref_mut())
    }

    result
  } else {
    1
  }
}

pub fn create_log_from_unattached_events<TLog>(log: &TLog, activities: &TracesActivities) -> TLog
where
  TLog: EventLog,
{
  let mut new_log = TLog::default();

  for (trace, trace_activities) in log.traces().iter().zip(activities) {
    let trace = trace.borrow();
    let mut new_trace = TLog::TTrace::default();

    let process_undef_activity = |start, end| {
      for event in &trace.events()[start..end] {
        new_trace.push(event.clone());
      }
    };

    process_activities_in_trace(trace.events().len(), trace_activities, process_undef_activity, |_| {});

    new_log.push(Rc::new(RefCell::new(new_trace)));
  }

  new_log
}

pub fn execute_with_underlying_events(event: &Rc<RefCell<XesEventImpl>>, action: &mut impl FnMut(&mut XesEventImpl)) {
  let mut event = event.borrow_mut();
  if let Some(underlying_events) = event.user_data_mut().concrete_mut(UNDERLYING_EVENTS_KEY.key()) {
    for underlying_event in underlying_events.underlying_events() {
      execute_with_underlying_events(underlying_event, action);
    }
  } else {
    action(&mut event);
  }
}

pub fn substitute_underlying_events(event: &Rc<RefCell<XesEventImpl>>, trace: &mut XesTraceImpl) {
  if let Some(underlying_events) = event.borrow_mut().user_data_mut().concrete(UNDERLYING_EVENTS_KEY.key()) {
    for underlying_event in underlying_events.underlying_events() {
      substitute_underlying_events(underlying_event, trace);
    }
  } else {
    trace.push(event.clone());
  }
}

pub fn create_vector_of_underlying_events(event: &Rc<RefCell<XesEventImpl>>) -> Vec<Rc<RefCell<XesEventImpl>>> {
  let mut result = vec![];
  create_vector_of_underlying_events_intenral(event, &mut result);

  result
}

pub fn try_get_base_pattern(event: &Rc<RefCell<XesEventImpl>>) -> Option<Vec<Rc<RefCell<XesEventImpl>>>> {
  if let Some(info) = event.borrow().user_data().concrete(UNDERLYING_EVENTS_KEY.key()) {
    info.base_pattern().cloned()
  } else {
    None
  }
}

fn create_vector_of_underlying_events_intenral(event: &Rc<RefCell<XesEventImpl>>, result: &mut Vec<Rc<RefCell<XesEventImpl>>>) {
  if let Some(underlying_events) = event.borrow_mut().user_data_mut().concrete(UNDERLYING_EVENTS_KEY.key()) {
    for underlying_event in underlying_events.underlying_events() {
      create_vector_of_underlying_events_intenral(underlying_event, result);
    }
  } else {
    result.push(event.clone());
  }
}

pub fn create_vector_of_immediate_underlying_events(event: &Rc<RefCell<XesEventImpl>>) -> Vec<Rc<RefCell<XesEventImpl>>> {
  let mut events = vec![];

  if let Some(underlying_events) = event.borrow_mut().user_data_mut().concrete(UNDERLYING_EVENTS_KEY.key()) {
    for underlying_event in underlying_events.underlying_events() {
      events.push(underlying_event.clone());
    }
  } else {
    events.push(event.clone());
  }

  events
}
