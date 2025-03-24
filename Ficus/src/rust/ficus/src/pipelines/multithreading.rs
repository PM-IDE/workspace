use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::event_log::xes::xes_trace::XesTraceImpl;
use crate::features::analysis::log_info::event_log_info::create_threads_log_by_attribute;
use crate::features::clustering::traces::dbscan::clusterize_log_by_traces_dbscan;
use crate::features::discovery::timeline::discovery::{discover_timeline_diagram, TraceThread};
use crate::features::discovery::timeline::events_groups::enumerate_event_groups;
use crate::pipelines::errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError};
use crate::pipelines::keys::context_keys::{EVENT_LOG_KEY, LABELED_LOG_TRACES_DATASET_KEY, LOG_THREADS_DIAGRAM_KEY, MIN_EVENTS_IN_CLUSTERS_COUNT_KEY, PIPELINE_KEY, SOFTWARE_DATA_KEY, THREAD_ATTRIBUTE_KEY, TIME_ATTRIBUTE_KEY, TIME_DELTA_KEY, TOLERANCE_KEY};
use crate::pipelines::pipeline_parts::PipelineParts;
use crate::pipelines::pipelines::{PipelinePart, PipelinePartFactory};
use crate::utils::user_data::user_data::UserData;
use log::error;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Copy, Clone)]
pub enum FeatureCountKindDto {
  One,
  Count,
  OneIfMoreThanMaxFromAllFeatures,
}

#[derive(Clone, Debug)]
pub struct SoftwareData {
  event_classes: HashMap<String, usize>,
  thread_diagram_fragment: Vec<TraceThread>,
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
        let abstracted_log = Self::create_simple_abstracted_log(events_groups, labeled_dataset.labels());
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

  fn create_simple_abstracted_log(event_groups: Vec<Vec<Vec<Rc<RefCell<XesEventImpl>>>>>, labels: &Vec<usize>) -> XesEventLogImpl {
    let mut current_label_index = 0;
    let mut abstracted_log = XesEventLogImpl::empty();

    for trace_groups in event_groups {
      let mut abstracted_trace = XesTraceImpl::empty();
      for event_group in trace_groups {
        if event_group.is_empty() {
          error!("Encountered empty event group");
          continue;
        }

        abstracted_trace.push(Self::create_abstracted_event(&event_group, labels.get(current_label_index).as_ref().unwrap()));
        current_label_index += 1;
      }

      abstracted_log.push(Rc::new(RefCell::new(abstracted_trace)));
    }

    abstracted_log
  }

  fn create_abstracted_event(event_group: &Vec<Rc<RefCell<XesEventImpl>>>, label: &usize) -> Rc<RefCell<XesEventImpl>> {
    let first_stamp = event_group.first().unwrap().borrow().timestamp().clone();
    let abstracted_event_stamp = *event_group.last().unwrap().borrow().timestamp() - first_stamp;
    let abstracted_event_stamp = first_stamp + abstracted_event_stamp;

    let label_name = Rc::new(Box::new(label.to_string()));

    let mut event_classes = HashMap::new();
    for event in event_group {
      *event_classes.entry(event.borrow().name().clone()).or_insert(0) += 1;
    }

    let software_data = SoftwareData {
      event_classes,
      thread_diagram_fragment: vec![],
    };

    let mut event = XesEventImpl::new_all_fields(label_name, abstracted_event_stamp, None);
    event.user_data_mut().put_concrete(SOFTWARE_DATA_KEY.key(), software_data);

    Rc::new(RefCell::new(event))
  }
}