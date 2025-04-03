use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use log::error;
use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::event_log::xes::xes_trace::XesTraceImpl;
use crate::features::discovery::root_sequence::models::ActivityStartEndTimeData;
use crate::features::discovery::timeline::discovery::{TraceThread, TraceThreadEvent};
use crate::features::discovery::timeline::utils::{extract_thread_id, get_stamp};
use crate::pipelines::errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError};
use crate::pipelines::keys::context_keys::{SOFTWARE_DATA_KEY, START_END_ACTIVITY_TIME_KEY};
use crate::utils::user_data::user_data::{UserData, UserDataOwner};

pub fn abstract_event_groups(
  event_groups: Vec<Vec<Vec<Rc<RefCell<XesEventImpl>>>>>,
  labels: &Vec<usize>,
  thread_attribute: String,
  time_attribute: Option<String>,
) -> Result<XesEventLogImpl, PipelinePartExecutionError> {
  let mut current_label_index = 0;
  let mut abstracted_log = XesEventLogImpl::empty();

  for trace_groups in event_groups.iter() {
    let mut abstracted_trace = XesTraceImpl::empty();
    for event_group in trace_groups.iter() {
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
      )?;

      abstracted_trace.push(abstracted_event);
      current_label_index += 1;
    }

    abstracted_log.push(Rc::new(RefCell::new(abstracted_trace)));
  }

  Ok(abstracted_log)
}

#[derive(Clone, Debug)]
pub struct SoftwareData {
  event_classes: HashMap<String, usize>,
  thread_diagram_fragment: Vec<TraceThread>,
}

impl SoftwareData {
  pub fn empty() -> Self {
    Self {
      event_classes: HashMap::new(),
      thread_diagram_fragment: vec![],
    }
  }

  pub fn event_classes(&self) -> &HashMap<String, usize> {
    &self.event_classes
  }

  pub fn thread_diagram_fragment(&self) -> &Vec<TraceThread> {
    &self.thread_diagram_fragment
  }
}

fn create_abstracted_event(
  event_group: &Vec<Rc<RefCell<XesEventImpl>>>,
  label: &usize,
  thread_attribute: &str,
  time_attribute: Option<&String>,
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
  };

  let first_stamp = event_group.first().unwrap().borrow().timestamp().clone();
  let abstracted_event_stamp = *event_group.last().unwrap().borrow().timestamp() - first_stamp;
  let abstracted_event_stamp = first_stamp + abstracted_event_stamp;

  let label_name = Rc::new(Box::new(label.to_string()));

  let mut event = XesEventImpl::new_all_fields(label_name, abstracted_event_stamp, None);
  event.user_data_mut().put_concrete(SOFTWARE_DATA_KEY.key(), vec![software_data]);

  let first_stamp = get_stamp(&event_group.first().unwrap().borrow(), time_attribute).map_err(|e| e.into())?;
  let last_stamp = get_stamp(&event_group.last().unwrap().borrow(), time_attribute).map_err(|e| e.into())?;

  event.user_data_mut().put_concrete(START_END_ACTIVITY_TIME_KEY.key(), ActivityStartEndTimeData::new(first_stamp, last_stamp));

  Ok(Rc::new(RefCell::new(event)))
}