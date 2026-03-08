use crate::{
  features::cases::cases_discovery::discover_cases,
  pipeline_part,
  pipelines::{
    context::{PipelineContext, PipelineInfrastructure},
    keys::context_keys::{END_CASE_REGEX_KEY, EVENT_LOG_KEY, INLINE_INNER_CASES_KEY, PIPELINE_KEY, START_CASE_REGEX_KEY},
    pipeline_parts::PipelineParts,
    pipelines::{PipelinePart, PipelinePartFactory},
  },
  utils::user_data::user_data::{UserData, UserDataImpl},
};

impl PipelineParts {
  pipeline_part!(discover_cases, |context: &mut PipelineContext,
                                  infra: &PipelineInfrastructure,
                                  config: &UserDataImpl| {
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
    let pipeline = Self::get_user_data(config, &PIPELINE_KEY)?;
    let start_case_regex = Self::get_user_data(config, &START_CASE_REGEX_KEY)?;
    let end_case_regex = Self::get_user_data(config, &END_CASE_REGEX_KEY)?;
    let inline_inner_cases = *Self::get_user_data(config, &INLINE_INNER_CASES_KEY)?;

    let new_log = discover_cases(log, start_case_regex.as_ref(), end_case_regex.as_ref(), inline_inner_cases);

    let mut new_context = context.clone();
    new_context.put_concrete(EVENT_LOG_KEY.key(), new_log);

    pipeline.execute(&mut new_context, infra)?;

    Ok(())
  });
}
