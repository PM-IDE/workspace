use super::{context::PipelineContext, errors::pipeline_errors::PipelinePartExecutionError, pipelines::PipelinePartFactory};
use crate::features::analysis::patterns::pattern_info::{UnderlyingPatternKind, UNDERLYING_PATTERN_KIND};
use crate::features::analysis::patterns::strict_loops::find_loops_strict;
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
use std::str::FromStr;

#[derive(Clone, Copy)]
pub enum PatternsKindDto {
  PrimitiveTandemArrays,
  MaximalTandemArrays,

  MaximalRepeats,
  SuperMaximalRepeats,
  NearSuperMaximalRepeats,
}

impl Into<UnderlyingPatternKind> for PatternsKindDto {
  fn into(self) -> UnderlyingPatternKind {
    match self {
      PatternsKindDto::PrimitiveTandemArrays => UnderlyingPatternKind::PrimitiveTandemArray,
      PatternsKindDto::MaximalTandemArrays => UnderlyingPatternKind::MaximalTandemArray,
      PatternsKindDto::MaximalRepeats => UnderlyingPatternKind::MaximalRepeat,
      PatternsKindDto::SuperMaximalRepeats => UnderlyingPatternKind::SuperMaximalRepeat,
      PatternsKindDto::NearSuperMaximalRepeats => UnderlyingPatternKind::NearSuperMaximalRepeat,
    }
  }
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
      Self::find_repeats_and_put_to_context(context, config, find_maximal_repeats, UnderlyingPatternKind::MaximalRepeat)
    })
  }

  pub(super) fn find_super_maximal_repeats() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::FIND_SUPER_MAXIMAL_REPEATS, &|context, _, config| {
      Self::find_repeats_and_put_to_context(context, config, find_super_maximal_repeats, UnderlyingPatternKind::SuperMaximalRepeat)
    })
  }

  pub(super) fn find_near_super_maximal_repeats() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::FIND_NEAR_SUPER_MAXIMAL_REPEATS, &|context, _, config| {
      Self::find_repeats_and_put_to_context(context, config, find_near_super_maximal_repeats, UnderlyingPatternKind::NearSuperMaximalRepeat)
    })
  }

  pub(super) fn find_primitive_tandem_arrays() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::FIND_PRIMITIVE_TANDEM_ARRAYS, &|context, _, config| {
      Self::find_tandem_arrays_and_put_to_context(context, &config, find_primitive_tandem_arrays, UnderlyingPatternKind::PrimitiveTandemArray)
    })
  }

  pub(super) fn find_maximal_tandem_arrays() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::FIND_MAXIMAL_TANDEM_ARRAYS, &|context, _, config| {
      Self::find_tandem_arrays_and_put_to_context(context, &config, find_maximal_tandem_arrays, UnderlyingPatternKind::MaximalTandemArray)
    })
  }

  pub(super) fn find_tandem_arrays_and_put_to_context(
    context: &mut PipelineContext,
    config: &UserDataImpl,
    patterns_finder: impl Fn(&Vec<Vec<u64>>, usize, bool) -> Vec<Vec<SubArrayInTraceInfo>>,
    underlying_pattern_kind: UnderlyingPatternKind
  ) -> Result<(), PipelinePartExecutionError> {
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
    let array_length = *config.concrete(TANDEM_ARRAY_LENGTH_KEY.key()).unwrap() as usize;

    let hashed_log = Self::create_hashed_event_log(config, log);

    let arrays = patterns_finder(&hashed_log, array_length, false);

    context.put_concrete(UNDERLYING_PATTERN_KIND.key(), underlying_pattern_kind);
    context.put_concrete(HASHES_EVENT_LOG_KEY.key(), hashed_log);
    context.put_concrete(PATTERNS_KEY.key(), arrays);

    Ok(())
  }

  pub(super) fn find_repeats_and_put_to_context(
    context: &mut PipelineContext,
    config: &UserDataImpl,
    patterns_finder: impl Fn(&Vec<Vec<u64>>, &PatternsDiscoveryStrategy) -> Vec<Vec<SubArrayInTraceInfo>>,
    underlying_pattern_kind: UnderlyingPatternKind
  ) -> Result<(), PipelinePartExecutionError> {
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
    let strategy = Self::get_user_data(config, &PATTERNS_DISCOVERY_STRATEGY_KEY)?;

    let hashed_log = Self::create_hashed_event_log(config, log);

    let repeats = patterns_finder(&hashed_log, &strategy);

    context.put_concrete(UNDERLYING_PATTERN_KIND.key(), underlying_pattern_kind);
    context.put_concrete(HASHES_EVENT_LOG_KEY.key(), hashed_log);
    context.put_concrete(PATTERNS_KEY.key(), repeats);

    Ok(())
  }

  pub(super) fn find_patterns(context: &mut PipelineContext, config: &UserDataImpl) -> Result<(), PipelinePartExecutionError> {
    let patterns_kind = *Self::get_user_data(config, &PATTERNS_KIND_KEY)?;

    match patterns_kind {
      PatternsKindDto::PrimitiveTandemArrays => {
        Self::find_tandem_arrays_and_put_to_context(context, config, find_primitive_tandem_arrays, patterns_kind.into())?
      }
      PatternsKindDto::MaximalTandemArrays => {
        Self::find_tandem_arrays_and_put_to_context(context, config, find_maximal_tandem_arrays, patterns_kind.into())?
      }
      PatternsKindDto::MaximalRepeats => Self::find_repeats_and_put_to_context(context, config, find_maximal_repeats, patterns_kind.into())?,
      PatternsKindDto::SuperMaximalRepeats => Self::find_repeats_and_put_to_context(context, config, find_super_maximal_repeats, patterns_kind.into())?,
      PatternsKindDto::NearSuperMaximalRepeats => {
        Self::find_repeats_and_put_to_context(context, config, find_near_super_maximal_repeats, patterns_kind.into())?
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

      context.put_concrete(TRACE_ACTIVITIES_KEY.key(), find_loops_strict(log, &hashed_log, *max_array_length as usize));
      context.put_concrete(UNDERLYING_PATTERN_KIND.key(), UnderlyingPatternKind::StrictLoop);

      Ok(())
    })
  }
}
