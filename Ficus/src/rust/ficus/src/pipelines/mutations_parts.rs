use crate::{
  features::mutations::mutations::{add_artificial_start_end_activities, append_attributes_to_name},
  pipelines::{
    context::PipelineContext,
    errors::pipeline_errors::PipelinePartExecutionError,
    keys::context_keys::{ATTRIBUTES_KEY, EVENT_LOG_KEY},
    pipeline_parts::PipelineParts,
    pipelines::PipelinePartFactory,
  },
  utils::user_data::user_data::UserDataImpl,
};

impl PipelineParts {
  pub(super) fn add_artificial_start_end_events() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::ADD_ARTIFICIAL_START_END_EVENTS, &|context, _, config| {
      Self::create_add_start_end_events_internal(context, config, true, true)
    })
  }

  fn create_add_start_end_events_internal(
    context: &mut PipelineContext,
    config: &UserDataImpl,
    add_start_events: bool,
    add_end_events: bool,
  ) -> Result<(), PipelinePartExecutionError> {
    let log = Self::get_user_data_mut(context, &EVENT_LOG_KEY)?;
    let attributes_to_copy = match Self::get_user_data(config, &ATTRIBUTES_KEY) {
      Ok(attributes_to_copy) => Some(attributes_to_copy.iter().map(|a| a.clone()).collect()),
      Err(_) => None,
    };

    add_artificial_start_end_activities(log, add_start_events, add_end_events, attributes_to_copy.as_ref());

    Ok(())
  }

  pub(super) fn add_artificial_start_events() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::ADD_ARTIFICIAL_START_EVENTS, &|context, _, config| {
      Self::create_add_start_end_events_internal(context, config, true, false)
    })
  }

  pub(super) fn add_artificial_end_events() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::ADD_ARTIFICIAL_END_EVENTS, &|context, _, config| {
      Self::create_add_start_end_events_internal(context, config, false, true)
    })
  }

  pub(super) fn append_attributes_to_name() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::APPEND_ATTRIBUTES_TO_NAME, &|context, _, config| {
      let log = Self::get_user_data_mut(context, &EVENT_LOG_KEY)?;
      let attributes = Self::get_user_data(config, &ATTRIBUTES_KEY)?;

      append_attributes_to_name(log, attributes);

      Ok(())
    })
  }
}
