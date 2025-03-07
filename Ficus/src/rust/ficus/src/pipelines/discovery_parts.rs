use crate::features::analysis::directly_follows_graph::{construct_dfg, construct_dfg_by_attribute};
use crate::features::analysis::log_info::event_log_info::{create_threads_log_by_attribute, OfflineEventLogInfo};
use crate::features::analysis::log_info::log_info_creation_dto::EventLogInfoCreationDto;
use crate::features::discovery::threads_diagram::discovery::discover_timeline_diagram;
use crate::features::discovery::alpha::alpha::{discover_petri_net_alpha, discover_petri_net_alpha_plus, find_transitions_one_length_loop};
use crate::features::discovery::alpha::alpha_plus_plus_nfc::alpha_plus_plus_nfc::discover_petri_net_alpha_plus_plus_nfc;
use crate::features::discovery::alpha::providers::alpha_plus_provider::AlphaPlusRelationsProviderImpl;
use crate::features::discovery::alpha::providers::alpha_provider::DefaultAlphaRelationsProvider;
use crate::features::discovery::fuzzy::fuzzy_miner::discover_graph_fuzzy;
use crate::features::discovery::heuristic::heuristic_miner::discover_petri_net_heuristic;
use crate::features::discovery::petri_net::marking::ensure_initial_marking;
use crate::features::discovery::petri_net::pnml_serialization::serialize_to_pnml_file;
use crate::features::discovery::relations::triangle_relation::OfflineTriangleRelation;
use crate::pipelines::context::PipelineContext;
use crate::pipelines::errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError};
use crate::pipelines::keys::context_keys::{AND_THRESHOLD_KEY, ATTRIBUTE_KEY, BINARY_FREQUENCY_SIGNIFICANCE_THRESHOLD_KEY, DEPENDENCY_THRESHOLD_KEY, EDGE_CUTOFF_THRESHOLD_KEY, EVENT_LOG_INFO_KEY, EVENT_LOG_KEY, GRAPH_KEY, LOG_THREADS_DIAGRAM_KEY, LOG_THREADS_DIAGRAM, LOOP_LENGTH_TWO_THRESHOLD_KEY, NODE_CUTOFF_THRESHOLD_KEY, PATH_KEY, PETRI_NET_KEY, PNML_USE_NAMES_AS_IDS_KEY, POSITIVE_OBSERVATIONS_THRESHOLD_KEY, PRESERVE_THRESHOLD_KEY, RATIO_THRESHOLD_KEY, RELATIVE_TO_BEST_THRESHOLD_KEY, THREAD_ATTRIBUTE_KEY, TIME_ATTRIBUTE, TIME_ATTRIBUTE_KEY, UNARY_FREQUENCY_THRESHOLD_KEY, UTILITY_RATE_KEY, TIME_DELTA_KEY};
use crate::pipelines::pipeline_parts::PipelineParts;
use crate::pipelines::pipelines::PipelinePartFactory;
use crate::utils::user_data::user_data::UserData;

impl PipelineParts {
  pub(super) fn discover_petri_net_alpha() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::DISCOVER_PETRI_NET_ALPHA, &|context, _, _| {
      let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
      let event_log_info = OfflineEventLogInfo::create_from(EventLogInfoCreationDto::default(log));
      let provider = DefaultAlphaRelationsProvider::new(&event_log_info);
      let discovered_net = discover_petri_net_alpha(&provider);

      context.put_concrete(PETRI_NET_KEY.key(), discovered_net);

      Ok(())
    })
  }

  pub(super) fn discover_petri_net_alpha_stream() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::DISCOVER_PETRI_NET_ALPHA_STREAM, &|context, _, _| {
      let event_log_info = Self::get_user_data(context, &EVENT_LOG_INFO_KEY)?;
      let provider = DefaultAlphaRelationsProvider::new(event_log_info);
      let discovered_net = discover_petri_net_alpha(&provider);

      context.put_concrete(PETRI_NET_KEY.key(), discovered_net);

      Ok(())
    })
  }

  pub(super) fn serialize_petri_net() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::SERIALIZE_PETRI_NET, &|context, _, config| {
      let petri_net = Self::get_user_data(context, &PETRI_NET_KEY)?;
      let save_path = Self::get_user_data(config, &PATH_KEY)?;
      let use_names_as_ids = *Self::get_user_data(config, &PNML_USE_NAMES_AS_IDS_KEY)?;

      match serialize_to_pnml_file(petri_net, save_path, use_names_as_ids) {
        Ok(_) => Ok(()),
        Err(error) => Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(error.to_string()))),
      }
    })
  }

  pub(super) fn discover_petri_net_alpha_plus() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::DISCOVER_PETRI_NET_ALPHA_PLUS, &|context, _, _| {
      Self::do_discover_petri_net_alpha_plus(context, false)
    })
  }

  fn do_discover_petri_net_alpha_plus(context: &mut PipelineContext, alpha_plus_plus: bool) -> Result<(), PipelinePartExecutionError> {
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;

    let one_length_loop_transitions = find_transitions_one_length_loop(log);
    let original_log_info = OfflineEventLogInfo::create_from(EventLogInfoCreationDto::default(log));

    let dto = EventLogInfoCreationDto::default_ignore(log, &one_length_loop_transitions);
    let ignored_event_log_info = OfflineEventLogInfo::create_from(dto);

    let triangle_relation = OfflineTriangleRelation::new(log);

    let provider = AlphaPlusRelationsProviderImpl::new(&ignored_event_log_info, &triangle_relation, &one_length_loop_transitions);

    let discovered_net = discover_petri_net_alpha_plus(&provider, &original_log_info, alpha_plus_plus);

    context.put_concrete(PETRI_NET_KEY.key(), discovered_net);

    Ok(())
  }

  pub(super) fn discover_petri_net_alpha_plus_plus() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::DISCOVER_PETRI_NET_ALPHA_PLUS_PLUS, &|context, _, _| {
      Self::do_discover_petri_net_alpha_plus(context, true)
    })
  }

  pub(super) fn discover_petri_net_alpha_plus_plus_nfc() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::DISCOVER_PETRI_NET_ALPHA_PLUS_PLUS_NFC, &|context, _, _| {
      let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
      let discovered_petri_net = discover_petri_net_alpha_plus_plus_nfc(log);
      context.put_concrete(PETRI_NET_KEY.key(), discovered_petri_net);

      Ok(())
    })
  }

  pub(super) fn discover_directly_follows_graph() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::DISCOVER_DFG, &|context, _, config| {
      let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
      let creation_dto = match Self::get_user_data(config, &THREAD_ATTRIBUTE_KEY) {
        Ok(thread_attribute) => EventLogInfoCreationDto::default_thread(log, thread_attribute.to_owned()),
        Err(_) => EventLogInfoCreationDto::default(log)
      };

      context.put_concrete(GRAPH_KEY.key(), construct_dfg(&OfflineEventLogInfo::create_from(creation_dto)));

      Ok(())
    })
  }

  pub(super) fn discover_directly_follows_graph_stream() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::DISCOVER_DFG_STREAM, &|context, _, _| {
      let info = Self::get_user_data(context, &EVENT_LOG_INFO_KEY)?;
      context.put_concrete(GRAPH_KEY.key(), construct_dfg(info));

      Ok(())
    })
  }

  pub(super) fn discover_directly_follows_graph_by_attribute() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::DISCOVER_DFG_BY_ATTRIBUTE, &|context, _, config| {
      let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
      let attribute = Self::get_user_data(config, &ATTRIBUTE_KEY)?;
      let dfg = construct_dfg_by_attribute(log, attribute);

      context.put_concrete(GRAPH_KEY.key(), dfg);

      Ok(())
    })
  }

  pub(super) fn discover_petri_net_heuristic_miner() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::DISCOVER_PETRI_NET_HEURISTIC, &|context, _, config| {
      let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
      let dependency_threshold = *Self::get_user_data(config, &DEPENDENCY_THRESHOLD_KEY)?;
      let positive_observations_threshold = *Self::get_user_data(config, &POSITIVE_OBSERVATIONS_THRESHOLD_KEY)? as usize;
      let relative_to_best_threshold = *Self::get_user_data(config, &RELATIVE_TO_BEST_THRESHOLD_KEY)?;
      let and_threshold = *Self::get_user_data(config, &AND_THRESHOLD_KEY)?;
      let loop_length_two_threshold = *Self::get_user_data(config, &LOOP_LENGTH_TWO_THRESHOLD_KEY)?;

      let triangle_relation = OfflineTriangleRelation::new(log);
      let info = OfflineEventLogInfo::create_from(EventLogInfoCreationDto::default(log));

      let petri_net = discover_petri_net_heuristic(
        &info,
        &triangle_relation,
        dependency_threshold,
        positive_observations_threshold,
        relative_to_best_threshold,
        and_threshold,
        loop_length_two_threshold,
      );

      context.put_concrete(PETRI_NET_KEY.key(), petri_net);

      Ok(())
    })
  }

  pub(super) fn discover_fuzzy_graph() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::DISCOVER_FUZZY_GRAPH, &|context, _, config| {
      let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
      let unary_freq_threshold = *Self::get_user_data(config, &UNARY_FREQUENCY_THRESHOLD_KEY)?;
      let binary_sig_threshold = *Self::get_user_data(config, &BINARY_FREQUENCY_SIGNIFICANCE_THRESHOLD_KEY)?;
      let preserve_ratio = *Self::get_user_data(config, &PRESERVE_THRESHOLD_KEY)?;
      let ratio_threshold = *Self::get_user_data(config, &RATIO_THRESHOLD_KEY)?;
      let utility_rate = *Self::get_user_data(config, &UTILITY_RATE_KEY)?;
      let edge_cutoff_threshold = *Self::get_user_data(config, &EDGE_CUTOFF_THRESHOLD_KEY)?;
      let node_cutoff_threshold = *Self::get_user_data(config, &NODE_CUTOFF_THRESHOLD_KEY)?;

      let graph = discover_graph_fuzzy(
        log,
        unary_freq_threshold,
        binary_sig_threshold,
        preserve_ratio,
        ratio_threshold,
        utility_rate,
        edge_cutoff_threshold,
        node_cutoff_threshold,
      );

      context.put_concrete(GRAPH_KEY.key(), graph.to_default_graph());

      Ok(())
    })
  }

  pub(super) fn ensure_initial_marking() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::ENSURE_INITIAL_MARKING, &|context, _, _| {
      let petri_net = Self::get_user_data_mut(context, &PETRI_NET_KEY)?;
      let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
      ensure_initial_marking(log, petri_net);

      Ok(())
    })
  }

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
    Self::create_pipeline_part(Self::ABSTRACT_TIMELINE_DIAGRAM, &|context, _, _| {
      Ok(())
    })
  }
}
