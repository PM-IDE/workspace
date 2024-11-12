use std::{cell::RefCell, rc::Rc};

use chrono::{DateTime, Duration, Utc};

use super::pipelines::PipelinePartFactory;
use crate::pipelines::keys::context_keys::{
    EVENT_CLASS_REGEX_KEY, EVENT_LOG_INFO_KEY, EVENT_LOG_KEY, HASHES_EVENT_LOG_KEY, NAMES_EVENT_LOG_KEY, PIPELINE_KEY,
};
use crate::pipelines::pipeline_parts::PipelineParts;
use crate::pipelines::pipelines::PipelinePart;
use crate::{
    event_log::{
        core::{
            event::{
                event::Event,
                event_hasher::{NameEventHasher, RegexEventHasher},
            },
            event_log::EventLog,
            trace::trace::Trace,
        },
        xes::{xes_event::XesEventImpl, xes_event_log::XesEventLogImpl, xes_trace::XesTraceImpl},
    },
    features::analysis::event_log_info::{EventLogInfo, EventLogInfoCreationDto},
    utils::user_data::user_data::{UserData, UserDataImpl},
};

impl PipelineParts {
    pub(super) fn create_hashed_event_log(config: &UserDataImpl, log: &XesEventLogImpl) -> Vec<Vec<u64>> {
        match Self::get_user_data(config, &EVENT_CLASS_REGEX_KEY) {
            Ok(regex) => {
                let hasher = RegexEventHasher::new(regex).ok().unwrap();
                log.to_hashes_event_log(&hasher)
            }
            Err(_) => log.to_hashes_event_log(&NameEventHasher::new()),
        }
    }

    pub(super) fn get_event_log_info() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::GET_EVENT_LOG_INFO, &|context, _, _| {
            let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
            let log_info = EventLogInfo::create_from(EventLogInfoCreationDto::default(log));
            context.put_concrete(EVENT_LOG_INFO_KEY.key(), log_info);

            Ok(())
        })
    }

    pub(super) fn get_hashes_event_log() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::GET_HASHES_EVENT_LOG, &|context, _, config| {
            let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
            let hashes_event_log = Self::create_hashed_event_log(config, log);

            context.put_concrete(HASHES_EVENT_LOG_KEY.key(), hashes_event_log);

            Ok(())
        })
    }

    pub(super) fn get_names_event_log() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::GET_NAMES_EVENT_LOG, &|context, _, _| {
            let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;

            let mut result = vec![];
            for trace in log.traces() {
                let mut vec = vec![];
                for event in trace.borrow().events() {
                    vec.push(event.borrow().name().to_string());
                }

                result.push(vec);
            }

            context.put_concrete(NAMES_EVENT_LOG_KEY.key(), result);

            Ok(())
        })
    }

    pub(super) fn use_names_event_log() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::USE_NAMES_EVENT_LOG, &|context, _, _| {
            let names_log = Self::get_user_data(context, &NAMES_EVENT_LOG_KEY)?;
            let mut log = XesEventLogImpl::empty();
            for names_trace in names_log {
                let mut trace = XesTraceImpl::empty();
                let mut date = DateTime::<Utc>::MIN_UTC;

                for name in names_trace {
                    let event = XesEventImpl::new(name.clone(), date.clone());
                    trace.push(Rc::new(RefCell::new(event)));
                    date = date + Duration::seconds(1);
                }

                log.push(Rc::new(RefCell::new(trace)));
            }

            context.put_concrete::<XesEventLogImpl>(EVENT_LOG_KEY.key(), log);

            Ok(())
        })
    }

    pub(super) fn execute_frontend_pipeline() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::EXECUTE_FRONTEND_PIPELINE, &|context, infra, config| {
            let pipeline = Self::get_user_data(config, &PIPELINE_KEY)?;
            pipeline.execute(context, infra)?;

            Ok(())
        })
    }
}
