use std::{cell::RefCell, rc::Rc};

use chrono::{DateTime, Duration, Utc};

use super::pipelines::PipelinePartFactory;
use crate::features::analysis::log_info::event_log_info::OfflineEventLogInfo;
use crate::features::analysis::log_info::log_info_creation_dto::EventLogInfoCreationDto;
use crate::pipelines::keys::context_keys::{EVENT_CLASS_REGEX_KEY, EVENT_LOG_INFO_KEY, EVENT_LOG_KEY, GRAPH, GRAPHS, GRAPHS_KEY, GRAPH_KEY, HASHES_EVENT_LOG_KEY, NAMES_EVENT_LOG_KEY, PIPELINE_KEY};
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
  utils::user_data::user_data::{UserData, UserDataImpl},
};
use crate::pipelines::errors::pipeline_errors::PipelinePartExecutionError;
use crate::utils::graph::graph::DefaultGraph;
use crate::utils::graph::graphs_merging::merge_graphs;

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
      let log_info = OfflineEventLogInfo::create_from(EventLogInfoCreationDto::default(log));
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

  pub(super) fn merge_graphs() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::MERGE_GRAPHS, &|context, _, _| {
      let graphs = Self::get_user_data(context, &GRAPHS_KEY)?;

      let graph = merge_graphs(graphs).map_err(|e| PipelinePartExecutionError::new_raw(e.to_string()))?;
      context.put_concrete(GRAPH_KEY.key(), graph);

      Ok(())
    })
  }

  pub(super) fn add_graph_to_graphs() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::ADD_GRAPH_TO_GRAPHS, &|context, _, _| {
      let graph = Self::get_user_data(context, &GRAPH_KEY)?.clone();

      match Self::get_user_data_mut(context, &GRAPHS_KEY).ok() {
        None => context.put_concrete(GRAPHS_KEY.key(), vec![graph]),
        Some(graphs) => graphs.push(graph)
      }

      Ok(())
    })
  }
  
  pub(super) fn clear_graphs() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::CLEAR_GRAPHS, &|context, _, _| {
      if let Some(graphs) = Self::get_user_data_mut(context, &GRAPHS_KEY).ok() {
        graphs.clear();
      }

      Ok(())
    })
  }

  pub(super) fn terminate_if_empty_log() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::TERMINATE_IF_EMPTY_LOG, &|context, _, _| {
      let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
      if log.traces().iter().map(|t| t.borrow().events().len()).sum::<usize>() == 0 {
        return Err(PipelinePartExecutionError::new_raw("Empty log".to_string()))
      }

      Ok(())
    })
  }
}
