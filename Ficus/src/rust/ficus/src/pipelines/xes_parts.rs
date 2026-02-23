use super::{
  errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError},
  pipelines::PipelinePartFactory,
};
use crate::{
  event_log::{
    bxes::{
      bxes_to_xes_converter::{BxesToXesConversionResult, read_bxes_into_xes_log, read_bxes_into_xes_log_from_bytes},
      xes_to_bxes_converter::{write_event_log_to_bxes, write_event_log_to_bxes_bytes},
    },
    xes::{
      logs_merger::merge_xes_logs,
      reader::file_xes_log_reader::{read_event_log, read_event_log_from_bytes},
      writer::xes_event_log_writer::{write_xes_log, write_xes_log_to_bytes},
    },
  },
  pipeline_part,
  pipelines::{
    context::PipelineContext,
    keys::context_keys::{BYTES_KEY, EVENT_LOG_KEY, PATH_KEY, PATHS_KEY, SYSTEM_METADATA_KEY},
    pipeline_parts::PipelineParts,
  },
  utils::user_data::user_data::{UserData, UserDataImpl},
};

impl PipelineParts {
  pipeline_part!(write_log_to_xes, |context: &mut PipelineContext, _, config: &UserDataImpl| {
    let path = Self::get_user_data(config, &PATH_KEY)?;
    match write_xes_log(context.concrete(EVENT_LOG_KEY.key()).unwrap(), path) {
      Ok(()) => Ok(()),
      Err(err) => Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(err.to_string()))),
    }
  });

  pipeline_part!(read_log_from_xes, |context: &mut PipelineContext, _, _| {
    let path = Self::get_user_data(context, &PATH_KEY)?;

    let log = read_event_log(path);
    if log.is_none() {
      let message = format!("Failed to read event log from {}", path.as_str());
      return Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(message)));
    }

    context.put_concrete(EVENT_LOG_KEY.key(), log.unwrap());
    Ok(())
  });

  pipeline_part!(read_log_from_bxes, |context: &mut PipelineContext, _, _| {
    let path = Self::get_user_data(context, &PATH_KEY)?;

    match read_bxes_into_xes_log(path) {
      Ok(result) => {
        Self::put_read_result_to_context(context, result);
        Ok(())
      }
      Err(err) => {
        let message = format!("Failed to read event log from {}, error: {}", path.as_str(), err);
        Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(message)))
      }
    }
  });

  fn put_read_result_to_context(context: &mut PipelineContext, result: BxesToXesConversionResult) {
    context.put_concrete(EVENT_LOG_KEY.key(), result.xes_log);
    context.put_concrete(SYSTEM_METADATA_KEY.key(), result.system_metadata);
  }

  pipeline_part!(write_log_to_bxes, |context: &mut PipelineContext, _, config: &UserDataImpl| {
    let path = Self::get_user_data(config, &PATH_KEY)?;
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
    let system_metadata = Self::get_user_data(context, &SYSTEM_METADATA_KEY).ok();

    match write_event_log_to_bxes(log, system_metadata, path) {
      Ok(_) => Ok(()),
      Err(err) => Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(err.to_string()))),
    }
  });

  pipeline_part!(read_xes_log_from_bytes, |context: &mut PipelineContext, _, _| {
    let bytes = Self::get_user_data(context, &BYTES_KEY)?;
    match read_event_log_from_bytes(bytes) {
      Some(log) => {
        context.put_concrete(EVENT_LOG_KEY.key(), log);
        Ok(())
      }
      None => {
        let message = "Failed to read event log from bytes array".to_string();
        Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(message)))
      }
    }
  });

  pipeline_part!(read_bxes_log_from_bytes, |context: &mut PipelineContext, _, _| {
    let bytes = Self::get_user_data(context, &BYTES_KEY)?;
    match read_bxes_into_xes_log_from_bytes(bytes) {
      Ok(read_result) => {
        Self::put_read_result_to_context(context, read_result);
        Ok(())
      }
      Err(err) => {
        let message = format!("Failed to read event log from bytes: {}", err);
        Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(message)))
      }
    }
  });

  pipeline_part!(write_bxes_log_to_bytes, |context: &mut PipelineContext, _, _| {
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
    let system_metadata = Self::get_user_data(context, &SYSTEM_METADATA_KEY).ok();

    match write_event_log_to_bxes_bytes(log, system_metadata) {
      Ok(bytes) => {
        context.put_concrete::<Vec<u8>>(BYTES_KEY.key(), bytes);
        Ok(())
      }
      Err(err) => Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(err.to_string()))),
    }
  });

  pipeline_part!(write_xes_log_to_bytes, |context: &mut PipelineContext, _, _| {
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
    match write_xes_log_to_bytes(log) {
      Ok(bytes) => {
        context.put_concrete::<Vec<u8>>(BYTES_KEY.key(), bytes);
        Ok(())
      }
      Err(err) => Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(err.to_string()))),
    }
  });

  pipeline_part!(merge_xes_logs_from_paths, |context: &mut PipelineContext, _, _| {
    let paths = Self::get_user_data(context, &PATHS_KEY)?;
    let log = merge_xes_logs(paths);

    context.put_concrete(EVENT_LOG_KEY.key(), log);

    Ok(())
  });
}
