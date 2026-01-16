use crate::{
  features::mutations::mutations::{add_artificial_start_end_activities, append_attributes_to_name},
  pipeline_part,
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
  pipeline_part!(
    add_artificial_start_end_events,
    |context: &mut PipelineContext, _, config: &UserDataImpl| { Self::create_add_start_end_events_internal(context, config, true, true) }
  );

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

  pipeline_part!(
    add_artificial_start_events,
    |context: &mut PipelineContext, _, config: &UserDataImpl| { Self::create_add_start_end_events_internal(context, config, true, false) }
  );

  pipeline_part!(
    add_artificial_end_events,
    |context: &mut PipelineContext, _, config: &UserDataImpl| { Self::create_add_start_end_events_internal(context, config, false, true) }
  );

  pipeline_part!(
    append_attributes_to_name,
    |context: &mut PipelineContext, _, config: &UserDataImpl| {
      let log = Self::get_user_data_mut(context, &EVENT_LOG_KEY)?;
      let attributes = Self::get_user_data(config, &ATTRIBUTES_KEY)?;

      append_attributes_to_name(log, attributes);

      Ok(())
    }
  );
}
