use std::{cell::RefCell, rc::Rc};

use chrono::{DateTime, Duration, Utc};

use super::pipelines::PipelinePartFactory;
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
  features::analysis::log_info::{event_log_info::OfflineEventLogInfo, log_info_creation_dto::EventLogInfoCreationDto},
  pipeline_part,
  pipelines::{
    context::{PipelineContext, PipelineInfrastructure},
    errors::pipeline_errors::PipelinePartExecutionError,
    keys::context_keys::{
      EVENT_CLASS_REGEX_KEY, EVENT_LOG_INFO_KEY, EVENT_LOG_KEY, GRAPH_KEY, GRAPHS_KEY, HASHES_EVENT_LOG_KEY, NAMES_EVENT_LOG_KEY,
      PIPELINE_KEY,
    },
    pipeline_parts::PipelineParts,
    pipelines::PipelinePart,
  },
  utils::{
    graph::graphs_merging::merge_graphs,
    user_data::user_data::{UserData, UserDataImpl},
  },
};

impl PipelineParts {
  pub(super) fn create_hashed_event_log(config: &UserDataImpl, log: &XesEventLogImpl) -> Vec<Vec<u64>> {
    match Self::get_user_data(config, &EVENT_CLASS_REGEX_KEY) {
      Ok(regex) => {
        let hasher = RegexEventHasher::new(regex).ok().unwrap();
        log.to_hashes_event_log(&hasher)
      }
      Err(_) => log.to_hashes_event_log(&NameEventHasher),
    }
  }

  pipeline_part!(get_event_log_info, |context: &mut PipelineContext, _, _| {
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
    let log_info = OfflineEventLogInfo::create_from(EventLogInfoCreationDto::default(log));
    context.put_concrete(EVENT_LOG_INFO_KEY.key(), log_info);

    Ok(())
  });

  pipeline_part!(get_hashes_event_log, |context: &mut PipelineContext, _, config: &UserDataImpl| {
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
    let hashes_event_log = Self::create_hashed_event_log(config, log);

    context.put_concrete(HASHES_EVENT_LOG_KEY.key(), hashes_event_log);

    Ok(())
  });

  pipeline_part!(get_names_event_log, |context: &mut PipelineContext, _, _| {
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;

    let mut result = vec![];
    for trace in log.traces() {
      let mut vec = vec![];
      for event in trace.borrow().events() {
        vec.push(event.borrow().name_pointer().clone());
      }

      result.push(vec);
    }

    context.put_concrete(NAMES_EVENT_LOG_KEY.key(), result);

    Ok(())
  });

  pipeline_part!(use_names_event_log, |context: &mut PipelineContext, _, _| {
    let names_log = Self::get_user_data(context, &NAMES_EVENT_LOG_KEY)?;
    let mut log = XesEventLogImpl::empty();
    for names_trace in names_log {
      let mut trace = XesTraceImpl::empty();
      let mut date = DateTime::<Utc>::MIN_UTC;

      for name in names_trace {
        let event = XesEventImpl::new(name.clone(), date);
        trace.push(Rc::new(RefCell::new(event)));
        date += Duration::seconds(1);
      }

      log.push(Rc::new(RefCell::new(trace)));
    }

    context.put_concrete::<XesEventLogImpl>(EVENT_LOG_KEY.key(), log);

    Ok(())
  });

  pipeline_part!(
    execute_frontend_pipeline,
    |context: &mut PipelineContext, infra: &PipelineInfrastructure, config: &UserDataImpl| {
      let pipeline = Self::get_user_data(config, &PIPELINE_KEY)?;
      pipeline.execute(context, infra)?;

      Ok(())
    }
  );

  pipeline_part!(merge_graphs, |context: &mut PipelineContext, _, _| {
    let graphs = Self::get_user_data(context, &GRAPHS_KEY)?;

    let graph = merge_graphs(graphs).map_err(|e| PipelinePartExecutionError::new_raw(e.to_string()))?;
    context.put_concrete(GRAPH_KEY.key(), graph);

    Ok(())
  });

  pipeline_part!(add_graph_to_graphs, |context: &mut PipelineContext, _, _| {
    let graph = Self::get_user_data(context, &GRAPH_KEY)?.clone();

    match Self::get_user_data_mut(context, &GRAPHS_KEY).ok() {
      None => context.put_concrete(GRAPHS_KEY.key(), vec![graph]),
      Some(graphs) => graphs.push(graph),
    }

    Ok(())
  });

  pipeline_part!(clear_graphs, |context: &mut PipelineContext, _, _| {
    if let Ok(graphs) = Self::get_user_data_mut(context, &GRAPHS_KEY) {
      graphs.clear();
    }

    Ok(())
  });

  pipeline_part!(terminate_if_empty_log, |context: &mut PipelineContext, _, _| {
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
    if log.traces().iter().map(|t| t.borrow().events().len()).sum::<usize>() == 0 {
      return Err(PipelinePartExecutionError::new_raw("Empty log".to_string()));
    }

    Ok(())
  });
}
