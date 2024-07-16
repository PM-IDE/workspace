use crate::event_log::bxes::bxes_to_xes_converter::{read_bxes_into_xes_log, read_bxes_into_xes_log_from_bytes, BxesToXesConversionResult};
use crate::event_log::bxes::xes_to_bxes_converter::{write_event_log_to_bxes, write_event_log_to_bxes_bytes};
use crate::event_log::xes::reader::file_xes_log_reader::read_event_log_from_bytes;
use crate::event_log::xes::writer::xes_event_log_writer::write_xes_log_to_bytes;
use crate::pipelines::context::PipelineContext;
use crate::pipelines::pipeline_parts::PipelineParts;
use crate::{
    event_log::xes::{reader::file_xes_log_reader::read_event_log, writer::xes_event_log_writer::write_xes_log},
    utils::user_data::user_data::UserData,
};
use crate::pipelines::keys::context_keys::{BYTES_KEY, EVENT_LOG_KEY, PATH, PATH_KEY, SYSTEM_METADATA, SYSTEM_METADATA_KEY};
use super::{
    errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError},
    pipelines::PipelinePartFactory,
};

impl PipelineParts {
    pub(super) fn write_log_to_xes() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::WRITE_LOG_TO_XES, &|context, _, config| {
            let path = Self::get_user_data(config, &PATH_KEY)?;
            match write_xes_log(&context.concrete(EVENT_LOG_KEY.key()).unwrap(), path) {
                Ok(()) => Ok(()),
                Err(err) => Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(err.to_string()))),
            }
        })
    }

    pub(super) fn read_log_from_xes() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::READ_LOG_FROM_XES, &|context, infra, _| {
            let path = Self::get_user_data(context, &PATH_KEY)?;

            let log = read_event_log(path);
            if log.is_none() {
                let message = format!("Failed to read event log from {}", path.as_str());
                return Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(message)));
            }

            context.put_concrete(EVENT_LOG_KEY.key(), log.unwrap());
            Ok(())
        })
    }

    pub(super) fn read_log_from_bxes() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::READ_LOG_FROM_BXES, &|context, _, _| {
            let path = Self::get_user_data(context, &PATH_KEY)?;

            match read_bxes_into_xes_log(path) {
                Ok(result) => {
                    Self::put_read_result_to_context(context, result);
                    Ok(())
                }
                Err(err) => {
                    let message = format!("Failed to read event log from {}, error: {}", path.as_str(), err.to_string());
                    Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(message)))
                }
            }
        })
    }

    fn put_read_result_to_context(context: &mut PipelineContext, result: BxesToXesConversionResult) {
        context.put_concrete(EVENT_LOG_KEY.key(), result.xes_log);
        context.put_concrete(SYSTEM_METADATA_KEY.key(), result.system_metadata);
    }

    pub(super) fn write_log_to_bxes() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::WRITE_LOG_TO_BXES, &|context, _, config| {
            let path = Self::get_user_data(config, &PATH_KEY)?;
            let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
            let system_metadata = match Self::get_user_data(context, &SYSTEM_METADATA_KEY) {
                Ok(metadata) => Some(metadata),
                Err(_) => None,
            };

            match write_event_log_to_bxes(log, system_metadata, path) {
                Ok(_) => Ok(()),
                Err(err) => Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(err.to_string()))),
            }
        })
    }

    pub(super) fn read_xes_from_bytes() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::READ_XES_LOG_FROM_BYTES, &|context, _, config| {
            let bytes = Self::get_user_data(context, &BYTES_KEY)?;
            match read_event_log_from_bytes(bytes) {
                Some(log) => {
                    context.put_concrete(EVENT_LOG_KEY.key(), log);
                    Ok(())
                }
                None => {
                    let message = "Failed to read event log from bytes array".to_string();
                    return Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(message)));
                }
            }
        })
    }

    pub(super) fn read_bxes_from_bytes() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::READ_BXES_LOG_FROM_BYTES, &|context, _, config| {
            let bytes = Self::get_user_data(context, &BYTES_KEY)?;
            match read_bxes_into_xes_log_from_bytes(bytes) {
                Ok(read_result) => {
                    Self::put_read_result_to_context(context, read_result);
                    Ok(())
                }
                Err(err) => {
                    let message = format!("Failed to read event log from bytes: {}", err.to_string());
                    Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(message)))
                }
            }
        })
    }

    pub(super) fn write_bxes_to_bytes() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::WRITE_BXES_LOG_TO_BYTES, &|context, _, config| {
            let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
            let system_metadata = match Self::get_user_data(context, &SYSTEM_METADATA_KEY) {
                Ok(metadata) => Some(metadata),
                Err(_) => None,
            };

            match write_event_log_to_bxes_bytes(log, system_metadata) {
                Ok(bytes) => {
                    context.put_concrete::<Vec<u8>>(BYTES_KEY.key(), bytes);
                    Ok(())
                }
                Err(err) => Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(err.to_string()))),
            }
        })
    }

    pub(super) fn write_xes_to_bytes() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::WRITE_XES_LOG_TO_BYTES, &|context, _, config| {
            let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
            match write_xes_log_to_bytes(log) {
                Ok(bytes) => {
                    context.put_concrete::<Vec<u8>>(&BYTES_KEY.key(), bytes);
                    Ok(())
                }
                Err(err) => Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(err.to_string()))),
            }
        })
    }
}
