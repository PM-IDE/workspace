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

use super::{
    context::PipelineContext, errors::pipeline_errors::PipelinePartExecutionError, keys::context_keys::ContextKeys,
    pipelines::PipelinePartFactory,
};

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
        Self::create_pipeline_part(Self::FIND_MAXIMAL_REPEATS, &|context, _, keys, config| {
            Self::find_repeats_and_put_to_context(context, keys, config, find_maximal_repeats)
        })
    }

    pub(super) fn find_super_maximal_repeats() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::FIND_SUPER_MAXIMAL_REPEATS, &|context, _, keys, config| {
            Self::find_repeats_and_put_to_context(context, keys, config, find_super_maximal_repeats)
        })
    }

    pub(super) fn find_near_super_maximal_repeats() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::FIND_NEAR_SUPER_MAXIMAL_REPEATS, &|context, _, keys, config| {
            Self::find_repeats_and_put_to_context(context, keys, config, find_near_super_maximal_repeats)
        })
    }

    pub(super) fn find_primitive_tandem_arrays() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::FIND_PRIMITIVE_TANDEM_ARRAYS, &|context, _, keys, config| {
            Self::find_tandem_arrays_and_put_to_context(context, keys, &config, find_primitive_tandem_arrays)
        })
    }

    pub(super) fn find_maximal_tandem_arrays() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::FIND_MAXIMAL_TANDEM_ARRAYS, &|context, _, keys, config| {
            Self::find_tandem_arrays_and_put_to_context(context, keys, &config, find_maximal_tandem_arrays)
        })
    }

    pub(super) fn find_tandem_arrays_and_put_to_context(
        context: &mut PipelineContext,
        keys: &ContextKeys,
        config: &UserDataImpl,
        patterns_finder: impl Fn(&Vec<Vec<u64>>, usize) -> Vec<Vec<SubArrayInTraceInfo>>,
    ) -> Result<(), PipelinePartExecutionError> {
        let log = Self::get_user_data(context, keys.event_log())?;
        let array_length = *config.concrete(keys.tandem_array_length().key()).unwrap() as usize;

        let hashed_log = Self::create_hashed_event_log(config, keys, log);

        let arrays = patterns_finder(&hashed_log, array_length);

        context.put_concrete(keys.hashes_event_log().key(), hashed_log);
        context.put_concrete(keys.patterns().key(), arrays);

        Ok(())
    }

    pub(super) fn find_repeats_and_put_to_context(
        context: &mut PipelineContext,
        keys: &ContextKeys,
        config: &UserDataImpl,
        patterns_finder: impl Fn(&Vec<Vec<u64>>, &PatternsDiscoveryStrategy) -> Vec<Vec<SubArrayInTraceInfo>>,
    ) -> Result<(), PipelinePartExecutionError> {
        let log = Self::get_user_data(context, keys.event_log())?;
        let strategy = Self::get_user_data(config, keys.patterns_discovery_strategy())?;

        let hashed_log = Self::create_hashed_event_log(config, keys, log);

        let repeats = patterns_finder(&hashed_log, &strategy);

        context.put_concrete(keys.hashes_event_log().key(), hashed_log);
        context.put_concrete(keys.patterns().key(), repeats);

        Ok(())
    }

    pub(super) fn find_patterns(
        context: &mut PipelineContext,
        keys: &ContextKeys,
        config: &UserDataImpl,
    ) -> Result<(), PipelinePartExecutionError> {
        let patterns_kind = Self::get_user_data(config, keys.patterns_kind())?;
        match patterns_kind {
            PatternsKindDto::PrimitiveTandemArrays => {
                Self::find_tandem_arrays_and_put_to_context(context, keys, config, find_primitive_tandem_arrays)?
            }
            PatternsKindDto::MaximalTandemArrays => {
                Self::find_tandem_arrays_and_put_to_context(context, keys, config, find_maximal_tandem_arrays)?
            }
            PatternsKindDto::MaximalRepeats => Self::find_repeats_and_put_to_context(context, keys, config, find_maximal_repeats)?,
            PatternsKindDto::SuperMaximalRepeats => {
                Self::find_repeats_and_put_to_context(context, keys, config, find_super_maximal_repeats)?
            }
            PatternsKindDto::NearSuperMaximalRepeats => {
                Self::find_repeats_and_put_to_context(context, keys, config, find_near_super_maximal_repeats)?
            }
        };

        let activity_level = Self::get_user_data(config, keys.activity_level())?;
        Self::do_discover_activities(context, keys, *activity_level, config)?;

        Ok(())
    }
}
