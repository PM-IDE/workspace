use super::repeat_sets::{ActivityNode, SubArrayWithTraceIndex};
use crate::event_log::core::event::event::EventPayloadValue;
use crate::pipelines::keys::context_key::{ContextKey, DefaultContextKey};
use crate::{
    event_log::core::{event::event::Event, event_log::EventLog, trace::trace::Trace},
    pipelines::aliases::TracesActivities,
    utils::user_data::{keys::DefaultKey, user_data::UserData},
};
use fancy_regex::Regex;
use lazy_static::lazy_static;
use once_cell::unsync::Lazy;
use std::any::{Any, TypeId};
use std::borrow::ToOwned;
use std::sync::Mutex;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    ops::DerefMut,
    rc::Rc,
    str::FromStr,
};

#[derive(Debug, Clone)]
pub struct ActivityInTraceInfo {
    pub node: Rc<RefCell<ActivityNode>>,
    pub start_pos: usize,
    pub length: usize,
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

pub fn extract_activities_instances(
    log: &Vec<Vec<u64>>,
    activities: &mut Vec<Rc<RefCell<ActivityNode>>>,
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
            if index.is_none() {
                index = Some(0);
            } else {
                *index.as_mut().unwrap() += 1;
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
                    if activities_set.len() == 0 || activities_set[0].borrow().len() < current_activity.as_ref().unwrap().borrow().len() {
                        continue;
                    }

                    for activity in activities_set {
                        if new_set.is_subset(&activity.borrow().event_classes()) {
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

        if last_activity_start_index.is_some() {
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
    } else {
        if instance.node.borrow().len() < min_events_in_activity {
            false
        } else {
            instance.length > instance.node.borrow().len() / 2
        }
    }
}

fn split_activities_nodes_by_size(activities: &mut Vec<Rc<RefCell<ActivityNode>>>) -> Vec<Vec<Rc<RefCell<ActivityNode>>>> {
    if activities.is_empty() {
        return vec![];
    }

    activities.sort_by(|first, second| first.borrow().len().cmp(&second.borrow().len()));
    let mut current_length = activities[0].borrow().len();
    let mut result = vec![vec![Rc::clone(activities.get(0).unwrap())]];

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
            .sort_by(|first, second| first.borrow().name().cmp(&second.borrow().name()));
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

        if current_activity.event_classes().is_superset(&activities_set) {
            result.push(Rc::clone(&current_activity_ptr));
            for child_node in current_activity.children() {
                q.push_back(Rc::clone(child_node));
            }
        }
    }

    if result.is_empty() {
        return Rc::clone(&node_ptr);
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
    TUndefActivityHandleFunc: FnMut(usize, usize) -> (),
    TActivityHandleFunc: FnMut(&ActivityInTraceInfo) -> (),
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

pub struct ActivityInstancesKeys {
    underlying_events_keys: Mutex<HashMap<TypeId, Box<dyn Any>>>,
}

impl ActivityInstancesKeys {
    pub fn new() -> Self {
        Self {
            underlying_events_keys: Mutex::new(HashMap::new()),
        }
    }

    pub fn underlying_events_key<TEvent: Event + 'static>(&mut self) -> DefaultKey<Vec<Rc<RefCell<TEvent>>>> {
        let type_id = TypeId::of::<TEvent>();
        let mut map = self.underlying_events_keys.lock();
        let map = map.as_mut().ok().unwrap();

        if let Some(key) = map.get(&type_id) {
            key.downcast_ref::<DefaultKey<Vec<Rc<RefCell<TEvent>>>>>().unwrap().clone()
        } else {
            let key = DefaultKey::<Vec<Rc<RefCell<TEvent>>>>::new("UNDERLYING_EVENTS".to_owned());
            map.insert(type_id, Box::new(key) as Box<dyn Any>);
            map.get(&type_id)
                .unwrap()
                .downcast_ref::<DefaultKey<Vec<Rc<RefCell<TEvent>>>>>()
                .unwrap()
                .clone()
        }
    }
}

static mut KEYS: Mutex<Lazy<ActivityInstancesKeys>> = Mutex::new(Lazy::new(|| ActivityInstancesKeys::new()));

lazy_static! {
    pub static ref HIERARCHY_LEVEL: DefaultContextKey<usize> = DefaultContextKey::new("HIERARCHY_LEVEL");
}

pub fn create_new_log_from_activities_instances<TLog, TEventFactory>(
    log: &TLog,
    instances: &Vec<Vec<ActivityInTraceInfo>>,
    strategy: &UndefActivityHandlingStrategy<TLog::TEvent>,
    event_from_activity_factory: &TEventFactory,
) -> TLog
where
    TLog: EventLog,
    TLog::TEvent: 'static,
    TEventFactory: Fn(&ActivityInTraceInfo) -> Rc<RefCell<TLog::TEvent>>,
{
    let level = log.user_data().get(HIERARCHY_LEVEL.key()).unwrap_or(&0usize);
    let mut new_log = TLog::empty();

    for (instances, trace) in instances.iter().zip(log.traces()) {
        let trace = trace.borrow();
        let new_trace_ptr = Rc::new(RefCell::new(TLog::TTrace::empty()));

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
            let ptr = event_from_activity_factory(activity);

            new_trace_ptr.borrow_mut().push(Rc::clone(&ptr));

            let mut underlying_events = vec![];
            for i in activity.start_pos..(activity.start_pos + activity.length) {
                underlying_events.push(Rc::clone(&trace.events()[i]));
            }

            let mut event = ptr.borrow_mut();
            let user_data = event.user_data();

            for event in &underlying_events {
                execute_with_underlying_events::<TLog>(event, &mut |event| {
                    let payload_value = EventPayloadValue::String(activity.node.borrow().id().clone());
                    let key = format!("hierarchy_level_{}", level);
                    event.add_or_update_payload(key, payload_value);
                })
            }

            unsafe {
                user_data.put_any(&KEYS.lock().unwrap().underlying_events_key::<TLog::TEvent>(), underlying_events);
            }
        };

        process_activities_in_trace(trace.events().len(), &instances, undef_activity_func, activity_func);

        new_log.push(new_trace_ptr)
    }

    new_log.user_data_mut().put_concrete(HIERARCHY_LEVEL.key(), level + 1);
    new_log
}

pub fn add_unattached_activities(
    log: &Vec<Vec<u64>>,
    activities: &mut Vec<Rc<RefCell<ActivityNode>>>,
    existing_instances: &Vec<Vec<ActivityInTraceInfo>>,
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

        new_trace_activities.extend(trace_activities.iter().map(|instance| instance.clone()));
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

pub fn create_logs_for_activities<TLog>(activities_source: &ActivitiesLogSource<TLog>) -> HashMap<String, Rc<RefCell<TLog>>>
where
    TLog: EventLog,
{
    match activities_source {
        ActivitiesLogSource::Log(log) => create_activities_logs_from_log(log),
        ActivitiesLogSource::TracesActivities(log, activities, level) => create_log_from_traces_activities(log, activities, *level),
    }
}

fn create_activities_logs_from_log<TLog: EventLog>(log: &TLog) -> HashMap<String, Rc<RefCell<TLog>>> {
    let mut activities_to_logs: HashMap<String, Rc<RefCell<TLog>>> = HashMap::new();
    let key = unsafe { KEYS.lock().unwrap().underlying_events_key::<TLog::TEvent>() };

    for trace in log.traces() {
        for event in trace.borrow().events() {
            if event.borrow_mut().user_data().get::<Vec<Rc<RefCell<TLog::TEvent>>>>(&key).is_some() {
                let name = event.borrow().name().to_owned();
                let mut new_trace = TLog::TTrace::empty();
                substitute_underlying_events::<TLog>(event, &mut new_trace);

                if let Some(existing_log) = activities_to_logs.get_mut(&name) {
                    existing_log.borrow_mut().push(Rc::new(RefCell::new(new_trace)));
                } else {
                    let mut new_log = TLog::empty();
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
    activities: &Vec<Vec<ActivityInTraceInfo>>,
    activity_level: usize,
) -> HashMap<String, Rc<RefCell<TLog>>> {
    let mut activities_to_logs: HashMap<String, Rc<RefCell<TLog>>> = HashMap::new();
    for (trace_activities, trace) in activities.iter().zip(log.traces()) {
        let activity_handler = |activity_info: &ActivityInTraceInfo| {
            if activity_level != activity_info.node.borrow().level() {
                return;
            }

            let new_trace_ptr = Rc::new(RefCell::new(TLog::TTrace::empty()));
            let mut new_trace = new_trace_ptr.borrow_mut();

            let start = activity_info.start_pos;
            let end = start + activity_info.length;

            let trace = trace.borrow();
            let events = trace.events();

            for i in start..end {
                new_trace.push(Rc::new(RefCell::new(events[i].borrow().clone())));
            }

            let name = activity_info.node.borrow().name().as_ref().as_ref().to_owned();
            if let Some(activity_log) = activities_to_logs.get_mut(&name) {
                activity_log.borrow_mut().push(Rc::clone(&new_trace_ptr));
            } else {
                let log = Rc::new(RefCell::new(TLog::empty()));
                log.borrow_mut().push(Rc::clone(&new_trace_ptr));

                activities_to_logs.insert(name, log);
            }
        };

        let length = trace.borrow().events().len();
        process_activities_in_trace(length, trace_activities, |_, _| {}, activity_handler);
    }

    activities_to_logs
}

pub fn create_activity_name<TLog>(log: &TLog, sub_array: &SubArrayWithTraceIndex, class_extractor: Option<&String>) -> String
where
    TLog: EventLog,
{
    let mut name = String::new();

    let left = sub_array.sub_array.start_index;
    let right = left + sub_array.sub_array.length;
    let trace = log.traces().get(sub_array.trace_index).unwrap().borrow();
    let events = trace.events();

    let regex = match class_extractor {
        Some(extractor) => Some(Regex::new(&extractor).unwrap()),
        None => None,
    };

    for index in left..right {
        name.push('(');

        let event_name = events[index].borrow();
        let event_name = event_name.name();
        let event_name = match regex.as_ref() {
            Some(regex) => match regex.find(event_name) {
                Ok(Some(m)) => {
                    if m.start() == 0 {
                        &event_name[0..m.end()]
                    } else {
                        event_name.as_str()
                    }
                }
                _ => event_name,
            },
            None => event_name,
        };

        name.push_str(&event_name);
        name.push(')');

        if index != right - 1 {
            name.push_str("::");
        }
    }

    name
}

pub fn count_underlying_events<TLog>(log: &TLog) -> usize
where
    TLog: EventLog,
{
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

fn count_underlying_events_for_event<TEvent>(event: &mut TEvent) -> usize
where
    TEvent: Event + 'static,
{
    let key = unsafe { KEYS.lock().unwrap().underlying_events_key::<TEvent>() };

    if let Some(underlying_events) = event.user_data().concrete_mut(&key) {
        let mut result = 0usize;
        for underlying_event in underlying_events {
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
    let mut new_log = TLog::empty();

    for (trace, trace_activities) in log.traces().into_iter().zip(activities) {
        let trace = trace.borrow();
        let mut new_trace = TLog::TTrace::empty();

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

pub fn execute_with_underlying_events<TLog>(event: &Rc<RefCell<TLog::TEvent>>, action: &mut impl FnMut(&mut TLog::TEvent))
where
    TLog: EventLog,
{
    let key = unsafe { KEYS.lock().unwrap().underlying_events_key::<TLog::TEvent>() };

    let mut event = event.borrow_mut();
    if let Some(underlying_events) = event.user_data().get::<Vec<Rc<RefCell<TLog::TEvent>>>>(&key) {
        for underlying_event in underlying_events {
            execute_with_underlying_events::<TLog>(underlying_event, action);
        }
    } else {
        action(&mut event);
    }
}

pub fn substitute_underlying_events<TLog>(event: &Rc<RefCell<TLog::TEvent>>, trace: &mut TLog::TTrace)
where
    TLog: EventLog,
{
    let key = unsafe { KEYS.lock().unwrap().underlying_events_key::<TLog::TEvent>() };

    if let Some(underlying_events) = event.borrow_mut().user_data().get::<Vec<Rc<RefCell<TLog::TEvent>>>>(&key) {
        for underlying_event in underlying_events {
            substitute_underlying_events::<TLog>(underlying_event, trace);
        }
    } else {
        trace.push(event.clone());
    }
}

pub fn create_vector_of_underlying_events<TLog: EventLog>(event: &Rc<RefCell<TLog::TEvent>>) -> Vec<Rc<RefCell<TLog::TEvent>>> {
    let mut result = vec![];
    create_vector_of_underlying_events_intenral::<TLog>(event, &mut result);

    result
}

fn create_vector_of_underlying_events_intenral<TLog: EventLog>(
    event: &Rc<RefCell<TLog::TEvent>>,
    result: &mut Vec<Rc<RefCell<TLog::TEvent>>>,
) {
    let key = unsafe { KEYS.lock().unwrap().underlying_events_key::<TLog::TEvent>() };

    if let Some(underlying_events) = event.borrow_mut().user_data().get::<Vec<Rc<RefCell<TLog::TEvent>>>>(&key) {
        for underlying_event in underlying_events {
            create_vector_of_underlying_events_intenral::<TLog>(underlying_event, result);
        }
    } else {
        result.push(event.clone());
    }
}

pub fn create_vector_of_immediate_underlying_events<TLog: EventLog>(event: &Rc<RefCell<TLog::TEvent>>) -> Vec<Rc<RefCell<TLog::TEvent>>> {
    let mut events = vec![];

    let key = unsafe { KEYS.lock().unwrap().underlying_events_key::<TLog::TEvent>() };
    if let Some(underlying_events) = event.borrow_mut().user_data().get::<Vec<Rc<RefCell<TLog::TEvent>>>>(&key) {
        for underlying_event in underlying_events {
            events.push(underlying_event.clone());
        }
    } else {
        events.push(event.clone());
    }

    events
}
