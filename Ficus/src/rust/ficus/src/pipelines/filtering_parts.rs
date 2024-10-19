use std::collections::HashSet;

use fancy_regex::Regex;

use super::{
    errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError},
    pipelines::PipelinePartFactory,
};
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::features::mutations::filtering::remain_events_in_event_log;
use crate::pipelines::context::PipelineContext;
use crate::pipelines::keys::context_keys::{EVENTS_COUNT_KEY, EVENT_LOG_KEY, EVENT_NAME_KEY, REGEX_KEY};
use crate::pipelines::pipeline_parts::PipelineParts;
use crate::utils::user_data::user_data::UserDataImpl;
use crate::{
    event_log::core::{event_log::EventLog, trace::trace::Trace},
    features::mutations::{
        filtering::{filter_log_by_name, filter_log_by_regex},
        split::get_traces_groups_indices,
    },
};

impl PipelineParts {
    pub(super) fn filter_log_by_event_name() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::FILTER_EVENTS_BY_NAME, &|context, _, config| {
            let log = Self::get_user_data_mut(context, &EVENT_LOG_KEY)?;
            let event_name = Self::get_user_data(config, &EVENT_NAME_KEY)?;
            filter_log_by_name(log, &event_name);

            Ok(())
        })
    }

    pub(super) fn filter_log_by_regex() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::FILTER_EVENTS_BY_REGEX, &|context, _, config| {
            Self::filter_log_by_regex_internal(context, config, |log, regex| filter_log_by_regex(log, regex))
        })
    }

    fn filter_log_by_regex_internal(
        context: &mut PipelineContext,
        config: &UserDataImpl,
        filtering_func: impl Fn(&mut XesEventLogImpl, &Regex),
    ) -> Result<(), PipelinePartExecutionError> {
        let log = Self::get_user_data_mut(context, &EVENT_LOG_KEY)?;
        let regex = Self::get_user_data(config, &REGEX_KEY)?;

        match Regex::new(&regex) {
            Ok(regex) => {
                filtering_func(log, &regex);
                Ok(())
            }
            Err(err) => Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(err.to_string()))),
        }
    }

    pub(super) fn remain_events_by_regex() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::REMAIN_EVENTS_BY_REGEX, &|context, _, config| {
            Self::filter_log_by_regex_internal(context, config, |log, regex| remain_events_in_event_log(log, regex))
        })
    }

    pub(super) fn filter_log_by_variants() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::FILTER_LOG_BY_VARIANTS, &|context, _, _| {
            let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
            let groups_indices: HashSet<usize> = get_traces_groups_indices(log)
                .into_iter()
                .map(|group| *(group.first().unwrap()))
                .collect();

            let log = Self::get_user_data_mut(context, &EVENT_LOG_KEY)?;
            log.filter_traces(&|_, index| !groups_indices.contains(&index));

            Ok(())
        })
    }

    pub(super) fn filter_traces_by_count() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::FILTER_TRACES_BY_EVENTS_COUNT, &|context, _, config| {
            let log = Self::get_user_data_mut(context, &EVENT_LOG_KEY)?;
            let min_events_count = *Self::get_user_data(config, &EVENTS_COUNT_KEY)? as usize;
            log.filter_traces(&|trace, _| trace.events().len() < min_events_count);

            Ok(())
        })
    }
}
