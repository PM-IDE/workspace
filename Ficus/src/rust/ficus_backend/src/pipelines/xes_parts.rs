use crate::event_log::bxes::bxes_to_xes_converter::read_bxes_into_xes_log;
use crate::event_log::bxes::xes_to_bxes_converter::write_event_log_to_bxes;
use crate::pipelines::pipeline_parts::PipelineParts;
use crate::{
    event_log::xes::{reader::file_xes_log_reader::read_event_log, writer::xes_event_log_writer::write_xes_log},
    utils::user_data::user_data::UserData,
};

use super::{
    errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError},
    pipelines::PipelinePartFactory,
};

impl PipelineParts {
    pub(super) fn write_log_to_xes() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::WRITE_LOG_TO_XES, &|context, _, keys, config| {
            let path = Self::get_user_data(config, &keys.path())?;
            match write_xes_log(&context.concrete(&keys.event_log().key()).unwrap(), path) {
                Ok(()) => Ok(()),
                Err(err) => Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(err.to_string()))),
            }
        })
    }

    pub(super) fn read_log_from_xes() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::READ_LOG_FROM_XES, &|context, infra, keys, _| {
            let path = Self::get_user_data(context, keys.path())?;

            let log = read_event_log(path);
            if log.is_none() {
                let message = format!("Failed to read event log from {}", path.as_str());
                return Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(message)));
            }

            context.put_concrete(keys.event_log().key(), log.unwrap());
            Ok(())
        })
    }

    pub(super) fn read_log_from_bxes() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::READ_LOG_FROM_BXES, &|context, _, keys, _| {
            let path = Self::get_user_data(context, keys.path())?;

            match read_bxes_into_xes_log(path) {
                Ok(result) => {
                    context.put_concrete(keys.event_log().key(), result.xes_log);
                    context.put_concrete(keys.system_metadata().key(), result.system_metadata);

                    Ok(())
                }
                Err(err) => {
                    let message = format!("Failed to read event log from {}, error: {}", path.as_str(), err.to_string());
                    Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(message)))
                }
            }
        })
    }

    pub(super) fn write_log_to_bxes() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::WRITE_LOG_TO_BXES, &|context, _, keys, config| {
            let path = Self::get_user_data(config, keys.path())?;
            let log = Self::get_user_data(context, keys.event_log())?;
            let system_metadata = match Self::get_user_data(context, keys.system_metadata()) {
                Ok(metadata) => Some(metadata),
                Err(_) => None,
            };

            match write_event_log_to_bxes(log, system_metadata, path) {
                Ok(_) => Ok(()),
                Err(err) => Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(err.to_string()))),
            }
        })
    }
}
