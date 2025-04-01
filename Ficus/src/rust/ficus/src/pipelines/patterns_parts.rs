use super::{context::PipelineContext, errors::pipeline_errors::PipelinePartExecutionError, pipelines::PipelinePartFactory};
use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::features::analysis::patterns::activity_instances::ActivityInTraceInfo;
use crate::features::analysis::patterns::repeat_sets::ActivityNode;
use crate::features::analysis::patterns::tandem_arrays::find_maximal_tandem_arrays_with_length;
use crate::pipelines::keys::context_keys::{ACTIVITY_LEVEL_KEY, EVENT_LOG_KEY, HASHES_EVENT_LOG_KEY, PATTERNS_DISCOVERY_STRATEGY_KEY, PATTERNS_KEY, PATTERNS_KIND_KEY, TANDEM_ARRAY_LENGTH_KEY, TRACE_ACTIVITIES_KEY};
use crate::pipelines::pipeline_parts::PipelineParts;
use crate::{
  features::analysis::patterns::{
    contexts::PatternsDiscoveryStrategy,
    repeats::{find_maximal_repeats, find_near_super_maximal_repeats, find_super_maximal_repeats},
    tandem_arrays::{find_maximal_tandem_arrays, find_primitive_tandem_arrays, SubArrayInTraceInfo},
  },
  utils::user_data::user_data::{UserData, UserDataImpl},
};
use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Clone, Copy)]
pub enum PatternsKindDto {
  PrimitiveTandemArrays,
  MaximalTandemArrays,

  MaximalRepeats,
  SuperMaximalRepeats,
  NearSuperMaximalRepeats,
}

impl FromStr for PatternsKindDto {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "PrimitiveTandemArrays" => Ok(Self::PrimitiveTandemArrays),
      "MaximalTandemArrays" => Ok(Self::MaximalTandemArrays),
      "MaximalRepeats" => Ok(Self::MaximalRepeats),
      "SuperMaximalRepeats" => Ok(Self::SuperMaximalRepeats),
      "NearSuperMaximalRepeats" => Ok(Self::NearSuperMaximalRepeats),
      _ => Err(()),
    }
  }
}

impl PipelineParts {
  pub(super) fn find_maximal_repeats() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::FIND_MAXIMAL_REPEATS, &|context, _, config| {
      Self::find_repeats_and_put_to_context(context, config, find_maximal_repeats)
    })
  }

  pub(super) fn find_super_maximal_repeats() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::FIND_SUPER_MAXIMAL_REPEATS, &|context, _, config| {
      Self::find_repeats_and_put_to_context(context, config, find_super_maximal_repeats)
    })
  }

  pub(super) fn find_near_super_maximal_repeats() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::FIND_NEAR_SUPER_MAXIMAL_REPEATS, &|context, _, config| {
      Self::find_repeats_and_put_to_context(context, config, find_near_super_maximal_repeats)
    })
  }

  pub(super) fn find_primitive_tandem_arrays() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::FIND_PRIMITIVE_TANDEM_ARRAYS, &|context, _, config| {
      Self::find_tandem_arrays_and_put_to_context(context, &config, find_primitive_tandem_arrays)
    })
  }

  pub(super) fn find_maximal_tandem_arrays() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::FIND_MAXIMAL_TANDEM_ARRAYS, &|context, _, config| {
      Self::find_tandem_arrays_and_put_to_context(context, &config, find_maximal_tandem_arrays)
    })
  }

  pub(super) fn find_tandem_arrays_and_put_to_context(
    context: &mut PipelineContext,
    config: &UserDataImpl,
    patterns_finder: impl Fn(&Vec<Vec<u64>>, usize, bool) -> Vec<Vec<SubArrayInTraceInfo>>,
  ) -> Result<(), PipelinePartExecutionError> {
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
    let array_length = *config.concrete(TANDEM_ARRAY_LENGTH_KEY.key()).unwrap() as usize;

    let hashed_log = Self::create_hashed_event_log(config, log);

    let arrays = patterns_finder(&hashed_log, array_length, false);

    context.put_concrete(HASHES_EVENT_LOG_KEY.key(), hashed_log);
    context.put_concrete(PATTERNS_KEY.key(), arrays);

    Ok(())
  }

  pub(super) fn find_repeats_and_put_to_context(
    context: &mut PipelineContext,
    config: &UserDataImpl,
    patterns_finder: impl Fn(&Vec<Vec<u64>>, &PatternsDiscoveryStrategy) -> Vec<Vec<SubArrayInTraceInfo>>,
  ) -> Result<(), PipelinePartExecutionError> {
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
    let strategy = Self::get_user_data(config, &PATTERNS_DISCOVERY_STRATEGY_KEY)?;

    let hashed_log = Self::create_hashed_event_log(config, log);

    let repeats = patterns_finder(&hashed_log, &strategy);

    context.put_concrete(HASHES_EVENT_LOG_KEY.key(), hashed_log);
    context.put_concrete(PATTERNS_KEY.key(), repeats);

    Ok(())
  }

  pub(super) fn find_patterns(context: &mut PipelineContext, config: &UserDataImpl) -> Result<(), PipelinePartExecutionError> {
    let patterns_kind = Self::get_user_data(config, &PATTERNS_KIND_KEY)?;
    match patterns_kind {
      PatternsKindDto::PrimitiveTandemArrays => {
        Self::find_tandem_arrays_and_put_to_context(context, config, find_primitive_tandem_arrays)?
      }
      PatternsKindDto::MaximalTandemArrays => {
        Self::find_tandem_arrays_and_put_to_context(context, config, find_maximal_tandem_arrays)?
      }
      PatternsKindDto::MaximalRepeats => Self::find_repeats_and_put_to_context(context, config, find_maximal_repeats)?,
      PatternsKindDto::SuperMaximalRepeats => Self::find_repeats_and_put_to_context(context, config, find_super_maximal_repeats)?,
      PatternsKindDto::NearSuperMaximalRepeats => {
        Self::find_repeats_and_put_to_context(context, config, find_near_super_maximal_repeats)?
      }
    };

    let activity_level = Self::get_user_data(config, &ACTIVITY_LEVEL_KEY)?;
    Self::do_discover_activities(context, *activity_level, config)?;

    Ok(())
  }

  pub(super) fn discover_loops_strict() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::DISCOVER_LOOPS_STRICT, &|context, _, config| {
      let max_array_length = Self::get_user_data(config, &TANDEM_ARRAY_LENGTH_KEY)?;
      let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
      let hashed_log = Self::create_hashed_event_log(config, log);

      let mut instances = find_maximal_tandem_arrays_with_length(&hashed_log, *max_array_length as usize, true)
        .into_iter()
        .enumerate()
        .map(|(trace_index, trace_arrays)|
          trace_arrays
            .into_iter()
            .map(|array| {
              let repeat_count = *array.get_repeat_count();
              let array = array.get_sub_array_info();

              let mut name = log.traces().get(trace_index).unwrap().borrow().events()[array.start_index..array.start_index + array.length]
                .iter()
                .map(|e| e.borrow().name().clone())
                .collect::<HashSet<String>>()
                .into_iter()
                .collect::<Vec<String>>();

              name.sort();

              ActivityInTraceInfo {
                start_pos: array.start_index,
                length: array.length * repeat_count,
                node: Rc::new(RefCell::new(ActivityNode::new(
                  None,
                  HashSet::from_iter(hashed_log.get(trace_index.clone()).unwrap()[array.start_index..array.start_index + array.length].iter().map(|x| *x)),
                  vec![],
                  0,
                  Rc::new(Box::new(format!("Loop[{}]", name.join("::")))),
                ))),
              }
            })
            .into_group_map_by(|activity| activity.start_pos)
            .into_iter()
            .map(|(_, activities_by_start_pos)| {
              activities_by_start_pos.into_iter().max_by(|f, s| f.length.cmp(&s.length)).unwrap()
            })
            .collect()
        )
        .collect::<Vec<Vec<ActivityInTraceInfo>>>();

      instances.iter_mut().for_each(|trace| trace.sort_by(|first, second| first.start_pos.cmp(&second.start_pos)));

      let mut filtered_instances = vec![];
      for trace_instances in instances {
        let mut filtered_trace_instances = vec![];
        let mut covered_range = None;

        for activity in trace_instances {
          match covered_range {
            Some(to_index) => if activity.start_pos >= to_index {
              covered_range = Some(activity.start_pos + activity.length);
              filtered_trace_instances.push(activity);
            },
            None => {
              covered_range = Some(activity.start_pos + activity.length);
              filtered_trace_instances.push(activity);
            }
          }
        }

        filtered_instances.push(filtered_trace_instances);
      }

      context.put_concrete(TRACE_ACTIVITIES_KEY.key(), filtered_instances);

      Ok(())
    })
  }
}
