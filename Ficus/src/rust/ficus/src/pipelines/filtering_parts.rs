use std::collections::HashSet;

use fancy_regex::Regex;

use super::{
  errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError},
  pipelines::PipelinePartFactory,
};
use crate::{
  event_log::{
    core::{event_log::EventLog, trace::trace::Trace},
    xes::xes_event_log::XesEventLogImpl,
  },
  features::mutations::{
    filtering::{filter_log_by_name, filter_log_by_regex, remain_events_in_event_log},
    split::get_traces_groups_indices,
  },
  pipeline_part,
  pipelines::{
    context::PipelineContext,
    keys::context_keys::{EVENT_LOG_KEY, EVENT_NAME_KEY, EVENTS_COUNT_KEY, REGEX_KEY},
    pipeline_parts::PipelineParts,
  },
  utils::user_data::user_data::UserDataImpl,
};

impl PipelineParts {
  pipeline_part!(filter_events_by_name, |context: &mut PipelineContext, _, config: &UserDataImpl| {
    let log = Self::get_user_data_mut(context, &EVENT_LOG_KEY)?;
    let event_name = Self::get_user_data(config, &EVENT_NAME_KEY)?;
    filter_log_by_name(log, event_name);

    Ok(())
  });

  pipeline_part!(filter_events_by_regex, |context: &mut PipelineContext, _, config: &UserDataImpl| {
    Self::filter_log_by_regex_internal(context, config, filter_log_by_regex)
  });

  fn filter_log_by_regex_internal(
    context: &mut PipelineContext,
    config: &UserDataImpl,
    filtering_func: impl Fn(&mut XesEventLogImpl, &Regex),
  ) -> Result<(), PipelinePartExecutionError> {
    let log = Self::get_user_data_mut(context, &EVENT_LOG_KEY)?;
    let regex = Self::get_user_data(config, &REGEX_KEY)?;

    match Regex::new(regex) {
      Ok(regex) => {
        filtering_func(log, &regex);
        Ok(())
      }
      Err(err) => Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(err.to_string()))),
    }
  }

  pipeline_part!(remain_events_by_regex, |context: &mut PipelineContext, _, config: &UserDataImpl| {
    Self::filter_log_by_regex_internal(context, config, remain_events_in_event_log)
  });

  pipeline_part!(filter_log_by_variants, |context: &mut PipelineContext, _, _| {
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
    let groups_indices: HashSet<usize> = get_traces_groups_indices(log)
      .into_iter()
      .map(|group| *(group.first().unwrap()))
      .collect();

    let log = Self::get_user_data_mut(context, &EVENT_LOG_KEY)?;
    log.filter_traces(&|_, index| !groups_indices.contains(index));

    Ok(())
  });

  pipeline_part!(
    filter_traces_by_events_count,
    |context: &mut PipelineContext, _, config: &UserDataImpl| {
      let log = Self::get_user_data_mut(context, &EVENT_LOG_KEY)?;
      let min_events_count = *Self::get_user_data(config, &EVENTS_COUNT_KEY)? as usize;
      log.filter_traces(&|trace, _| trace.events().len() < min_events_count);

      Ok(())
    }
  );
}
