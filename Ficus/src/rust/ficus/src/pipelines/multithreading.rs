use crate::event_log::core::event::event::{Event, EventPayloadValue};
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::event_log::xes::xes_trace::XesTraceImpl;
use crate::features::analysis::log_info::event_log_info::create_threads_log_by_attribute;
use crate::features::clustering::traces::dbscan::clusterize_log_by_traces_dbscan;
use crate::features::discovery::multithreaded_dfg::dfg::{discover_multithreaded_dfg, enumerate_multithreaded_events_groups, MultithreadedTracePartsCreationStrategy};
use crate::features::discovery::root_sequence::log_prepare::prepare_software_log;
use crate::features::discovery::timeline::abstraction::abstract_event_groups;
use crate::features::discovery::timeline::discovery::{discover_timeline_diagram, discover_traces_timeline_diagram};
use crate::features::discovery::timeline::events_groups::{enumerate_event_groups, EventGroup};
use crate::features::discovery::timeline::software_data::extraction_config::{ExtractionConfig, MethodStartEndConfig, SoftwareDataExtractionConfig};
use crate::pipelines::context::{PipelineContext, PipelineInfrastructure};
use crate::pipelines::errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError};
use crate::pipelines::keys::context_keys::{DISCOVER_EVENTS_GROUPS_IN_EACH_TRACE_KEY, EVENT_LOG_KEY, GRAPH_KEY, LABELED_LOG_TRACES_DATASET_KEY, LOG_THREADS_DIAGRAM_KEY, MIN_EVENTS_IN_CLUSTERS_COUNT_KEY, PIPELINE_KEY, PUT_NOISE_EVENTS_IN_ONE_CLUSTER_KEY, REGEXES_KEY, SOFTWARE_DATA_EXTRACTION_CONFIG_KEY, THREAD_ATTRIBUTE_KEY, TIME_ATTRIBUTE_KEY, TIME_DELTA_KEY, TOLERANCE_KEY};
use crate::pipelines::pipeline_parts::PipelineParts;
use crate::pipelines::pipelines::{PipelinePart, PipelinePartFactory};
use crate::utils::display_name::DISPLAY_NAME_KEY;
use crate::utils::user_data::user_data::{UserData, UserDataImpl, UserDataOwner};
use fancy_regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Copy, Clone)]
pub enum FeatureCountKindDto {
  One,
  Count,
  OneIfMoreThanMaxFromAllFeatures,
}

impl FromStr for FeatureCountKindDto {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "One" => Ok(Self::One),
      "Count" => Ok(Self::Count),
      "OneIfMoreThanMaxFromAllFeatures" => Ok(Self::OneIfMoreThanMaxFromAllFeatures),
      _ => Err(()),
    }
  }
}

impl PipelineParts {
  pub(super) fn discover_log_threads_diagram() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::DISCOVER_LOG_TIMELINE_DIAGRAM, &|context, _, config| {
      let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
      let thread_attribute = Self::get_user_data(config, &THREAD_ATTRIBUTE_KEY)?;
      let time_attribute = Self::get_user_data(config, &TIME_ATTRIBUTE_KEY).ok();
      let event_group_delta = Self::get_user_data(config, &TIME_DELTA_KEY).ok();
      let software_data_extraction_config = Self::get_software_data_extraction_config(context);

      let diagram = discover_timeline_diagram(
        log,
        thread_attribute.as_str(),
        time_attribute,
        match event_group_delta {
          None => None,
          Some(delta) => Some(*delta as u64)
        },
        Self::get_control_flow_regexes(&software_data_extraction_config)?.as_ref(),
      );

      match diagram {
        Err(err) => return Err(err.into()),
        Ok(diagram) => context.put_concrete(LOG_THREADS_DIAGRAM_KEY.key(), diagram),
      }

      Ok(())
    })
  }

  fn get_control_flow_regexes(config: &SoftwareDataExtractionConfig) -> Result<Option<Vec<Regex>>, PipelinePartExecutionError> {
    config.control_flow_regexes().map_err(|message| PipelinePartExecutionError::Raw(RawPartExecutionError::new(message)))
  }

  pub(super) fn create_threads_log() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::CREATE_THREADS_LOG, &|context, _, config| {
      let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
      let thread_attribute = Self::get_user_data(config, &THREAD_ATTRIBUTE_KEY)?;
      context.put_concrete(EVENT_LOG_KEY.key(), create_threads_log_by_attribute(log, thread_attribute));

      Ok(())
    })
  }

  pub(super) fn abstract_timeline_diagram() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::ABSTRACT_TIMELINE_DIAGRAM, &|context, infra, config| {
      let (thread_attribute, time_attribute) = Self::extract_thread_and_time_attribute(context)?;

      Self::abstract_event_groups(
        Self::create_event_groups_from_timeline(context)?,
        context,
        config,
        infra,
        thread_attribute,
        time_attribute,
      )
    })
  }

  fn create_event_groups_from_timeline(context: &PipelineContext) -> Result<Vec<Vec<EventGroup>>, PipelinePartExecutionError> {
    let timeline = Self::get_user_data(context, &LOG_THREADS_DIAGRAM_KEY)?;
    Ok(enumerate_event_groups(timeline))
  }

  fn extract_thread_and_time_attribute(context: &PipelineContext) -> Result<(String, Option<String>), PipelinePartExecutionError> {
    let timeline = Self::get_user_data(context, &LOG_THREADS_DIAGRAM_KEY)?;
    Ok((timeline.thread_attribute().clone(), timeline.time_attribute().as_ref().cloned()))
  }

  fn abstract_event_groups(
    events_groups: Vec<Vec<EventGroup>>,
    context: &mut PipelineContext,
    config: &UserDataImpl,
    infra: &PipelineInfrastructure,
    thread_attribute: String,
    time_attribute: Option<String>,
  ) -> Result<(), PipelinePartExecutionError> {
    let extraction_config = Self::get_software_data_extraction_config(context);
    let events_groups_log = Self::create_groups_event_log(&events_groups);
    let mut params = Self::create_traces_clustering_params(context, config)?;
    params.vis_params.log = &events_groups_log;

    let min_points_in_cluster = *Self::get_user_data(config, &MIN_EVENTS_IN_CLUSTERS_COUNT_KEY)? as usize;
    let tolerance = *Self::get_user_data(config, &TOLERANCE_KEY)?;
    let put_noise_events_in_one_cluster = *Self::get_user_data(config, &PUT_NOISE_EVENTS_IN_ONE_CLUSTER_KEY)?;

    let (_, labeled_dataset) = match clusterize_log_by_traces_dbscan(&mut params, tolerance, min_points_in_cluster, put_noise_events_in_one_cluster) {
      Ok(new_logs) => new_logs,
      Err(error) => return Err(error.into()),
    };

    if let Some(after_clusterization_pipeline) = Self::get_user_data(config, &PIPELINE_KEY).ok() {
      let abstracted_log = abstract_event_groups(
        events_groups,
        labeled_dataset.labels(),
        thread_attribute,
        time_attribute,
        &extraction_config,
      )?;

      let mut new_context = context.clone();
      new_context.put_concrete(EVENT_LOG_KEY.key(), abstracted_log);

      after_clusterization_pipeline.execute(&mut new_context, infra)?;
    }

    context.put_concrete(LABELED_LOG_TRACES_DATASET_KEY.key(), labeled_dataset);

    Ok(())
  }

  pub fn abstract_multithreaded_events_groups() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::ABSTRACT_MULTITHREADED_EVENTS_GROUPS, &|context, infra, config| {
      let thread_attribute = Self::get_user_data(config, &THREAD_ATTRIBUTE_KEY)?.to_owned();
      let time_attribute = Self::get_user_data(config, &TIME_ATTRIBUTE_KEY).ok().cloned();
      let software_config = Self::get_software_data_extraction_config(context);
      let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;

      let strategy = Self::create_multithreaded_trace_parts_creation_strategy(config)?;

      let groups = enumerate_multithreaded_events_groups(log, &software_config, thread_attribute.as_str(), &strategy)
        .map_err(|e| PipelinePartExecutionError::new_raw(e))?;

      Self::abstract_event_groups(
        groups,
        context,
        config,
        infra,
        thread_attribute,
        time_attribute,
      )
    })
  }

  fn get_software_data_extraction_config(context: &PipelineContext) -> SoftwareDataExtractionConfig {
    match Self::get_user_data(context, &SOFTWARE_DATA_EXTRACTION_CONFIG_KEY) {
      Ok(config) => config.clone(),
      Err(_) => SoftwareDataExtractionConfig::empty(),
    }
  }

  fn create_groups_event_log(events_groups: &Vec<Vec<EventGroup>>) -> XesEventLogImpl {
    let mut log = XesEventLogImpl::empty();

    for trace in events_groups {
      for group in trace {
        let mut new_trace = XesTraceImpl::empty();

        for event in group.control_flow_events() {
          new_trace.push(event.clone());
        }

        log.push(Rc::new(RefCell::new(new_trace)));
      }
    }

    log
  }

  pub(super) fn discover_traces_timeline_diagram() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::DISCOVER_TRACES_TIMELINE_DIAGRAM, &|context, _, config| {
      let time_attribute = Self::get_user_data(config, &TIME_ATTRIBUTE_KEY).ok();
      let event_group_delta = Self::get_user_data(config, &TIME_DELTA_KEY).ok();
      let discover_events_groups_in_each_trace = Self::get_user_data(config, &DISCOVER_EVENTS_GROUPS_IN_EACH_TRACE_KEY)?;
      let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
      let software_data_extraction_config = Self::get_software_data_extraction_config(context);

      let diagram = discover_traces_timeline_diagram(
        log,
        time_attribute,
        match event_group_delta {
          None => None,
          Some(delta) => Some(*delta as u64)
        },
        *discover_events_groups_in_each_trace,
        Self::get_control_flow_regexes(&software_data_extraction_config)?.as_ref(),
      );

      match diagram {
        Err(err) => return Err(err.into()),
        Ok(diagram) => context.put_concrete(LOG_THREADS_DIAGRAM_KEY.key(), diagram),
      }

      Ok(())
    })
  }

  pub(super) fn prepare_software_log() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::PREPARE_SOFTWARE_EVENT_LOG, &|context, _, config| {
      let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
      let software_data_extraction_config = Self::get_software_data_extraction_config(context);
      let time_attribute = Self::get_user_data(config, &TIME_ATTRIBUTE_KEY).ok();

      let prepared_log = match prepare_software_log(log, &software_data_extraction_config, time_attribute) {
        Ok(log) => log,
        Err(err) => return Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(err)))
      };

      context.put_concrete(EVENT_LOG_KEY.key(), prepared_log);

      Ok(())
    })
  }

  pub(super) fn shorten_allocation_types() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::SHORTEN_ALLOCATION_TYPE, &|context, _, config| {
      let log = Self::get_user_data_mut(context, &EVENT_LOG_KEY)?;
      let software_data_extraction_config = Self::get_software_data_extraction_config(context);
      if let Some(config) = software_data_extraction_config.allocation() {
        let alloc_regex = config.event_class_regex();
        let alloc_regex = match Regex::new(alloc_regex) {
          Ok(regex) => regex,
          Err(err) => return Err(PipelinePartExecutionError::Raw(
            RawPartExecutionError::new(format!("Failed to create regex from {}, error: {}", alloc_regex, err.to_string()))
          ))
        };

        for trace in log.traces() {
          let trace = trace.borrow_mut();
          for event in trace.events() {
            if alloc_regex.is_match(event.borrow().name().as_str()).unwrap_or(false) {
              let mut event = event.borrow_mut();
              if let Some(map) = event.payload_map_mut() {
                if let Some(type_name) = map.get_mut(config.info().type_name_attr().as_str()) {
                  let string = type_name.to_string_repr().to_string();
                  *type_name = EventPayloadValue::String(Rc::new(Box::new(Self::shorten_type_or_method_name(string))));
                }
              }
            }
          }
        }
      }

      Ok(())
    })
  }

  fn shorten_type_or_method_name(name: String) -> String {
    let mut result = String::new();
    let mut chars = name.chars();
    let mut last_seen_word = String::new();

    'main_loop: while let Some(char) = chars.next() {
      if char == '`' {
        result.push_str(last_seen_word.as_str());
        last_seen_word.clear();

        while let Some(char) = chars.next() {
          if char == '[' {
            result.push('[');
            continue 'main_loop;
          }
        }
      }

      if char == '.' {
        if !last_seen_word.is_empty() {
          result.push(last_seen_word.chars().next().unwrap());
        }

        result.push(char);
        last_seen_word.clear();
        continue;
      }

      if char == ',' || char == ']' || char == '[' {
        result.push_str(last_seen_word.as_str());
        last_seen_word.clear();
        result.push(char);
        continue;
      }

      last_seen_word.push(char);
    }

    if !last_seen_word.is_empty() {
      result.push_str(last_seen_word.as_str());
    }

    result
  }

  pub(super) fn shorten_methods_names() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::SHORTEN_METHOD_NAMES, &|context, _, _| {
      let log = Self::get_user_data_mut(context, &EVENT_LOG_KEY)?;

      let methods_id_factories: Vec<&dyn Fn(&String, &String, &String) -> String> = vec![
        &|_, name, _| name.to_owned(),
        &|_, name, signature| name.to_owned() + signature,
        &|namespace, name, _| namespace.to_string() + name,
      ];

      let configs = Self::create_processed_method_extraction_configs(context);
      if configs.is_empty() {
        return Ok(());
      }

      for config in &configs {
        for method_id_factory in &methods_id_factories {
          if Self::check_if_can_use_method_id(log, config, method_id_factory) {
            for trace in log.traces() {
              let trace = trace.borrow_mut();
              for event in trace.events() {
                let mut event = event.borrow_mut();
                let name = if let Some(payload) = event.payload_map() {
                  if let Some((namespace, name, signature)) = Self::extract_method_name_parts(payload, config) {
                    method_id_factory(&namespace, &name, &signature)
                  } else {
                    continue;
                  }
                } else {
                  continue;
                };

                event.set_name(name);
              }
            }

            return Ok(());
          }
        }

        for trace in log.traces() {
          let trace = trace.borrow_mut();
          for event in trace.events() {
            if config.event_regex.is_match(event.borrow().name().as_str()).unwrap_or(false) {
              Self::shorten_method_name(config, &mut event.borrow_mut());
            }
          }
        }
      }

      Ok(())
    })
  }

  fn create_processed_method_extraction_configs(context: &PipelineContext) -> Vec<ProcessedMethodStartEndConfig> {
    let software_data_extraction_config = Self::get_software_data_extraction_config(context);

    let mut configs = vec![];

    Self::try_add_processed_config(software_data_extraction_config.method_start().as_ref(), &mut configs);
    Self::try_add_processed_config(software_data_extraction_config.method_end().as_ref(), &mut configs);

    configs
  }

  fn try_add_processed_config(config: Option<&ExtractionConfig<MethodStartEndConfig>>, processed_configs: &mut Vec<ProcessedMethodStartEndConfig>) {
    if let Ok(Some(processed_config)) = Self::create_method_extraction_info(config) {
      processed_configs.push(processed_config)
    }
  }

  fn create_method_extraction_info(config: Option<&ExtractionConfig<MethodStartEndConfig>>) -> Result<Option<ProcessedMethodStartEndConfig>, PipelinePartExecutionError> {
    if let Some(config) = config {
      let regex = Regex::new(config.event_class_regex()).map_err(|e| PipelinePartExecutionError::Raw(RawPartExecutionError::new(e.to_string())))?;

      Ok(Some(ProcessedMethodStartEndConfig {
        event_regex: regex,
        name_attr: config.info().method_attrs().name_attr().to_owned(),
        signature_attr: config.info().method_attrs().signature_attr().to_owned(),
        namespace_attr: config.info().method_attrs().namespace_attr().to_owned(),
        prefix: config.info().prefix().as_ref().cloned(),
      }))
    } else {
      Ok(None)
    }
  }

  fn shorten_method_name(config: &ProcessedMethodStartEndConfig, event: &mut XesEventImpl) {
    let shortened_name = if let Some(payload) = event.payload_map() {
      if let Some((namespace, name, signature)) = Self::extract_method_name_parts(payload, config) {
        let shortened_name = Self::shorten_type_or_method_name(namespace + "." + name.as_str()) + signature.as_str();
        if let Some(prefix) = config.prefix.as_ref().cloned() {
          prefix + shortened_name.as_str()
        } else {
          shortened_name
        }
      } else {
        return;
      }
    } else {
      return;
    };

    event.set_name(shortened_name);
  }

  fn extract_method_name_parts(payload: &HashMap<String, EventPayloadValue>, config: &ProcessedMethodStartEndConfig) -> Option<(String, String, String)> {
    let namespace = payload.get(config.namespace_attr.as_str()).map(|v| v.to_string_repr().as_str().to_owned())?;
    let name = payload.get(config.name_attr.as_str()).map(|v| v.to_string_repr().as_str().to_owned())?;
    let signature = payload.get(config.signature_attr.as_str()).map(|v| v.to_string_repr().as_str().to_owned())?;

    Some((namespace, name, signature))
  }

  fn check_if_can_use_method_id(
    log: &XesEventLogImpl,
    config: &ProcessedMethodStartEndConfig,
    method_id_factory: impl Fn(&String, &String, &String) -> String,
  ) -> bool {
    let mut map = HashMap::new();

    for trace in log.traces().iter().map(|t| t.borrow()) {
      for event in trace.events().iter().map(|e| e.borrow()) {
        if config.event_regex.is_match(event.name().as_str()).unwrap_or(false) {
          continue;
        }

        if let Some(payload) = event.payload_map() {
          if let Some((namespace, name, signature)) = Self::extract_method_name_parts(payload, config) {
            let id = method_id_factory(&namespace, &name, &signature);
            let fqn = namespace + name.as_str() + signature.as_str();

            if let Some(entry) = map.get(&id) {
              if !fqn.eq(entry) {
                return false;
              }
            } else {
              map.insert(id, fqn);
            }
          }
        }
      }
    }

    true
  }

  pub(super) fn set_methods_display_name() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::SET_METHODS_DISPLAY_NAME, &|context, _, _| {
      let log = Self::get_user_data_mut(context, &EVENT_LOG_KEY)?;
      let configs = Self::create_processed_method_extraction_configs(context);

      for trace in log.traces() {
        let trace = trace.borrow();
        for event in trace.events() {
          let mut display_name = None;
          if let Some(payload) = event.borrow().payload_map() {
            for config in &configs {
              if config.event_regex.is_match(event.borrow().name().as_str()).unwrap_or(false) {
                if let Some((_, name, _)) = Self::extract_method_name_parts(payload, config) {
                  display_name = Some(match config.prefix.as_ref() {
                    None => name,
                    Some(prefix) => prefix.to_string() + name.as_str()
                  });
                }
              }
            }
          }

          if let Some(display_name) = display_name {
            event.borrow_mut().user_data_mut().put_concrete(DISPLAY_NAME_KEY.key(), display_name);
          }
        }
      }

      Ok(())
    })
  }

  pub(super) fn remain_only_method_start_events() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::REMAIN_ONLY_METHOD_START_EVENTS, &|context, _, _| {
      Self::remain_only_method_start_or_end_events(context, true)
    })
  }

  fn remain_only_method_start_or_end_events(
    context: &PipelineContext,
    remain_start_events: bool,
  ) -> Result<(), PipelinePartExecutionError> {
    let log = Self::get_user_data_mut(context, &EVENT_LOG_KEY)?;
    let config = Self::get_software_data_extraction_config(context);
    let config = match remain_start_events {
      true => config.method_end(),
      false => config.method_start()
    };

    if let Some(config) = Self::create_method_extraction_info(config.as_ref())? {
      log.filter_events_by(|e| config.event_regex.is_match(e.name().as_str()).unwrap_or(false));
    }

    Ok(())
  }

  pub(super) fn remain_only_method_end_events() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::REMAIN_ONLY_METHOD_END_EVENTS, &|context, _, _| {
      Self::remain_only_method_start_or_end_events(context, false)
    })
  }

  pub(super) fn discover_multithreaded_dfg() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::DISCOVER_MULTITHREADED_DFG, &|context, _, config| {
      let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
      let thread_attribute = Self::get_user_data(config, &THREAD_ATTRIBUTE_KEY)?;
      let strategy = Self::create_multithreaded_trace_parts_creation_strategy(config)?;

      let dfg = discover_multithreaded_dfg(log, thread_attribute.as_str(), &strategy);
      context.put_concrete(GRAPH_KEY.key(), dfg);

      Ok(())
    })
  }
  
  fn create_multithreaded_trace_parts_creation_strategy(config: &UserDataImpl) -> Result<MultithreadedTracePartsCreationStrategy, PipelinePartExecutionError> {
    match Self::get_user_data(config, &REGEXES_KEY) {
      Ok(regexes) => {
        let mut result = vec![];
        for r in regexes.iter().map(|r| Regex::new(r.as_str()).map_err(|e| PipelinePartExecutionError::new_raw(e.to_string()))) {
          result.push(r?);
        }

        Ok(MultithreadedTracePartsCreationStrategy::Regexes(result))
      },
      Err(_) => Ok(MultithreadedTracePartsCreationStrategy::Default)
    }
  }
}

struct ProcessedMethodStartEndConfig {
  event_regex: Regex,
  namespace_attr: String,
  name_attr: String,
  signature_attr: String,
  prefix: Option<String>,
}