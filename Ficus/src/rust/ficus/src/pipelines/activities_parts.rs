use super::errors::pipeline_errors::RawPartExecutionError;
use super::{
  aliases::TracesActivities,
  context::PipelineContext,
  errors::pipeline_errors::PipelinePartExecutionError,
  pipelines::{DefaultPipelinePart, PipelinePart, PipelinePartFactory},
};
use crate::event_log::bxes::xes_to_bxes_converter::write_event_log_to_bxes;
use crate::event_log::core::event::event::Event;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::writer::xes_event_log_writer::write_xes_log;
use crate::event_log::xes::xes_trace::XesTraceImpl;
use crate::features::analysis::log_info::event_log_info::count_events;
use crate::features::analysis::patterns::activity_instances;
use crate::features::analysis::patterns::activity_instances::{substitute_underlying_events, ActivitiesLogSource, UNDEF_ACTIVITY_NAME};
use crate::features::analysis::patterns::pattern_info::{UnderlyingPatternKind, UNDERLYING_PATTERN_KIND_KEY};
use crate::pipelines::context::PipelineInfrastructure;
use crate::pipelines::keys::context_keys::{ACTIVITIES_KEY, ACTIVITIES_LOGS_SOURCE_KEY, ACTIVITY_IN_TRACE_FILTER_KIND_KEY, ACTIVITY_LEVEL_KEY, ACTIVITY_NAME_KEY, ADJUSTING_MODE_KEY, EVENTS_COUNT_KEY, EVENT_CLASSES_REGEXES_KEY, EVENT_CLASS_REGEX_KEY, EVENT_LOG_KEY, EXECUTE_ONLY_ON_LAST_EXTRACTION_KEY, HASHES_EVENT_LOG_KEY, LOG_SERIALIZATION_FORMAT_KEY, MIN_ACTIVITY_LENGTH_KEY, NARROW_ACTIVITIES_KEY, PATH_KEY, PATTERNS_DISCOVERY_STRATEGY_KEY, PATTERNS_KEY, PATTERNS_KIND_KEY, PIPELINE_KEY, REGEX_KEY, REPEAT_SETS_KEY, TRACE_ACTIVITIES_KEY, UNDEF_ACTIVITY_HANDLING_STRATEGY_KEY, UNDERLYING_EVENTS_COUNT_KEY};
use crate::pipelines::pipeline_parts::PipelineParts;
use crate::utils::log_serialization_format::LogSerializationFormat;
use crate::{
  event_log::{
    core::event_log::EventLog,
    xes::{xes_event::XesEventImpl, xes_event_log::XesEventLogImpl},
  },
  features::analysis::patterns::{
    activity_instances::{
      add_unattached_activities, count_underlying_events, create_activity_name, create_log_from_unattached_events,
      create_new_log_from_activities_instances, extract_activities_instances, ActivityInTraceInfo, AdjustingMode, SubTraceKind,
      UndefActivityHandlingStrategy,
    },
    repeat_sets::{build_repeat_set_tree_from_repeats, build_repeat_sets},
  },
  utils::user_data::user_data::{UserData, UserDataImpl},
};
use chrono::TimeDelta;
use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;
use std::{cell::RefCell, rc::Rc};

pub enum UndefActivityHandlingStrategyDto {
  DontInsert,
  InsertAsSingleEvent,
  InsertAllEvents,
}

impl FromStr for UndefActivityHandlingStrategyDto {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "DontInsert" => Ok(Self::DontInsert),
      "InsertAsSingleEvent" => Ok(Self::InsertAsSingleEvent),
      "InsertAllEvents" => Ok(Self::InsertAllEvents),
      _ => Err(()),
    }
  }
}

pub enum ActivitiesLogsSourceDto {
  Log,
  TracesActivities,
}

impl FromStr for ActivitiesLogsSourceDto {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Log" => Ok(Self::Log),
      "TracesActivities" => Ok(Self::TracesActivities),
      _ => Err(()),
    }
  }
}

impl PipelineParts {
  pub(super) fn discover_activities() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::DISCOVER_ACTIVITIES, &|context, _, config| {
      let activity_level = Self::get_user_data(config, &ACTIVITY_LEVEL_KEY)?;
      Self::do_discover_activities(context, *activity_level, config)
    })
  }

  pub(super) fn do_discover_activities(
    context: &mut PipelineContext,
    activity_level: u32,
    config: &UserDataImpl,
  ) -> Result<(), PipelinePartExecutionError> {
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
    let patterns = Self::get_user_data(context, &PATTERNS_KEY)?;
    let hashed_log = Self::get_user_data(context, &HASHES_EVENT_LOG_KEY)?;
    let event_class_regex = match Self::get_user_data(config, &EVENT_CLASS_REGEX_KEY) {
      Ok(regex) => Some(regex),
      Err(_) => None,
    };

    let repeat_sets = build_repeat_sets(&hashed_log, patterns);

    let underlying_patterns_kind = Self::get_user_data(context, &UNDERLYING_PATTERN_KIND_KEY).unwrap_or(&UnderlyingPatternKind::Unknown);

    let tree = build_repeat_set_tree_from_repeats(
      &hashed_log,
      &repeat_sets,
      activity_level as usize,
      underlying_patterns_kind.clone(),
      |sub_array| {
        create_activity_name(log, sub_array, event_class_regex)
      }
    );

    context.put_concrete(ACTIVITIES_KEY.key(), tree);
    Ok(())
  }

  pub(super) fn discover_activities_instances() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::DISCOVER_ACTIVITIES_INSTANCES, &|context, _, config| {
      Self::do_discover_activities_instances(context, config)?;
      Ok(())
    })
  }

  pub(super) fn do_discover_activities_instances(
    context: &mut PipelineContext,
    config: &UserDataImpl,
  ) -> Result<(), PipelinePartExecutionError> {
    let mut tree = Self::get_user_data_mut(context, &ACTIVITIES_KEY)?;
    let narrow = Self::get_user_data(config, &NARROW_ACTIVITIES_KEY)?;
    let hashed_log = Self::get_user_data(context, &HASHES_EVENT_LOG_KEY)?;
    let min_events_in_activity = *Self::get_user_data(config, &MIN_ACTIVITY_LENGTH_KEY)?;
    let activity_filter_kind = Self::get_user_data(config, &ACTIVITY_IN_TRACE_FILTER_KIND_KEY)?;

    let instances = extract_activities_instances(
      &hashed_log,
      &mut tree,
      narrow,
      min_events_in_activity as usize,
      activity_filter_kind,
    );

    context.put_concrete(TRACE_ACTIVITIES_KEY.key(), instances);
    Ok(())
  }

  pub(super) fn create_log_from_activities() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::CREATE_LOG_FROM_ACTIVITIES, &|context, _, config| {
      Self::do_create_log_from_activities(context, config)?;
      Ok(())
    })
  }

  pub(super) fn do_create_log_from_activities(
    context: &mut PipelineContext,
    config: &UserDataImpl,
  ) -> Result<(), PipelinePartExecutionError> {
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
    let instances = Self::get_user_data(context, &TRACE_ACTIVITIES_KEY)?;
    let undef_activity_strat = Self::get_user_data(config, &UNDEF_ACTIVITY_HANDLING_STRATEGY_KEY)?;

    let strategy = match undef_activity_strat {
      UndefActivityHandlingStrategyDto::DontInsert => UndefActivityHandlingStrategy::DontInsert,
      UndefActivityHandlingStrategyDto::InsertAsSingleEvent => UndefActivityHandlingStrategy::InsertAsSingleEvent(Box::new(|| {
        Rc::new(RefCell::new(XesEventImpl::new_with_min_date(UNDEF_ACTIVITY_NAME.to_owned())))
      })),
      UndefActivityHandlingStrategyDto::InsertAllEvents => UndefActivityHandlingStrategy::InsertAllEvents,
    };

    let log = create_new_log_from_activities_instances(log, instances, &strategy, &|info, events| {
      let stamp = if events.len() == 1 {
        events.first().unwrap().borrow().timestamp().clone()
      } else {
        let first_stamp = events.first().unwrap().borrow().timestamp().clone();
        let delta: TimeDelta = events.iter().skip(1).map(|e| e.borrow().timestamp().clone() - first_stamp).sum();

        first_stamp + delta / (events.len() as i32 - 1)
      };

      Rc::new(RefCell::new(XesEventImpl::new(
        info.node.borrow().name().as_ref().as_ref().clone(),
        stamp,
      )))
    });

    context.put_concrete(EVENT_LOG_KEY.key(), log);
    Ok(())
  }

  pub(super) fn discover_activities_instances_for_several_levels() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::DISCOVER_ACTIVITIES_FOR_SEVERAL_LEVEL, &|context, infra, config| {
      let event_classes = Self::get_user_data(config, &EVENT_CLASSES_REGEXES_KEY)?;
      let initial_activity_level = *Self::get_user_data(config, &ACTIVITY_LEVEL_KEY)?;
      let patterns_kind = Self::get_user_data(config, &PATTERNS_KIND_KEY)?;
      let adjusting_mode = Self::get_user_data(config, &ADJUSTING_MODE_KEY)?;
      let patterns_discovery_strategy = Self::get_user_data(config, &PATTERNS_DISCOVERY_STRATEGY_KEY)?;
      let narrow_activities = Self::get_user_data(config, &NARROW_ACTIVITIES_KEY)?;
      let events_count = Self::get_user_data(config, &EVENTS_COUNT_KEY)?;
      let min_events_in_activity = Self::get_user_data(config, &MIN_ACTIVITY_LENGTH_KEY)?;
      let activity_filter_kind = Self::get_user_data(config, &ACTIVITY_IN_TRACE_FILTER_KIND_KEY)?;

      let mut index = 0;
      for event_class_regex in event_classes.into_iter().rev() {
        let mut config = UserDataImpl::new();
        config.put_concrete(PATTERNS_KIND_KEY.key(), *patterns_kind);
        config.put_concrete(EVENT_CLASS_REGEX_KEY.key(), event_class_regex.to_owned());
        config.put_concrete(ADJUSTING_MODE_KEY.key(), *adjusting_mode);
        config.put_concrete(ACTIVITY_LEVEL_KEY.key(), initial_activity_level + index);
        config.put_concrete(PATTERNS_DISCOVERY_STRATEGY_KEY.key(), *patterns_discovery_strategy);
        config.put_concrete(NARROW_ACTIVITIES_KEY.key(), *narrow_activities);
        config.put_concrete(EVENTS_COUNT_KEY.key(), *events_count);
        config.put_concrete(MIN_ACTIVITY_LENGTH_KEY.key(), *min_events_in_activity);
        config.put_concrete(ACTIVITY_IN_TRACE_FILTER_KIND_KEY.key(), *activity_filter_kind);

        Self::adjust_with_activities_from_unattached_events(context, infra, &config)?;

        index += 1;
      }

      Ok(())
    })
  }

  pub(super) fn adjust_with_activities_from_unattached_events(
    old_context: &mut PipelineContext,
    infra: &PipelineInfrastructure,
    config: &UserDataImpl,
  ) -> Result<(), PipelinePartExecutionError> {
    if Self::get_user_data(old_context, &ACTIVITIES_KEY).is_err() {
      old_context.put_concrete(ACTIVITIES_KEY.key(), vec![])
    }

    let adjusting_mode = *Self::get_user_data(config, &ADJUSTING_MODE_KEY)?;
    let log = Self::get_user_data(old_context, &EVENT_LOG_KEY)?;

    let mut new_context = PipelineContext::empty_from(&old_context);

    if adjusting_mode == AdjustingMode::FromUnattachedSubTraces {
      match Self::get_user_data(old_context, &TRACE_ACTIVITIES_KEY) {
        Ok(activities) => new_context.put_concrete(EVENT_LOG_KEY.key(), create_log_from_unattached_events(log, activities)),
        Err(_) => {}
      }
    } else {
      new_context.put_concrete(EVENT_LOG_KEY.key(), log.clone());
    }

    Self::find_patterns(&mut new_context, config)?;

    let old_activities = Self::get_user_data_mut(old_context, &ACTIVITIES_KEY)?;
    let new_activities = Self::get_user_data(&new_context, &ACTIVITIES_KEY)?;
    for new_activity in new_activities {
      old_activities.push(new_activity.clone());
    }

    old_context
      .pipeline_parts()
      .unwrap()
      .create_add_unattached_events_part(config.clone())
      .execute(old_context, infra)?;

    Ok(())
  }

  pub(super) fn discover_activities_in_unattached_subtraces() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::DISCOVER_ACTIVITIES_IN_UNATTACHED_SUBTRACES, &|context, infra, config| {
      let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
      let mut existing_activities = &Self::create_empty_activities(log);

      if let Ok(activities) = Self::get_user_data(context, &TRACE_ACTIVITIES_KEY) {
        existing_activities = activities;
      }

      let activities = Self::get_user_data_mut(context, &ACTIVITIES_KEY)?;

      let narrow_kind = Self::get_user_data(config, &NARROW_ACTIVITIES_KEY)?;
      let hashed_log = Self::create_hashed_event_log(config, log);
      let min_events_count = *Self::get_user_data(config, &EVENTS_COUNT_KEY)? as usize;
      let min_events_in_activity = *Self::get_user_data(config, &MIN_ACTIVITY_LENGTH_KEY)? as usize;
      let activity_filter_kind = Self::get_user_data(config, &ACTIVITY_IN_TRACE_FILTER_KIND_KEY)?;

      let new_activities = add_unattached_activities(
        &hashed_log,
        activities,
        existing_activities,
        min_events_count,
        narrow_kind,
        min_events_in_activity,
        activity_filter_kind,
      );

      context.put_concrete(TRACE_ACTIVITIES_KEY.key(), new_activities);

      Ok(())
    })
  }

  pub(super) fn create_add_unattached_events_part(&self, config: UserDataImpl) -> DefaultPipelinePart {
    let name = Self::DISCOVER_ACTIVITIES_IN_UNATTACHED_SUBTRACES;
    let add_unattached_events_factory = self.find_part(name).unwrap();

    add_unattached_events_factory(Box::new(config))
  }

  pub(super) fn create_empty_activities(log: &XesEventLogImpl) -> TracesActivities {
    let mut activities = vec![];
    for _ in log.traces() {
      activities.push(vec![]);
    }

    return activities;
  }

  pub(super) fn clear_activities_related_stuff() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::CLEAR_ACTIVITIES, &|context, _, _| {
      Self::do_clear_activities_related_stuff(context);
      Ok(())
    })
  }

  pub(super) fn do_clear_activities_related_stuff(context: &mut PipelineContext) {
    context.remove_concrete(ACTIVITIES_KEY.key());
    context.remove_concrete(TRACE_ACTIVITIES_KEY.key());
    context.remove_concrete(PATTERNS_KEY.key());
    context.remove_concrete(REPEAT_SETS_KEY.key());
  }

  pub(super) fn get_number_of_underlying_events() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::GET_UNDERLYING_EVENTS_COUNT, &|context, infra, _| {
      let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
      let count = count_underlying_events(log);
      infra.log(format!("Number of underlying events: {}", &count).as_str())?;

      context.put_concrete(UNDERLYING_EVENTS_COUNT_KEY.key(), count);
      Ok(())
    })
  }

  pub(super) fn execute_with_activities_instances(
    activities: &Vec<ActivityInTraceInfo>,
    trace_len: usize,
    handler: &mut impl FnMut(SubTraceKind) -> (),
  ) -> Result<(), PipelinePartExecutionError> {
    let mut index = 0usize;
    for activity in activities {
      if activity.start_pos > index {
        handler(SubTraceKind::Unattached(index, activity.start_pos - index));
      }

      handler(SubTraceKind::Attached(&activity));
      index = activity.start_pos + activity.length;
    }

    if index < trace_len {
      handler(SubTraceKind::Unattached(index, trace_len - index));
    }

    Ok(())
  }

  pub(super) fn discover_activities_until_no_more() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::DISCOVER_ACTIVITIES_UNTIL_NO_MORE, &|context, infra, config| {
      let activity_level = *Self::get_user_data(config, &ACTIVITY_LEVEL_KEY)?;
      let after_activities_extraction_pipeline = Self::get_user_data(config, &PIPELINE_KEY);
      let execute_only_after_last_extraction = *Self::get_user_data(config, &EXECUTE_ONLY_ON_LAST_EXTRACTION_KEY)?;

      loop {
        let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
        let events_count = count_events(log);

        Self::do_clear_activities_related_stuff(context);
        Self::find_patterns(context, config)?;
        Self::do_discover_activities(context, activity_level, config)?;
        Self::do_discover_activities_instances(context, config)?;

        let activities_instances = Self::get_user_data(context, &TRACE_ACTIVITIES_KEY)?;
        infra.log(
          format!(
            "Discovered {} activities instances",
            activities_instances.iter().map(|t| t.len()).sum::<usize>()
          )
            .as_str(),
        )?;

        if activities_instances.iter().map(|t| t.len()).sum::<usize>() == 0 {
          Self::do_clear_activities_related_stuff(context);
          return Ok(());
        }

        let mut executed_pipeline = false;
        if let Ok(pipeline) = after_activities_extraction_pipeline {
          let should_execute = if execute_only_after_last_extraction {
            activities_instances.iter().all(|x| x.iter().all(|y| y.length == 1))
          } else {
            true
          };

          if should_execute {
            pipeline.execute(context, infra)?;
            executed_pipeline = true;
          }
        }

        Self::do_create_log_from_activities(context, config)?;

        let new_events_count = count_events(Self::get_user_data(context, &EVENT_LOG_KEY)?);
        if (execute_only_after_last_extraction && executed_pipeline) || new_events_count == events_count {
          Self::do_clear_activities_related_stuff(context);
          return Ok(());
        }
      }
    })
  }

  pub(super) fn execute_with_each_activity_log() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::EXECUTE_WITH_EACH_ACTIVITY_LOG, &|context, infra, config| {
      let pipeline = Self::get_user_data(config, &PIPELINE_KEY)?;
      let activities_to_logs = Self::create_activities_to_logs(context, config)?;

      for (activity_name, activity_log) in activities_to_logs {
        let mut temp_context = context.clone();
        temp_context.put_concrete(&EVENT_LOG_KEY.key(), activity_log.borrow().clone());
        temp_context.put_concrete(&ACTIVITY_NAME_KEY.key(), activity_name.clone());

        pipeline.execute(&mut temp_context, infra)?;
      }

      Ok(())
    })
  }

  fn create_activities_to_logs(
    context: &mut PipelineContext,
    config: &UserDataImpl,
  ) -> Result<HashMap<String, Rc<RefCell<XesEventLogImpl>>>, PipelinePartExecutionError> {
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
    let dto = Self::get_user_data(config, &ACTIVITIES_LOGS_SOURCE_KEY)?;

    match dto {
      ActivitiesLogsSourceDto::Log => Ok(activity_instances::create_logs_for_activities(&ActivitiesLogSource::Log(log))),
      ActivitiesLogsSourceDto::TracesActivities => {
        let activity_level = *Self::get_user_data(config, &ACTIVITY_LEVEL_KEY)?;
        let activities = Self::get_user_data(context, &TRACE_ACTIVITIES_KEY)?;
        Ok(activity_instances::create_logs_for_activities(
          &ActivitiesLogSource::TracesActivities(log, activities, activity_level as usize),
        ))
      }
    }
  }

  pub(super) fn substitute_underlying_events() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::SUBSTITUTE_UNDERLYING_EVENTS, &|context, _, _| {
      let log = Self::get_user_data_mut(context, &EVENT_LOG_KEY)?;
      let mut new_log = XesEventLogImpl::empty();

      for trace in log.traces() {
        let mut new_trace = XesTraceImpl::empty();
        for event in trace.borrow().events() {
          substitute_underlying_events::<XesEventLogImpl>(event, &mut new_trace);
        }

        new_log.push(Rc::new(RefCell::new(new_trace)));
      }

      context.put_concrete(EVENT_LOG_KEY.key(), new_log);
      Ok(())
    })
  }

  pub(super) fn apply_class_extractor() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::APPLY_CLASS_EXTRACTOR, &|context, _, config| {
      let log = Self::get_user_data_mut(context, &EVENT_LOG_KEY)?;

      let event_class_regex = Self::get_user_data(config, &EVENT_CLASS_REGEX_KEY)?;
      let event_class_regex = Self::try_parse_regex(event_class_regex)?;

      let filter_regex = Self::get_user_data(config, &REGEX_KEY)?;
      let filter_regex = Self::try_parse_regex(filter_regex)?;

      for trace in log.traces() {
        for event in trace.borrow().events() {
          if !filter_regex.is_match(event.borrow().name()).ok().unwrap() {
            continue;
          }

          let borrowed_event = event.borrow();
          let found_match = event_class_regex.find(borrowed_event.name());
          if found_match.is_err() {
            continue;
          }

          let (start, end) = if let Ok(Some(found_match)) = found_match {
            (found_match.start(), found_match.end())
          } else {
            (0, borrowed_event.name().len())
          };

          drop(found_match);
          drop(borrowed_event);

          if start == 0 {
            let new_name = event.borrow().name()[start..end].to_owned();
            event.borrow_mut().set_name(new_name);
          }
        }
      }

      Ok(())
    })
  }

  pub(super) fn serialize_activities_logs() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::SERIALIZE_ACTIVITIES_LOGS, &|context, infra, config| {
      let logs_to_activities = Self::create_activities_to_logs(context, config)?;
      let path = Path::new(Self::get_user_data(config, &PATH_KEY)?);
      let format = Self::get_user_data(config, &LOG_SERIALIZATION_FORMAT_KEY)?;
      let mut log_number = 1;

      for (_, log) in &logs_to_activities {
        let save_path = path.join(format!("log_{}.{}", log_number, format.extension()));
        let save_path = save_path.as_os_str().to_str().unwrap();

        match format {
          LogSerializationFormat::Xes => match write_xes_log(&log.borrow(), save_path) {
            Ok(_) => {}
            Err(err) => return Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(err.to_string()))),
          },
          LogSerializationFormat::Bxes => match write_event_log_to_bxes(&log.borrow(), None, save_path) {
            Ok(_) => {}
            Err(err) => return Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(err.to_string()))),
          },
        };

        log_number += 1;
      }

      Ok(())
    })
  }

  pub(super) fn reverse_hierarchy_indices() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::REVERSE_HIERARCHY_INDICES, &|context, infra, config| {
      let log = Self::get_user_data_mut(context, &EVENT_LOG_KEY)?;

      const HIERARCHY_LEVEL: &str = "hierarchy_level_";
      let mut max_level = 0usize;
      for trace in log.traces() {
        let trace = trace.borrow();
        for event in trace.events() {
          let event = event.borrow();
          for (key, _) in event.ordered_payload() {
            if key.starts_with(HIERARCHY_LEVEL) {
              let level = &key[HIERARCHY_LEVEL.len()..];
              let level = level.parse::<usize>().ok().unwrap();
              max_level = max_level.max(level);
            }
          }
        }
      }

      for trace in log.traces() {
        let trace = trace.borrow();
        for event in trace.events() {
          let mut updates = vec![];
          let mut event = event.borrow_mut();
          if let Some(payload) = event.payload_map() {
            let keys = payload.keys().into_iter().filter(|k| k.starts_with(HIERARCHY_LEVEL));
            for key in keys {
              let level = &key[HIERARCHY_LEVEL.len()..].parse::<usize>().ok().unwrap();
              let new_level = max_level - level;
              let old_value = payload.get(key).unwrap().clone();
              let old_key = key.to_owned();

              updates.push((new_level, old_value, old_key));
            }
          }

          for update in &updates {
            event.payload_map_mut().unwrap().remove(&update.2);
          }

          for update in updates {
            let new_key = format!("{}{}", HIERARCHY_LEVEL, update.0);
            event.payload_map_mut().unwrap().insert(new_key, update.1);
          }
        }
      }

      Ok(())
    })
  }
}
