use crate::pipelines::context::{PipelineContext, PipelineInfrastructure};
use crate::pipelines::errors::pipeline_errors::{MissingContextError, PipelinePartExecutionError, RawPartExecutionError};
use crate::pipelines::keys::context_key::DefaultContextKey;
use crate::pipelines::pipelines::{DefaultPipelinePart, PipelinePartFactory};
use crate::utils::performance::performance_cookie::performance_cookie;
use crate::utils::user_data::keys::Key;
use crate::utils::user_data::user_data::{UserData, UserDataImpl};
use fancy_regex::Regex;
use std::collections::HashMap;

pub struct PipelineParts {
  names_to_parts: HashMap<String, PipelinePartFactory>,
}

impl PipelineParts {
  pub fn find_part(&self, name: &str) -> Option<&PipelinePartFactory> {
    self.names_to_parts.get(name)
  }
}

unsafe impl Sync for PipelineParts {}
unsafe impl Send for PipelineParts {}

impl PipelineParts {
  pub fn new() -> Self {
    let parts = vec![
      Self::read_log_from_xes(),
      Self::write_log_to_xes(),
      Self::find_primitive_tandem_arrays(),
      Self::find_maximal_tandem_arrays(),
      Self::find_maximal_repeats(),
      Self::find_super_maximal_repeats(),
      Self::find_near_super_maximal_repeats(),
      Self::discover_activities(),
      Self::discover_activities_instances(),
      Self::create_log_from_activities(),
      Self::filter_log_by_event_name(),
      Self::filter_log_by_regex(),
      Self::remain_events_by_regex(),
      Self::filter_log_by_variants(),
      Self::draw_placements_of_event_by_name(),
      Self::draw_events_placements_by_regex(),
      Self::draw_full_activities_diagram(),
      Self::draw_short_activities_diagram(),
      Self::get_event_log_info(),
      Self::clear_activities_related_stuff(),
      Self::get_number_of_underlying_events(),
      Self::filter_traces_by_count(),
      Self::traces_diversity_diagram(),
      Self::get_names_event_log(),
      Self::get_hashes_event_log(),
      Self::use_names_event_log(),
      Self::discover_activities_instances_for_several_levels(),
      Self::discover_activities_in_unattached_subtraces(),
      Self::discover_activities_until_no_more(),
      Self::execute_with_each_activity_log(),
      Self::substitute_underlying_events(),
      Self::execute_frontend_pipeline(),
      Self::apply_class_extractor(),
      Self::discover_petri_net_alpha(),
      Self::serialize_petri_net(),
      Self::add_artificial_start_end_events(),
      Self::add_artificial_start_events(),
      Self::add_artificial_end_events(),
      Self::discover_petri_net_alpha_plus(),
      Self::discover_petri_net_alpha_plus_plus(),
      Self::discover_petri_net_alpha_plus_plus_nfc(),
      Self::discover_directly_follows_graph(),
      Self::discover_petri_net_heuristic_miner(),
      Self::discover_fuzzy_graph(),
      Self::annotate_petri_net_count(),
      Self::annotate_petri_net_frequency(),
      Self::annotate_petri_net_trace_frequency(),
      Self::ensure_initial_marking(),
      Self::read_log_from_bxes(),
      Self::clusterize_activities_from_traces_k_means(),
      Self::clusterize_activities_from_traces_k_means_grid_search(),
      Self::clusterize_activities_from_traces_dbscan(),
      Self::create_traces_activities_dataset(),
      Self::write_log_to_bxes(),
      Self::clusterize_log_traces(),
      Self::serialize_activities_logs(),
      Self::read_xes_from_bytes(),
      Self::read_bxes_from_bytes(),
      Self::write_bxes_to_bytes(),
      Self::write_xes_to_bytes(),
      Self::reverse_hierarchy_indices(),
      Self::discover_cases(),
      Self::annotate_graph_with_time_performance(),
      Self::draw_traces_diversity_diagram_by_attribute(),
      Self::discover_directly_follows_graph_by_attribute(),
      Self::append_attributes_to_name(),
      Self::merge_xes_logs_from_paths(),
      Self::discover_directly_follows_graph_stream(),
      Self::discover_petri_net_alpha_stream(),
      Self::discover_log_threads_diagram(),
      Self::create_threads_log(),
      Self::abstract_timeline_diagram(),
      Self::clusterize_log_by_traces_k_means_grid_search()
    ];

    let mut names_to_parts = HashMap::new();
    for part in parts {
      let prev = names_to_parts.insert((&part.0).to_owned(), part.1);
      assert!(prev.is_none());
    }

    Self { names_to_parts }
  }

  pub fn len(&self) -> usize {
    self.names_to_parts.len()
  }

  pub(super) fn create_pipeline_part(
    name: &'static str,
    executor: &'static impl Fn(&mut PipelineContext, &PipelineInfrastructure, &UserDataImpl) -> Result<(), PipelinePartExecutionError>,
  ) -> (String, PipelinePartFactory) {
    (
      name.to_string(),
      Box::new(|config| {
        DefaultPipelinePart::new(
          name.to_string(),
          config,
          Box::new(|context, infra, config| performance_cookie(name, infra, &mut || executor(context, infra, config))),
        )
      }),
    )
  }

  pub(super) fn get_user_data<'a, T>(
    context: &'a impl UserData,
    key: &DefaultContextKey<T>,
  ) -> Result<&'a T, PipelinePartExecutionError> {
    match context.concrete(key.key()) {
      Some(value) => Ok(value),
      None => Err(PipelinePartExecutionError::MissingContext(MissingContextError::new(
        key.key().name().to_owned(),
      ))),
    }
  }

  pub(super) fn get_user_data_mut<'a, T>(
    context: &'a PipelineContext,
    key: &DefaultContextKey<T>,
  ) -> Result<&'a mut T, PipelinePartExecutionError> {
    match context.concrete_mut(key.key()) {
      Some(value) => Ok(value),
      None => Err(PipelinePartExecutionError::MissingContext(MissingContextError::new(
        key.key().name().to_owned(),
      ))),
    }
  }

  pub(super) fn try_parse_regex(raw_regex: &str) -> Result<Regex, PipelinePartExecutionError> {
    match Regex::new(raw_regex) {
      Ok(regex) => Ok(regex),
      Err(err) => Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(err.to_string()))),
    }
  }
}
