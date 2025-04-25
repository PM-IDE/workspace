use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::event_log::xes::xes_trace::XesTraceImpl;
use crate::features::analysis::log_info::event_log_info::create_threads_log_by_attribute;
use crate::features::clustering::traces::dbscan::clusterize_log_by_traces_dbscan;
use crate::features::discovery::root_sequence::log_prepare::prepare_software_log;
use crate::features::discovery::timeline::abstraction::abstract_event_groups;
use crate::features::discovery::timeline::discovery::{discover_timeline_diagram, discover_traces_timeline_diagram};
use crate::features::discovery::timeline::events_groups::{enumerate_event_groups, EventGroup};
use crate::features::discovery::timeline::software_data::extraction_config::SoftwareDataExtractionConfig;
use crate::pipelines::context::PipelineContext;
use crate::pipelines::errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError};
use crate::pipelines::keys::context_keys::{DISCOVER_EVENTS_GROUPS_IN_EACH_TRACE_KEY, EVENT_LOG_KEY, LABELED_LOG_TRACES_DATASET_KEY, LOG_THREADS_DIAGRAM_KEY, MIN_EVENTS_IN_CLUSTERS_COUNT_KEY, PIPELINE_KEY, SOFTWARE_DATA_EXTRACTION_CONFIG_KEY, THREAD_ATTRIBUTE_KEY, TIME_ATTRIBUTE_KEY, TIME_DELTA_KEY, TOLERANCE_KEY};
use crate::pipelines::pipeline_parts::PipelineParts;
use crate::pipelines::pipelines::{PipelinePart, PipelinePartFactory};
use crate::utils::user_data::user_data::UserData;
use fancy_regex::Regex;
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
      let software_data_extraction_config = Self::get_software_data_extraction_config(context);

      let diagram = discover_timeline_diagram(
        log,
        thread_attribute.as_str(),
        time_attribute,
        match event_group_delta {
          None => None,
          Some(delta) => Some(*delta as u64)
        },
        Self::get_control_flow_regexes(&software_data_extraction_config)?.as_ref()
      );

      match diagram {
        Err(err) => return Err(err.into()),
        Ok(diagram) => context.put_concrete(LOG_THREADS_DIAGRAM_KEY.key(), diagram),
      }

      Ok(())
    })
  }

  fn get_control_flow_regexes(config: &SoftwareDataExtractionConfig) -> Result<Option<Vec<Regex>>, PipelinePartExecutionError> {
    config.control_flow_regexes().map_err(|message| PipelinePartExecutionError::Raw(RawPartExecutionError::new(message)))
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
      let time_attribute = timeline.time_attribute().as_ref().cloned();
      let extraction_config = Self::get_software_data_extraction_config(context);

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
        let abstracted_log = abstract_event_groups(
          events_groups,
          labeled_dataset.labels(),
          thread_attribute,
          time_attribute,
          &extraction_config,
        )?;

        let mut new_context = context.clone();
        new_context.put_concrete(EVENT_LOG_KEY.key(), abstracted_log);

        after_clusterization_pipeline.execute(&mut new_context, infra)?;
      }

      context.put_concrete(LABELED_LOG_TRACES_DATASET_KEY.key(), labeled_dataset);

      Ok(())
    })
  }

  fn get_software_data_extraction_config(context: &PipelineContext) -> SoftwareDataExtractionConfig {
    match Self::get_user_data(context, &SOFTWARE_DATA_EXTRACTION_CONFIG_KEY) {
      Ok(config) => config.clone(),
      Err(_) => SoftwareDataExtractionConfig::empty(),
    }
  }

  fn create_groups_event_log(events_groups: &Vec<Vec<EventGroup>>) -> XesEventLogImpl {
    let mut log = XesEventLogImpl::empty();

    for trace in events_groups {
      for group in trace {
        let mut new_trace = XesTraceImpl::empty();

        for event in group.control_flow_events() {
          new_trace.push(event.clone());
        }

        log.push(Rc::new(RefCell::new(new_trace)));
      }
    }

    log
  }

  pub(super) fn discover_traces_timeline_diagram() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::DISCOVER_TRACES_TIMELINE_DIAGRAM, &|context, _, config| {
      let time_attribute = Self::get_user_data(config, &TIME_ATTRIBUTE_KEY).ok();
      let event_group_delta = Self::get_user_data(config, &TIME_DELTA_KEY).ok();
      let discover_events_groups_in_each_trace = Self::get_user_data(config, &DISCOVER_EVENTS_GROUPS_IN_EACH_TRACE_KEY)?;
      let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
      let software_data_extraction_config = Self::get_software_data_extraction_config(context);

      let diagram = discover_traces_timeline_diagram(
        log,
        time_attribute,
        match event_group_delta {
          None => None,
          Some(delta) => Some(*delta as u64)
        },
        *discover_events_groups_in_each_trace,
        Self::get_control_flow_regexes(&software_data_extraction_config)?.as_ref()
      );

      match diagram {
        Err(err) => return Err(err.into()),
        Ok(diagram) => context.put_concrete(LOG_THREADS_DIAGRAM_KEY.key(), diagram),
      }

      Ok(())
    })
  }

  pub(super) fn prepare_software_log() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::PREPARE_SOFTWARE_EVENT_LOG, &|context, _, config| {
      let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
      let software_data_extraction_config = Self::get_software_data_extraction_config(context);

      let prepared_log = match prepare_software_log(log, &software_data_extraction_config) {
        Ok(log) => log,
        Err(err) => return Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(err)))
      };

      context.put_concrete(EVENT_LOG_KEY.key(), prepared_log);

      Ok(())
    })
  }
}