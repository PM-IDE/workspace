use crate::features::cases::cases_discovery::discover_cases;
use crate::pipelines::keys::context_keys::{END_CASE_REGEX, EVENT_LOG_KEY, INLINE_INNER_CASES, PIPELINE_KEY, START_CASE_REGEX};
use crate::pipelines::pipeline_parts::PipelineParts;
use crate::pipelines::pipelines::{PipelinePart, PipelinePartFactory};
use crate::utils::user_data::user_data::UserData;

impl PipelineParts {
    pub(super) fn discover_cases() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DISCOVER_CASES, &|context, infra, config| {
            let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
            let pipeline = Self::get_user_data(config, &PIPELINE_KEY)?;
            let start_case_regex = Self::get_user_data(config, &START_CASE_REGEX)?;
            let end_case_regex = Self::get_user_data(config, &END_CASE_REGEX)?;
            let inline_inner_cases = *Self::get_user_data(config, &INLINE_INNER_CASES)?;

            let new_log = discover_cases(log, start_case_regex.as_str(), end_case_regex.as_str(), inline_inner_cases);

            let mut new_context = context.clone();
            new_context.put_concrete(EVENT_LOG_KEY.key(), new_log);

            pipeline.execute(&mut new_context, infra)?;

            Ok(())
        })
    }
}
