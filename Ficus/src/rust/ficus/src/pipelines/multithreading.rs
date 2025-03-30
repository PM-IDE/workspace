use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::event_log::xes::xes_trace::XesTraceImpl;
use crate::features::analysis::log_info::event_log_info::create_threads_log_by_attribute;
use crate::features::clustering::traces::dbscan::clusterize_log_by_traces_dbscan;
use crate::features::discovery::timeline::discovery::{discover_timeline_diagram, TraceThread, TraceThreadEvent};
use crate::features::discovery::timeline::events_groups::enumerate_event_groups;
use crate::features::discovery::timeline::utils::{extract_thread_id, get_stamp};
use crate::pipelines::errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError};
use crate::pipelines::keys::context_keys::{EVENT_LOG_KEY, LABELED_LOG_TRACES_DATASET_KEY, LOG_THREADS_DIAGRAM_KEY, MIN_EVENTS_IN_CLUSTERS_COUNT_KEY, PIPELINE_KEY, SOFTWARE_DATA_KEY, START_END_ACTIVITY_TIME_KEY, THREAD_ATTRIBUTE_KEY, TIME_ATTRIBUTE_KEY, TIME_DELTA_KEY, TOLERANCE_KEY};
use crate::pipelines::pipeline_parts::PipelineParts;
use crate::pipelines::pipelines::{PipelinePart, PipelinePartFactory};
use crate::utils::user_data::user_data::UserData;
use log::error;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use std::str::FromStr;
use crate::features::discovery::root_sequence::models::ActivityStartEndTimeData;

#[derive(Copy, Clone)]
pub enum FeatureCountKindDto {
  One,
  Count,
  OneIfMoreThanMaxFromAllFeatures,
}

impl FromStr for FeatureCountKindDto {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "One" => Ok(Self::One),
      "Count" => Ok(Self::Count),
      "OneIfMoreThanMaxFromAllFeatures" => Ok(Self::OneIfMoreThanMaxFromAllFeatures),
      _ => Err(()),
    }
  }
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

impl PipelineParts {
  pub(super) fn discover_log_threads_diagram() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::DISCOVER_LOG_TIMELINE_DIAGRAM, &|context, _, config| {
      let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
      let thread_attribute = Self::get_user_data(config, &THREAD_ATTRIBUTE_KEY)?;
      let time_attribute = Self::get_user_data(config, &TIME_ATTRIBUTE_KEY).ok();
      let event_group_delta = Self::get_user_data(config, &TIME_DELTA_KEY).ok();

      let diagram = discover_timeline_diagram(
        log,
        thread_attribute.as_str(),
        time_attribute,
        match event_group_delta {
          None => None,
          Some(delta) => Some(*delta as u64)
        },
      );

      match diagram {
        Err(_) => {
          return Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(
            "Failed to build diagram".to_string(),
          )))
        }
        Ok(diagram) => context.put_concrete(LOG_THREADS_DIAGRAM_KEY.key(), diagram),
      }

      Ok(())
    })
  }

  pub(super) fn create_threads_log() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::CREATE_THREADS_LOG, &|context, _, config| {
      let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
      let thread_attribute = Self::get_user_data(config, &THREAD_ATTRIBUTE_KEY)?;
      context.put_concrete(EVENT_LOG_KEY.key(), create_threads_log_by_attribute(log, thread_attribute));

      Ok(())
    })
  }

  pub(super) fn abstract_timeline_diagram() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::ABSTRACT_TIMELINE_DIAGRAM, &|context, infra, config| {
      let timeline = Self::get_user_data(context, &LOG_THREADS_DIAGRAM_KEY)?;
      let thread_attribute = timeline.thread_attribute().to_string();
      let time_attribute = timeline.time_attribute().cloned();

      let min_points_in_cluster = *Self::get_user_data(config, &MIN_EVENTS_IN_CLUSTERS_COUNT_KEY)? as usize;
      let tolerance = *Self::get_user_data(config, &TOLERANCE_KEY)?;

      let events_groups = enumerate_event_groups(timeline);
      let events_groups_log = Self::create_groups_event_log(&events_groups);
      let mut params = Self::create_traces_clustering_params(context, config)?;
      params.vis_params.log = &events_groups_log;

      let (_, labeled_dataset) = match clusterize_log_by_traces_dbscan(&mut params, tolerance, min_points_in_cluster) {
        Ok(new_logs) => new_logs,
        Err(error) => return Err(error.into()),
      };

      if let Some(after_clusterization_pipeline) = Self::get_user_data(config, &PIPELINE_KEY).ok() {
        let abstracted_log = Self::create_simple_abstracted_log(events_groups, labeled_dataset.labels(), thread_attribute, time_attribute)?;

        let mut new_context = context.clone();
        new_context.put_concrete(EVENT_LOG_KEY.key(), abstracted_log);

        after_clusterization_pipeline.execute(&mut new_context, infra)?;
      }

      context.put_concrete(LABELED_LOG_TRACES_DATASET_KEY.key(), labeled_dataset);

      Ok(())
    })
  }

  fn create_groups_event_log(events_groups: &Vec<Vec<Vec<Rc<RefCell<XesEventImpl>>>>>) -> XesEventLogImpl {
    let mut log = XesEventLogImpl::empty();

    for trace in events_groups {
      for group in trace {
        let mut new_trace = XesTraceImpl::empty();

        for event in group {
          new_trace.push(event.clone());
        }

        log.push(Rc::new(RefCell::new(new_trace)));
      }
    }

    log
  }

  fn create_simple_abstracted_log(
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
        let abstracted_event = Self::create_abstracted_event(
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
}