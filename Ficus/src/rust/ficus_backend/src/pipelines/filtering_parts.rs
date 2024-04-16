use std::collections::HashSet;

use fancy_regex::Regex;

use crate::pipelines::pipeline_parts::PipelineParts;
use crate::{
    event_log::core::{event_log::EventLog, trace::trace::Trace},
    features::mutations::{
        filtering::{filter_log_by_name, filter_log_by_regex},
        split::get_traces_groups_indices,
    },
};

use super::{
    errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError},
    pipelines::PipelinePartFactory,
};

impl PipelineParts {
    pub(super) fn filter_log_by_event_name() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::FILTER_EVENTS_BY_NAME, &|context, _, keys, config| {
            let log = Self::get_user_data_mut(context, keys.event_log())?;
            let event_name = Self::get_user_data(config, keys.event_name())?;
            filter_log_by_name(log, &event_name);

            Ok(())
        })
    }

    pub(super) fn filter_log_by_regex() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::FILTER_EVENTS_BY_REGEX, &|context, _, keys, config| {
            let log = Self::get_user_data_mut(context, keys.event_log())?;
            let regex = Self::get_user_data(config, keys.regex())?;

            match Regex::new(&regex) {
                Ok(regex) => {
                    filter_log_by_regex(log, &regex);
                    Ok(())
                }
                Err(err) => Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(err.to_string()))),
            }
        })
    }

    pub(super) fn filter_log_by_variants() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::FILTER_LOG_BY_VARIANTS, &|context, _, keys, _| {
            let log = Self::get_user_data(context, keys.event_log())?;
            let groups_indices: HashSet<usize> = get_traces_groups_indices(log)
                .into_iter()
                .map(|group| *(group.first().unwrap()))
                .collect();

            let log = Self::get_user_data_mut(context, keys.event_log())?;
            log.filter_traces(&|_, index| !groups_indices.contains(&index));

            Ok(())
        })
    }

    pub(super) fn filter_traces_by_count() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::FILTER_TRACES_BY_EVENTS_COUNT, &|context, _, keys, config| {
            let log = Self::get_user_data_mut(context, keys.event_log())?;
            let min_events_count = *Self::get_user_data(config, keys.events_count())? as usize;
            log.filter_traces(&|trace, _| trace.events().len() < min_events_count);

            Ok(())
        })
    }
}
