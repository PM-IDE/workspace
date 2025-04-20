use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::event_log::xes::xes_trace::XesTraceImpl;
use crate::features::discovery::root_sequence::context_keys::{EDGE_SOFTWARE_DATA_KEY, EDGE_START_END_ACTIVITIES_TIMES_KEY, EDGE_TRACE_EXECUTION_INFO_KEY, NODE_SOFTWARE_DATA_KEY, NODE_START_END_ACTIVITIES_TIMES_KEY};
use crate::features::discovery::root_sequence::models::{ActivityStartEndTimeData, EdgeTraceExecutionInfo, EventCoordinates, NodeAdditionalDataContainer};
use crate::features::discovery::timeline::events_groups::EventGroup;
use crate::features::discovery::timeline::software_data::extraction_config::SoftwareDataExtractionConfig;
use crate::features::discovery::timeline::software_data::extractors::allocations::AllocationDataExtractor;
use crate::features::discovery::timeline::software_data::extractors::core::SoftwareDataExtractor;
use crate::features::discovery::timeline::software_data::extractors::event_classes::EventClassesDataExtractor;
use crate::features::discovery::timeline::software_data::extractors::exceptions::ExceptionDataExtractor;
use crate::features::discovery::timeline::software_data::extractors::methods::MethodsDataExtractor;
use crate::features::discovery::timeline::software_data::models::SoftwareData;
use crate::features::discovery::timeline::utils::get_stamp;
use crate::pipelines::errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError};
use crate::utils::user_data::user_data::{UserData, UserDataOwner};
use log::error;
use std::cell::RefCell;
use std::rc::Rc;

pub fn abstract_event_groups(
  event_groups: Vec<Vec<EventGroup>>,
  labels: &Vec<usize>,
  thread_attribute: String,
  time_attribute: Option<String>,
  config: &SoftwareDataExtractionConfig,
) -> Result<XesEventLogImpl, PipelinePartExecutionError> {
  let mut current_label_index = 0;
  let mut abstracted_log = XesEventLogImpl::empty();

  for (trace_id, trace_groups) in event_groups.iter().enumerate() {
    let mut abstracted_trace = XesTraceImpl::empty();
    for (event_index, event_group) in trace_groups.iter().enumerate() {
      if event_group.control_flow_events().is_empty() {
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
        config,
      )?;

      abstracted_trace.push(abstracted_event);
      current_label_index += 1;
    }

    abstracted_log.push(Rc::new(RefCell::new(abstracted_trace)));
  }

  Ok(abstracted_log)
}

fn create_abstracted_event(
  event_group: &EventGroup,
  label: &usize,
  thread_attribute: &str,
  time_attribute: Option<&String>,
  event_coordinates: EventCoordinates,
  config: &SoftwareDataExtractionConfig,
) -> Result<Rc<RefCell<XesEventImpl>>, PipelinePartExecutionError> {
  let first_stamp = event_group.control_flow_events().first().unwrap().borrow().timestamp().clone();
  let abstracted_event_stamp = *event_group.control_flow_events().last().unwrap().borrow().timestamp() - first_stamp;
  let abstracted_event_stamp = first_stamp + abstracted_event_stamp;

  let label_name = Rc::new(Box::new(label.to_string()));

  let mut event = XesEventImpl::new_all_fields(label_name, abstracted_event_stamp, None);

  let (node_software_data, edge_software_data) = extract_software_data(config, event_group, thread_attribute, time_attribute)?;

  put_node_user_data(&mut event, node_software_data, event_coordinates, event_group, time_attribute)?;

  if let Some(after_group_events) = event_group.after_group_events() {
    put_edge_user_data(&mut event, edge_software_data, event_coordinates, after_group_events, time_attribute)?; 
  }

  Ok(Rc::new(RefCell::new(event)))
}

fn put_node_user_data(
  event: &mut XesEventImpl,
  node_software_data: SoftwareData,
  event_coordinates: EventCoordinates,
  event_group: &EventGroup,
  time_attribute: Option<&String>,
) -> Result<(), PipelinePartExecutionError> {
  let software_data = NodeAdditionalDataContainer::new(node_software_data, event_coordinates);
  event.user_data_mut().put_concrete(NODE_SOFTWARE_DATA_KEY.key(), vec![software_data]);

  let first_stamp = get_stamp(&event_group.control_flow_events().first().unwrap().borrow(), time_attribute).map_err(|e| e.into())?;
  let last_stamp = get_stamp(&event_group.control_flow_events().last().unwrap().borrow(), time_attribute).map_err(|e| e.into())?;

  let activity_start_end_time = ActivityStartEndTimeData::new(first_stamp, last_stamp);
  let activity_start_end_time = NodeAdditionalDataContainer::new(activity_start_end_time, event_coordinates);
  event.user_data_mut().put_concrete(NODE_START_END_ACTIVITIES_TIMES_KEY.key(), vec![activity_start_end_time]);

  Ok(())
}

fn put_edge_user_data(
  event: &mut XesEventImpl,
  edge_software_data: SoftwareData,
  event_coordinates: EventCoordinates,
  after_group_events: &Vec<Rc<RefCell<XesEventImpl>>>,
  time_attribute: Option<&String>,
) -> Result<(), PipelinePartExecutionError> {
  event.user_data_mut().put_concrete(EDGE_SOFTWARE_DATA_KEY.key(), vec![edge_software_data]);

  let first_stamp = get_stamp(&after_group_events.first().unwrap().borrow(), time_attribute).map_err(|e| e.into())?;
  let last_stamp = get_stamp(&after_group_events.last().unwrap().borrow(), time_attribute).map_err(|e| e.into())?;

  let edge_start_end_time = ActivityStartEndTimeData::new(first_stamp, last_stamp);

  event.user_data_mut().put_concrete(EDGE_START_END_ACTIVITIES_TIMES_KEY.key(), vec![edge_start_end_time]);
  event.user_data_mut().put_concrete(EDGE_TRACE_EXECUTION_INFO_KEY.key(), vec![EdgeTraceExecutionInfo::new(event_coordinates.trace_id())]);

  Ok(())
}

fn extract_software_data(
  config: &SoftwareDataExtractionConfig,
  event_group: &EventGroup,
  thread_attribute: &str,
  time_attribute: Option<&String>,
) -> Result<(SoftwareData, SoftwareData), PipelinePartExecutionError> {
  let extractors: Vec<Box<dyn SoftwareDataExtractor>> = vec![
    Box::new(AllocationDataExtractor::new(config)),
    Box::new(EventClassesDataExtractor::new(thread_attribute, time_attribute)),
    Box::new(MethodsDataExtractor::new(config)),
    Box::new(ExceptionDataExtractor::new(config)),
  ];

  let mut node_software_data = SoftwareData::empty();
  let mut edge_software_data = SoftwareData::empty();

  for extractor in extractors {
    extractor
      .extract(&mut node_software_data, event_group)
      .map_err(|e| PipelinePartExecutionError::Raw(RawPartExecutionError::new(e.to_string())))?;

    if let Some(after_group_events) = event_group.after_group_events() {
      extractor
        .extract_from_events(&mut edge_software_data, after_group_events)
        .map_err(|e| PipelinePartExecutionError::Raw(RawPartExecutionError::new(e.to_string())))? 
    }
  }

  Ok((node_software_data, edge_software_data))
}