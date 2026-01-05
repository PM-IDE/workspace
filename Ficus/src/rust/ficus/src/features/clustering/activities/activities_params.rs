use crate::{
  features::clustering::common::CommonVisualizationParams, pipelines::aliases::TracesActivities, utils::distance::distance::FicusDistance,
};
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ActivityRepresentationSource {
  EventClasses,
  SubTraces,
  SubTracesUnderlyingEvents,
}

impl FromStr for ActivityRepresentationSource {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "EventClasses" => Ok(Self::EventClasses),
      "SubTraces" => Ok(Self::SubTraces),
      "SubTracesUnderlyingEvents" => Ok(Self::SubTracesUnderlyingEvents),
      _ => Err(()),
    }
  }
}

pub struct ActivitiesVisualizationParams<'a> {
  pub common_vis_params: CommonVisualizationParams<'a>,
  pub traces_activities: &'a mut TracesActivities,
  pub activity_level: usize,
  pub activities_repr_source: ActivityRepresentationSource,
}

pub struct ActivitiesClusteringParams<'a> {
  pub(super) vis_params: ActivitiesVisualizationParams<'a>,
  pub(super) tolerance: f64,
  pub(super) distance: FicusDistance,
}

impl<'a> ActivitiesClusteringParams<'a> {
  pub fn new(vis_params: ActivitiesVisualizationParams<'a>, tolerance: f64, distance: FicusDistance) -> Option<Self> {
    if distance == FicusDistance::Levenshtein {
      None
    } else {
      Some(Self {
        vis_params,
        tolerance,
        distance,
      })
    }
  }
}
