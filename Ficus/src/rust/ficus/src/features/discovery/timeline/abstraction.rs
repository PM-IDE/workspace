use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::event_log::xes::xes_trace::XesTraceImpl;
use crate::features::discovery::root_sequence::context_keys::{EDGE_SOFTWARE_DATA_KEY, NODE_MULTITHREADED_FRAGMENT_LOG_KEY};
use crate::features::discovery::root_sequence::context_keys::EDGE_START_END_ACTIVITIES_TIMES_KEY;
use crate::features::discovery::root_sequence::context_keys::EDGE_TRACE_EXECUTION_INFO_KEY;
use crate::features::discovery::root_sequence::context_keys::NODE_SOFTWARE_DATA_KEY;
use crate::features::discovery::root_sequence::context_keys::NODE_START_END_ACTIVITIES_TIMES_KEY;
use crate::features::discovery::root_sequence::models::ActivityStartEndTimeData;
use crate::features::discovery::root_sequence::models::EdgeTraceExecutionInfo;
use crate::features::discovery::root_sequence::models::EventCoordinates;
use crate::features::discovery::root_sequence::models::NodeAdditionalDataContainer;
use crate::features::discovery::timeline::events_groups::EventGroup;
use crate::features::discovery::timeline::software_data::extraction_config::SoftwareDataExtractionConfig;
use crate::features::discovery::timeline::software_data::extractors::allocations::AllocationDataExtractor;
use crate::features::discovery::timeline::software_data::extractors::array_pools::ArrayPoolDataExtractor;
use crate::features::discovery::timeline::software_data::extractors::assemblies::AssemblySoftwareDataExtractor;
use crate::features::discovery::timeline::software_data::extractors::core::{SoftwareDataExtractionError, EventGroupSoftwareDataExtractor, EventGroupTraceSoftwareDataExtractor};
use crate::features::discovery::timeline::software_data::extractors::event_classes::EventClassesDataExtractor;
use crate::features::discovery::timeline::software_data::extractors::exceptions::ExceptionDataExtractor;
use crate::features::discovery::timeline::software_data::extractors::http::HTTPSoftwareDataExtractor;
use crate::features::discovery::timeline::software_data::extractors::methods::MethodsDataExtractor;
use crate::features::discovery::timeline::software_data::extractors::sockets::SocketsDataExtractor;
use crate::features::discovery::timeline::software_data::extractors::threads::ThreadDataExtractor;
use crate::features::discovery::timeline::software_data::models::SoftwareData;
use crate::features::discovery::timeline::utils::get_stamp;
use crate::pipelines::errors::pipeline_errors::PipelinePartExecutionError;
use crate::pipelines::errors::pipeline_errors::RawPartExecutionError;
use crate::utils::user_data::user_data::UserData;
use crate::utils::user_data::user_data::UserDataOwner;
use log::{error};
use std::cell::RefCell;
use std::rc::Rc;
use crate::features::discovery::multithreaded_dfg::dfg::MULTITHREAD_FRAGMENT_KEY;
use crate::features::discovery::timeline::software_data::extractors::general::activity_duration_extractor::ActivityDurationExtractor;
use crate::features::discovery::timeline::software_data::extractors::general::pie_chart_extractor::PieChartExtractor;
use crate::features::discovery::timeline::software_data::extractors::general::simple_counter::SimpleCounterExtractor;

pub fn abstract_event_groups(
  event_groups: Vec<Vec<EventGroup>>,
  labels: &Vec<usize>,
  thread_attribute: Option<String>,
  time_attribute: Option<String>,
  config: &SoftwareDataExtractionConfig,
) -> Result<XesEventLogImpl, PipelinePartExecutionError> {
  let mut current_label_index = 0;
  let mut abstracted_log = XesEventLogImpl::empty();

  for (trace_id, trace_groups) in event_groups.iter().enumerate() {
    let mut abstracted_trace = XesTraceImpl::empty();

    let mut software_data = trace_groups.iter().map(|_| (SoftwareData::empty(), SoftwareData::empty())).collect::<Vec<(SoftwareData, SoftwareData)>>();

    for extractor in create_trace_extractors(config) {
      extractor.extract(trace_groups, &mut software_data).map_err(|e| PipelinePartExecutionError::new_raw(e.to_string()))?;
    }

    for ((event_index, event_group), (node_data, edge_data)) in trace_groups.iter().enumerate().zip(software_data) {
      if event_group.control_flow_events().is_empty() {
        error!("Encountered empty event group");
        continue;
      }

      let group_label = *labels.get(current_label_index).as_ref().unwrap();
      let abstracted_event = create_abstracted_event(
        &event_group,
        group_label,
        thread_attribute.as_ref(),
        time_attribute.as_ref(),
        EventCoordinates::new(trace_id as u64, event_index as u64),
        config,
        node_data,
        edge_data
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
  thread_attribute: Option<&String>,
  time_attribute: Option<&String>,
  event_coordinates: EventCoordinates,
  config: &SoftwareDataExtractionConfig,
  mut node_software_data: SoftwareData,
  mut edge_software_data: SoftwareData
) -> Result<Rc<RefCell<XesEventImpl>>, PipelinePartExecutionError> {
  let first_stamp = event_group.control_flow_events().first().unwrap().borrow().timestamp().clone();
  let abstracted_event_stamp = *event_group.control_flow_events().last().unwrap().borrow().timestamp() - first_stamp;
  let abstracted_event_stamp = first_stamp + abstracted_event_stamp;

  let label_name = Rc::new(Box::new(label.to_string()));

  let mut event = XesEventImpl::new_all_fields(label_name, abstracted_event_stamp, None);

  extract_software_data(config, event_group, thread_attribute, time_attribute, &mut node_software_data, &mut edge_software_data)?;

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

  if let Some(multithreaded_log) = event_group.user_data().concrete(MULTITHREAD_FRAGMENT_KEY.key()) {
    event.user_data_mut().put_concrete(NODE_MULTITHREADED_FRAGMENT_LOG_KEY.key(), vec![
      NodeAdditionalDataContainer::new(multithreaded_log.clone(), event_coordinates)
    ])
  }

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
  thread_attribute: Option<&String>,
  time_attribute: Option<&String>,
  node_software_data: &mut SoftwareData,
  edge_software_data: &mut SoftwareData
) -> Result<(), PipelinePartExecutionError> {
  let edge_extractors: Vec<Rc<Box<dyn EventGroupSoftwareDataExtractor>>> = create_edge_software_data_extractors(config);

  let mut node_extractors = edge_extractors.clone();
  node_extractors.push(Rc::new(Box::new(EventClassesDataExtractor::new(thread_attribute, time_attribute))));

  for extractor in node_extractors {
    extractor
      .extract(node_software_data, event_group)
      .map_err(|e| PipelinePartExecutionError::Raw(RawPartExecutionError::new(e.to_string())))?;
  }

  if let Some(after_group_events) = event_group.after_group_events() {
    extract_edge_software_data(config, after_group_events.as_slice(), edge_software_data)
      .map_err(|e| PipelinePartExecutionError::Raw(RawPartExecutionError::new(e.to_string())))?;
  }

  Ok(())
}

pub fn extract_edge_software_data(
  config: &SoftwareDataExtractionConfig,
  events: &[Rc<RefCell<XesEventImpl>>],
  edge_software_data: &mut SoftwareData
) -> Result<(), SoftwareDataExtractionError> {
  if events.is_empty() {
    return Ok(());
  }

  for extractor in create_edge_software_data_extractors(config) {
    extractor.extract_from_events(edge_software_data, events)?
  }

  Ok(())
}

fn create_edge_software_data_extractors<'a>(config: &'a SoftwareDataExtractionConfig) -> Vec<Rc<Box<dyn EventGroupSoftwareDataExtractor + 'a>>> {
  vec![
    Rc::new(Box::new(AllocationDataExtractor::<'a>::new(config))),
    Rc::new(Box::new(MethodsDataExtractor::<'a>::new(config))),
    Rc::new(Box::new(ExceptionDataExtractor::<'a>::new(config))),
    Rc::new(Box::new(ArrayPoolDataExtractor::<'a>::new(config))),
    Rc::new(Box::new(AssemblySoftwareDataExtractor::<'a>::new(config))),
    Rc::new(Box::new(HTTPSoftwareDataExtractor::<'a>::new(config))),
    Rc::new(Box::new(SocketsDataExtractor::<'a>::new(config))),
    Rc::new(Box::new(ThreadDataExtractor::<'a>::new(config))),
    Rc::new(Box::new(PieChartExtractor::<'a>::new(config))),
    Rc::new(Box::new(SimpleCounterExtractor::<'a>::new(config))),
  ]
}

fn create_trace_extractors<'a>(config: &'a SoftwareDataExtractionConfig) -> Vec<Rc<Box<dyn EventGroupTraceSoftwareDataExtractor + 'a>>> {
  vec![
    Rc::new(Box::new(ActivityDurationExtractor::<'a>::new(config))),
  ]
}