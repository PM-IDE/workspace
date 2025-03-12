use std::str::FromStr;

use crate::{
  event_log::core::event_log::EventLog, features::clustering::common::CommonVisualizationParams, utils::distance::distance::FicusDistance,
};

pub struct TracesClusteringParams<'a, TLog>
where
  TLog: EventLog,
{
  pub vis_params: CommonVisualizationParams<'a, TLog>,
  pub tolerance: f64,
  pub distance: FicusDistance,
  pub repr_source: TracesRepresentationSource,
  pub feature_count_kind: FeatureCountKind,
}

#[derive(Copy, Clone)]
pub enum TracesRepresentationSource {
  Events,
  UnderlyingEvents,
  DeepestUnderlyingEvents,
}

impl FromStr for TracesRepresentationSource {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Events" => Ok(Self::Events),
      "UnderlyingEvents" => Ok(Self::UnderlyingEvents),
      "DeepestUnderlyingEvents" => Ok(Self::DeepestUnderlyingEvents),
      _ => Err(()),
    }
  }
}

#[derive(Copy, Clone)]
pub enum FeatureCountKind {
  One,
  Count,
  OneIfMoreThanMaxFromAllFeatures(f64)
}
