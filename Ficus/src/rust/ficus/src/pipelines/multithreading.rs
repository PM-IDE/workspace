use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::event_log::xes::xes_trace::XesTraceImpl;
use crate::features::analysis::log_info::event_log_info::create_threads_log_by_attribute;
use crate::features::clustering::traces::dbscan::clusterize_log_by_traces_dbscan;
use crate::features::clustering::traces::k_means::clusterize_log_by_traces_kmeans_grid_search;
use crate::features::discovery::timeline::discovery::discover_timeline_diagram;
use crate::features::discovery::timeline::events_groups::enumerate_event_groups;
use crate::pipelines::errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError};
use crate::pipelines::keys::context_keys::{EVENT_LOG_KEY, LABELED_LOG_TRACES_DATASET_KEY, LEARNING_ITERATIONS_COUNT_KEY, LOG_THREADS_DIAGRAM_KEY, MIN_EVENTS_IN_CLUSTERS_COUNT_KEY, PIPELINE_KEY, THREAD_ATTRIBUTE_KEY, TIME_ATTRIBUTE_KEY, TIME_DELTA_KEY};
use crate::pipelines::pipeline_parts::PipelineParts;
use crate::pipelines::pipelines::{PipelinePart, PipelinePartFactory};
use crate::utils::user_data::user_data::UserData;
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

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

      let events_groups = enumerate_event_groups(timeline);
      let events_groups_log = Self::create_groups_event_log(&events_groups);
      let mut params = Self::create_traces_clustering_params(context, config)?;
      params.vis_params.log = &events_groups_log;

      let (_, labeled_dataset) = match clusterize_log_by_traces_dbscan(&mut params, min_points_in_cluster) {
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
      for _ in trace_groups {
        let label_name = labels.get(current_label_index).unwrap().to_string();
        abstracted_trace.push(Rc::new(RefCell::new(XesEventImpl::new_with_min_date(label_name))));
        current_label_index += 1;
      }

      abstracted_log.push(Rc::new(RefCell::new(abstracted_trace)));
    }

    abstracted_log
  }

  pub fn clusterize_log_by_traces_k_means_grid_search() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::CLUSTERIZE_LOG_TRACES_K_MEANS_GRID_SEARCH, &|context, infra, config| {
      let mut params = Self::create_traces_clustering_params(context, config)?;
      let learning_iterations_count = *Self::get_user_data(config, &LEARNING_ITERATIONS_COUNT_KEY)? as u64;

      let (logs, labeled_dataset) = match clusterize_log_by_traces_kmeans_grid_search(&mut params, learning_iterations_count) {
        Ok(new_logs) => new_logs,
        Err(error) => return Err(error.into()),
      };

      if let Some(after_clusterization_pipeline) = Self::get_user_data(config, &PIPELINE_KEY).ok() {
        for log in logs {
          let mut new_context = context.clone();
          new_context.put_concrete(EVENT_LOG_KEY.key(), log);

          after_clusterization_pipeline.execute(&mut new_context, infra)?;
        }
      }

      context.put_concrete(LABELED_LOG_TRACES_DATASET_KEY.key(), labeled_dataset);

      Ok(())
    })
  }
}