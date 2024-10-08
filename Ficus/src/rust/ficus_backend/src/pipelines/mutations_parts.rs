use crate::features::mutations::mutations::add_artificial_start_end_activities;
use crate::pipelines::context::PipelineContext;
use crate::pipelines::errors::pipeline_errors::PipelinePartExecutionError;
use crate::pipelines::keys::context_keys::EVENT_LOG_KEY;
use crate::pipelines::pipeline_parts::PipelineParts;
use crate::pipelines::pipelines::PipelinePartFactory;

impl PipelineParts {
    pub(super) fn add_artificial_start_end_events() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::ADD_ARTIFICIAL_START_END_EVENTS, &|context, _, _| {
            Self::create_add_start_end_events_internal(context, true, true)
        })
    }

    fn create_add_start_end_events_internal(
        context: &mut PipelineContext,
        add_start_events: bool,
        add_end_events: bool,
    ) -> Result<(), PipelinePartExecutionError> {
        let log = Self::get_user_data_mut(context, &EVENT_LOG_KEY)?;
        add_artificial_start_end_activities(log, add_start_events, add_end_events);

        Ok(())
    }

    pub(super) fn add_artificial_start_events() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::ADD_ARTIFICIAL_START_EVENTS, &|context, _, _| {
            Self::create_add_start_end_events_internal(context, true, false)
        })
    }

    pub(super) fn add_artificial_end_events() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::ADD_ARTIFICIAL_END_EVENTS, &|context, _, _| {
            Self::create_add_start_end_events_internal(context, false, true)
        })
    }
}
