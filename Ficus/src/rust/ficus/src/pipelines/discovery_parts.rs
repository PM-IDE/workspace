use crate::{
  features::{
    analysis::{
      directly_follows_graph::{construct_dfg, construct_dfg_by_attribute},
      log_info::{event_log_info::OfflineEventLogInfo, log_info_creation_dto::EventLogInfoCreationDto},
    },
    discovery::{
      alpha::{
        alpha::{discover_petri_net_alpha, discover_petri_net_alpha_plus, find_transitions_one_length_loop},
        alpha_plus_plus_nfc::alpha_plus_plus_nfc::discover_petri_net_alpha_plus_plus_nfc,
        providers::{alpha_plus_provider::AlphaPlusRelationsProviderImpl, alpha_provider::DefaultAlphaRelationsProvider},
      },
      fuzzy::fuzzy_miner::discover_graph_fuzzy,
      heuristic::heuristic_miner::discover_petri_net_heuristic,
      petri_net::{marking::ensure_initial_marking, pnml_serialization::serialize_to_pnml_file},
      relations::triangle_relation::OfflineTriangleRelation,
      root_sequence::discovery_xes::discover_root_sequence_graph_from_event_log,
    },
  },
  pipeline_part,
  pipelines::{
    context::PipelineContext,
    errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError},
    keys::context_keys::{
      AND_THRESHOLD_KEY, ATTRIBUTE_KEY, BINARY_FREQUENCY_SIGNIFICANCE_THRESHOLD_KEY, DEPENDENCY_THRESHOLD_KEY, EDGE_CUTOFF_THRESHOLD_KEY,
      EVENT_LOG_INFO_KEY, EVENT_LOG_KEY, GRAPH_KEY, LOOP_LENGTH_TWO_THRESHOLD_KEY, MERGE_SEQUENCES_OF_EVENTS_KEY,
      NODE_CUTOFF_THRESHOLD_KEY, PATH_KEY, PETRI_NET_KEY, PNML_USE_NAMES_AS_IDS_KEY, POSITIVE_OBSERVATIONS_THRESHOLD_KEY,
      PRESERVE_THRESHOLD_KEY, RATIO_THRESHOLD_KEY, RELATIVE_TO_BEST_THRESHOLD_KEY, ROOT_SEQUENCE_KIND_KEY, THREAD_ATTRIBUTE_KEY,
      UNARY_FREQUENCY_THRESHOLD_KEY, UTILITY_RATE_KEY,
    },
    pipeline_parts::PipelineParts,
    pipelines::PipelinePartFactory,
  },
  utils::user_data::user_data::{UserData, UserDataImpl},
};

impl PipelineParts {
  pipeline_part!(discover_petri_net_alpha, |context: &mut PipelineContext, _, _| {
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
    let event_log_info = OfflineEventLogInfo::create_from(EventLogInfoCreationDto::default(log));
    let provider = DefaultAlphaRelationsProvider::new(&event_log_info);
    let discovered_net = discover_petri_net_alpha(&provider);

    context.put_concrete(PETRI_NET_KEY.key(), discovered_net);

    Ok(())
  });

  pipeline_part!(discover_petri_net_alpha_stream, |context: &mut PipelineContext, _, _| {
    let event_log_info = Self::get_user_data(context, &EVENT_LOG_INFO_KEY)?;
    let provider = DefaultAlphaRelationsProvider::new(event_log_info);
    let discovered_net = discover_petri_net_alpha(&provider);

    context.put_concrete(PETRI_NET_KEY.key(), discovered_net);

    Ok(())
  });

  pipeline_part!(serialize_petri_net, |context: &mut PipelineContext, _, config: &UserDataImpl| {
    let petri_net = Self::get_user_data(context, &PETRI_NET_KEY)?;
    let save_path = Self::get_user_data(config, &PATH_KEY)?;
    let use_names_as_ids = *Self::get_user_data(config, &PNML_USE_NAMES_AS_IDS_KEY)?;

    match serialize_to_pnml_file(petri_net, save_path, use_names_as_ids) {
      Ok(_) => Ok(()),
      Err(error) => Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(error.to_string()))),
    }
  });

  pipeline_part!(discover_petri_net_alpha_plus, |context: &mut PipelineContext, _, _| {
    Self::do_discover_petri_net_alpha_plus(context, false)
  });

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

  pipeline_part!(discover_petri_net_alpha_plus_plus, |context: &mut PipelineContext, _, _| {
    Self::do_discover_petri_net_alpha_plus(context, true)
  });

  pipeline_part!(discover_petri_net_alpha_plus_plus_nfc, |context: &mut PipelineContext, _, _| {
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
    let discovered_petri_net = discover_petri_net_alpha_plus_plus_nfc(log);
    context.put_concrete(PETRI_NET_KEY.key(), discovered_petri_net);

    Ok(())
  });

  pipeline_part!(discover_dfg, |context: &mut PipelineContext, _, config: &UserDataImpl| {
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
    let creation_dto = match Self::get_user_data(config, &THREAD_ATTRIBUTE_KEY) {
      Ok(thread_attribute) => EventLogInfoCreationDto::default_thread(log, thread_attribute.to_owned()),
      Err(_) => EventLogInfoCreationDto::default(log),
    };

    context.put_concrete(GRAPH_KEY.key(), construct_dfg(&OfflineEventLogInfo::create_from(creation_dto)));

    Ok(())
  });

  pipeline_part!(discover_dfg_stream, |context: &mut PipelineContext, _, _| {
    let info = Self::get_user_data(context, &EVENT_LOG_INFO_KEY)?;
    context.put_concrete(GRAPH_KEY.key(), construct_dfg(info));

    Ok(())
  });

  pipeline_part!(
    discover_dfg_by_attribute,
    |context: &mut PipelineContext, _, config: &UserDataImpl| {
      let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
      let attribute = Self::get_user_data(config, &ATTRIBUTE_KEY)?;
      let dfg = construct_dfg_by_attribute(log, attribute);

      context.put_concrete(GRAPH_KEY.key(), dfg);

      Ok(())
    }
  );

  pipeline_part!(
    discover_petri_net_heuristic,
    |context: &mut PipelineContext, _, config: &UserDataImpl| {
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
    }
  );

  pipeline_part!(discover_fuzzy_graph, |context: &mut PipelineContext, _, config: &UserDataImpl| {
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
  });

  pipeline_part!(ensure_initial_marking, |context: &mut PipelineContext, _, _| {
    let petri_net = Self::get_user_data_mut(context, &PETRI_NET_KEY)?;
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
    ensure_initial_marking(log, petri_net);

    Ok(())
  });

  pipeline_part!(
    discover_root_sequence_graph,
    |context: &mut PipelineContext, _, config: &UserDataImpl| {
      let log = Self::get_user_data_mut(context, &EVENT_LOG_KEY)?;
      let root_sequence_kind = Self::get_user_data(config, &ROOT_SEQUENCE_KIND_KEY)?;
      let merge_sequences_of_events = Self::get_user_data(config, &MERGE_SEQUENCES_OF_EVENTS_KEY)?;

      match discover_root_sequence_graph_from_event_log(log, *root_sequence_kind, *merge_sequences_of_events) {
        Ok(graph) => {
          context.put_concrete(GRAPH_KEY.key(), graph);
          Ok(())
        }
        Err(err) => Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(err.to_string()))),
      }
    }
  );
}
