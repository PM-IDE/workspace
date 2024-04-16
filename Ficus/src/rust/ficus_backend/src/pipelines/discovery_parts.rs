use crate::features::analysis::directly_follows_graph::construct_dfg;
use crate::features::analysis::event_log_info::{EventLogInfo, EventLogInfoCreationDto};
use crate::features::discovery::alpha::alpha::{discover_petri_net_alpha, discover_petri_net_alpha_plus, find_transitions_one_length_loop};
use crate::features::discovery::alpha::alpha_plus_plus_nfc::alpha_plus_plus_nfc::discover_petri_net_alpha_plus_plus_nfc;
use crate::features::discovery::alpha::providers::alpha_plus_provider::AlphaPlusRelationsProviderImpl;
use crate::features::discovery::alpha::providers::alpha_provider::DefaultAlphaRelationsProvider;
use crate::features::discovery::fuzzy::fuzzy_miner::discover_graph_fuzzy;
use crate::features::discovery::heuristic::heuristic_miner::discover_petri_net_heuristic;
use crate::features::discovery::petri_net::marking::ensure_initial_marking;
use crate::features::discovery::petri_net::pnml_serialization::serialize_to_pnml_file;
use crate::pipelines::context::PipelineContext;
use crate::pipelines::errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError};
use crate::pipelines::keys::context_keys::ContextKeys;
use crate::pipelines::pipeline_parts::PipelineParts;
use crate::pipelines::pipelines::PipelinePartFactory;
use crate::utils::user_data::user_data::UserData;

impl PipelineParts {
    pub(super) fn discover_petri_net_alpha() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DISCOVER_PETRI_NET_ALPHA, &|context, _, keys, _| {
            let log = Self::get_user_data(context, keys.event_log())?;
            let event_log_info = EventLogInfo::create_from(EventLogInfoCreationDto::default(log));
            let provider = DefaultAlphaRelationsProvider::new(&event_log_info);
            let discovered_net = discover_petri_net_alpha(&provider);

            context.put_concrete(keys.petri_net().key(), discovered_net);

            Ok(())
        })
    }

    pub(super) fn serialize_petri_net() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::SERIALIZE_PETRI_NET, &|context, _, keys, config| {
            let petri_net = Self::get_user_data(context, keys.petri_net())?;
            let save_path = Self::get_user_data(config, keys.path())?;
            let use_names_as_ids = *Self::get_user_data(config, keys.pnml_use_names_as_ids())?;

            match serialize_to_pnml_file(petri_net, save_path, use_names_as_ids) {
                Ok(_) => Ok(()),
                Err(error) => Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(error.to_string()))),
            }
        })
    }

    pub(super) fn discover_petri_net_alpha_plus() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DISCOVER_PETRI_NET_ALPHA_PLUS, &|context, _, keys, _| {
            Self::do_discover_petri_net_alpha_plus(context, keys, false)
        })
    }

    fn do_discover_petri_net_alpha_plus(
        context: &mut PipelineContext,
        keys: &ContextKeys,
        alpha_plus_plus: bool,
    ) -> Result<(), PipelinePartExecutionError> {
        let log = Self::get_user_data(context, keys.event_log())?;

        let one_length_loop_transitions = find_transitions_one_length_loop(log);
        let event_log_info = EventLogInfo::create_from(EventLogInfoCreationDto::default_ignore(log, &one_length_loop_transitions));

        let provider = AlphaPlusRelationsProviderImpl::new(&event_log_info, log, &one_length_loop_transitions);

        let discovered_net = discover_petri_net_alpha_plus(log, &provider, alpha_plus_plus);

        context.put_concrete(keys.petri_net().key(), discovered_net);

        Ok(())
    }

    pub(super) fn discover_petri_net_alpha_plus_plus() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DISCOVER_PETRI_NET_ALPHA_PLUS_PLUS, &|context, _, keys, _| {
            Self::do_discover_petri_net_alpha_plus(context, keys, true)
        })
    }

    pub(super) fn discover_petri_net_alpha_plus_plus_nfc() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DISCOVER_PETRI_NET_ALPHA_PLUS_PLUS_NFC, &|context, _, keys, _| {
            let log = Self::get_user_data(context, keys.event_log())?;
            let discovered_petri_net = discover_petri_net_alpha_plus_plus_nfc(log);
            context.put_concrete(keys.petri_net().key(), discovered_petri_net);

            Ok(())
        })
    }

    pub(super) fn discover_directly_follows_graph() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DISCOVER_DFG, &|context, _, keys, _| {
            let log = Self::get_user_data(context, keys.event_log())?;
            let info = EventLogInfo::create_from(EventLogInfoCreationDto::default(log));
            context.put_concrete(keys.graph().key(), construct_dfg(&info));

            Ok(())
        })
    }

    pub(super) fn discover_petri_net_heuristic_miner() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DISCOVER_PETRI_NET_HEURISTIC, &|context, _, keys, config| {
            let log = Self::get_user_data(context, keys.event_log())?;
            let dependency_threshold = *Self::get_user_data(config, keys.dependency_threshold())?;
            let positive_observations_threshold = *Self::get_user_data(config, keys.positive_observations_threshold())? as usize;
            let relative_to_best_threshold = *Self::get_user_data(config, keys.relative_to_best_threshold())?;
            let and_threshold = *Self::get_user_data(config, keys.and_threshold())?;
            let loop_length_two_threshold = *Self::get_user_data(config, keys.loop_length_two_threshold())?;

            let petri_net = discover_petri_net_heuristic(
                log,
                dependency_threshold,
                positive_observations_threshold,
                relative_to_best_threshold,
                and_threshold,
                loop_length_two_threshold,
            );

            context.put_concrete(keys.petri_net().key(), petri_net);

            Ok(())
        })
    }

    pub(super) fn discover_fuzzy_graph() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DISCOVER_FUZZY_GRAPH, &|context, _, keys, config| {
            let log = Self::get_user_data(context, keys.event_log())?;
            let unary_freq_threshold = *Self::get_user_data(config, keys.unary_frequency_threshold())?;
            let binary_sig_threshold = *Self::get_user_data(config, keys.binary_significance_threshold())?;
            let preserve_ratio = *Self::get_user_data(config, keys.preserve_threshold())?;
            let ratio_threshold = *Self::get_user_data(config, keys.ratio_threshold())?;
            let utility_rate = *Self::get_user_data(config, keys.utility_rate())?;
            let edge_cutoff_threshold = *Self::get_user_data(config, keys.edge_cutoff_threshold())?;
            let node_cutoff_threshold = *Self::get_user_data(config, keys.node_cutoff_threshold())?;

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

            context.put_concrete(keys.graph().key(), graph.to_default_graph());

            Ok(())
        })
    }

    pub(super) fn ensure_initial_marking() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::ENSURE_INITIAL_MARKING, &|context, _, keys, _| {
            let petri_net = Self::get_user_data_mut(context, keys.petri_net())?;
            let log = Self::get_user_data(context, keys.event_log())?;
            ensure_initial_marking(log, petri_net);

            Ok(())
        })
    }
}
