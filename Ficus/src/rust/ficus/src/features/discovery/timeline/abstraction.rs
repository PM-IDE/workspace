use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::event_log::xes::xes_trace::XesTraceImpl;
use crate::features::discovery::root_sequence::models::{ActivityStartEndTimeData, EventCoordinates, NodeAdditionalDataContainer};
use crate::features::discovery::timeline::discovery::{TraceThread, TraceThreadEvent};
use crate::features::discovery::timeline::utils::{extract_thread_id, get_stamp};
use crate::pipelines::errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError};
use crate::pipelines::keys::context_keys::{SOFTWARE_DATA_KEY, START_END_ACTIVITIES_TIMES_KEY};
use crate::utils::user_data::user_data::{UserData, UserDataOwner};
use getset::Getters;
use log::error;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

pub fn abstract_event_groups(
  event_groups: Vec<Vec<Vec<Rc<RefCell<XesEventImpl>>>>>,
  labels: &Vec<usize>,
  thread_attribute: String,
  time_attribute: Option<String>,
) -> Result<XesEventLogImpl, PipelinePartExecutionError> {
  let mut current_label_index = 0;
  let mut abstracted_log = XesEventLogImpl::empty();

  for (trace_id, trace_groups) in event_groups.iter().enumerate() {
    let mut abstracted_trace = XesTraceImpl::empty();
    for (event_index, event_group) in trace_groups.iter().enumerate() {
      if event_group.is_empty() {
        error!("Encountered empty event group");
        continue;
      }

      let group_label = *labels.get(current_label_index).as_ref().unwrap();
      let abstracted_event = create_abstracted_event(
        &event_group,
        group_label,
        thread_attribute.as_str(),
        time_attribute.as_ref(),
        EventCoordinates::new(trace_id as u64, event_index as u64),
      )?;

      abstracted_trace.push(abstracted_event);
      current_label_index += 1;
    }

    abstracted_log.push(Rc::new(RefCell::new(abstracted_trace)));
  }

  Ok(abstracted_log)
}

#[derive(Clone, Debug, Getters)]
pub struct SoftwareData {
  #[getset(get="pub")] event_classes: HashMap<String, usize>,
  #[getset(get="pub")] thread_diagram_fragment: Vec<TraceThread>,
  #[getset(get="pub")] suspensions: Vec<ExecutionSuspension>,
  #[getset(get="pub")] method_events: Vec<MethodEvent>,
  #[getset(get="pub")] thread_events: Vec<ThreadEvent>,
  #[getset(get="pub")] http_events: Vec<HTTPEvent>,
  #[getset(get="pub")] contention_events: Vec<ContentionEvent>,
  #[getset(get="pub")] exception_events: Vec<ExceptionEvent>,
  #[getset(get="pub")] pool_events: Vec<ArrayPoolEvent>,
  #[getset(get="pub")] socket_events: Vec<SocketEvent>,
}

#[derive(Clone, Debug, Getters)]
pub struct ExecutionSuspension {
  #[getset(get="pub")] start_time: u64,
  #[getset(get="pub")] end_time: u64,
  #[getset(get="pub")] reason: String,
}

impl ExecutionSuspension {
  pub fn new(start_time: u64, end_time: u64, reason: String) -> Self {
    Self {
      start_time,
      end_time,
      reason,
    }
  }
}

#[derive(Clone, Debug)]
pub enum MethodEvent {
  Success(String),
  Failed(String, String),
  Load(String),
  Unload(String),
}

#[derive(Clone, Debug)]
pub enum ThreadEvent {
  Created(u64),
  Terminated(u64),
}

#[derive(Clone, Debug)]
pub enum AssemblyEvent {
  Load(String),
  Unload(String),
}

#[derive(Clone, Debug, Getters)]
pub struct ArrayPoolEvent {
  #[getset(get="pub")] buffer_id: u64,
  #[getset(get="pub")] event_kind: ArrayPoolEventKind,
}

#[derive(Clone, Debug)]
pub enum ArrayPoolEventKind {
  Created,
  Rented,
  Returned,
  Trimmed,
}

#[derive(Clone, Debug, Getters)]
pub struct ExceptionEvent {
  #[getset(get="pub")] exception_type: String,
}

#[derive(Clone, Debug, Getters)]
pub struct HTTPEvent {
  #[getset(get="pub")] host: String,
  #[getset(get="pub")] port: String,
  #[getset(get="pub")] scheme: String,
  #[getset(get="pub")] path: String,
  #[getset(get="pub")] query: String,
}

impl HTTPEvent {
  pub fn new(host: String, port: String, scheme: String, path: String, query: String) -> Self {
    Self {
      host,
      port,
      scheme,
      path,
      query,
    }
  }
}

#[derive(Clone, Debug, Getters)]
pub struct ContentionEvent {
  #[getset(get="pub")] start_time: u64,
  #[getset(get="pub")] end_time: u64,
}

impl ContentionEvent {
  pub fn new(start_time: u64, end_time: u64) -> Self {
    Self {
      start_time,
      end_time,
    }
  }
}

#[derive(Clone, Debug, Getters)]
pub struct SocketEvent {
  #[getset(get="pub")] address: String,
}

impl SoftwareData {
  pub fn empty() -> Self {
    Self {
      event_classes: HashMap::new(),
      thread_diagram_fragment: vec![],
      suspensions: vec![],
      exception_events: vec![],
      http_events: vec![],
      thread_events: vec![],
      method_events: vec![],
      contention_events: vec![],
      pool_events: vec![],
      socket_events: vec![],
    }
  }
}

fn create_abstracted_event(
  event_group: &Vec<Rc<RefCell<XesEventImpl>>>,
  label: &usize,
  thread_attribute: &str,
  time_attribute: Option<&String>,
  event_coordinates: EventCoordinates,
) -> Result<Rc<RefCell<XesEventImpl>>, PipelinePartExecutionError> {
  let mut event_classes = HashMap::new();
  let mut threads = HashMap::new();

  for event in event_group {
    *event_classes.entry(event.borrow().name().clone()).or_insert(0) += 1;

    let thread_id = extract_thread_id(event.borrow().deref(), thread_attribute);
    let stamp = match get_stamp(event.borrow().deref(), time_attribute) {
      Ok(stamp) => stamp,
      Err(_) => return Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new("Failed to get stamp".to_string())))
    };

    threads.entry(thread_id).or_insert(TraceThread::empty()).events_mut().push(TraceThreadEvent::new(event.clone(), stamp))
  }

  let software_data = SoftwareData {
    event_classes,
    thread_diagram_fragment: threads.into_values().collect(),
    suspensions: vec![],
    exception_events: vec![],
    http_events: vec![],
    thread_events: vec![],
    method_events: vec![],
    contention_events: vec![],
    pool_events: vec![],
    socket_events: vec![],
  };

  let first_stamp = event_group.first().unwrap().borrow().timestamp().clone();
  let abstracted_event_stamp = *event_group.last().unwrap().borrow().timestamp() - first_stamp;
  let abstracted_event_stamp = first_stamp + abstracted_event_stamp;

  let label_name = Rc::new(Box::new(label.to_string()));

  let mut event = XesEventImpl::new_all_fields(label_name, abstracted_event_stamp, None);

  let software_data = NodeAdditionalDataContainer::new(software_data, event_coordinates);
  event.user_data_mut().put_concrete(SOFTWARE_DATA_KEY.key(), vec![software_data]);

  let first_stamp = get_stamp(&event_group.first().unwrap().borrow(), time_attribute).map_err(|e| e.into())?;
  let last_stamp = get_stamp(&event_group.last().unwrap().borrow(), time_attribute).map_err(|e| e.into())?;

  let activity_start_end_time = ActivityStartEndTimeData::new(first_stamp, last_stamp);
  let activity_start_end_time = NodeAdditionalDataContainer::new(activity_start_end_time, event_coordinates);
  event.user_data_mut().put_concrete(START_END_ACTIVITIES_TIMES_KEY.key(), vec![activity_start_end_time]);

  Ok(Rc::new(RefCell::new(event)))
}