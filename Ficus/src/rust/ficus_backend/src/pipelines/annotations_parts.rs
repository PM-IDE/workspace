use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::features::discovery::petri_net::annotations::{annotate_with_counts, annotate_with_frequencies, annotate_with_trace_frequency};
use crate::features::discovery::petri_net::petri_net::DefaultPetriNet;
use crate::pipelines::context::PipelineContext;
use crate::pipelines::errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError};
use crate::pipelines::keys::context_key::DefaultContextKey;
use crate::pipelines::keys::context_keys::ContextKeys;
use crate::pipelines::pipeline_parts::PipelineParts;
use crate::pipelines::pipelines::PipelinePartFactory;
use crate::utils::user_data::user_data::UserData;
use crate::utils::user_data::user_data::UserDataImpl;
use std::collections::HashMap;

impl PipelineParts {
    pub(super) fn annotate_petri_net_count() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::ANNOTATE_PETRI_NET_COUNT, &|context, _, keys, config| {
            Self::annotate_petri_net(
                keys.petri_net_count_annotation(),
                context,
                keys,
                config,
                |log, net, terminate_on_unreplayable_traces| annotate_with_counts(log, net, terminate_on_unreplayable_traces),
            )
        })
    }

    fn annotate_petri_net<T>(
        annotation_key: &DefaultContextKey<HashMap<u64, T>>,
        context: &mut PipelineContext,
        keys: &ContextKeys,
        config: &UserDataImpl,
        annotator: impl Fn(&XesEventLogImpl, &DefaultPetriNet, bool) -> Option<HashMap<u64, T>>,
    ) -> Result<(), PipelinePartExecutionError> {
        let log = Self::get_user_data(context, keys.event_log())?;
        let petri_net = Self::get_user_data(context, keys.petri_net())?;
        let terminate_on_unreplayable_traces = *Self::get_user_data(config, keys.terminate_on_unreplayable_traces())?;

        let annotation = annotator(log, petri_net, terminate_on_unreplayable_traces);
        if let Some(annotation) = annotation {
            context.put_concrete(annotation_key.key(), annotation);
            Ok(())
        } else {
            let error = RawPartExecutionError::new("Failed to annotate petri net".to_owned());
            Err(PipelinePartExecutionError::Raw(error))
        }
    }

    pub(super) fn annotate_petri_net_frequency() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::ANNOTATE_PETRI_NET_FREQUENCY, &|context, _, keys, config| {
            Self::annotate_petri_net(
                keys.petri_net_frequency_annotation(),
                context,
                keys,
                config,
                |log, net, terminate_on_unreplayable_traces| annotate_with_frequencies(log, net, terminate_on_unreplayable_traces),
            )
        })
    }

    pub(super) fn annotate_petri_net_trace_frequency() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::ANNOTATE_PETRI_NET_TRACE_FREQUENCY, &|context, _, keys, config| {
            Self::annotate_petri_net(
                keys.petri_net_trace_frequency_annotation(),
                context,
                keys,
                config,
                |log, net, terminate_on_unreplayable_traces| annotate_with_trace_frequency(log, net, terminate_on_unreplayable_traces),
            )
        })
    }
}
