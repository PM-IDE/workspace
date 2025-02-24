use std::str::FromStr;

use crate::{
  event_log::core::event_log::EventLog, features::clustering::common::CommonVisualizationParams, pipelines::aliases::TracesActivities,
  utils::distance::distance::FicusDistance,
};

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

pub struct ActivitiesVisualizationParams<'a, TLog>
where
  TLog: EventLog,
{
  pub common_vis_params: CommonVisualizationParams<'a, TLog>,
  pub traces_activities: &'a mut TracesActivities,
  pub activity_level: usize,
  pub activities_repr_source: ActivityRepresentationSource,
}

pub struct ActivitiesClusteringParams<'a, TLog>
where
  TLog: EventLog,
{
  pub(super) vis_params: ActivitiesVisualizationParams<'a, TLog>,
  pub(super) tolerance: f64,
  pub(super) distance: FicusDistance,
}

impl<'a, TLog> ActivitiesClusteringParams<'a, TLog>
where
  TLog: EventLog,
{
  pub fn new(vis_params: ActivitiesVisualizationParams<'a, TLog>, tolerance: f64, distance: FicusDistance) -> Option<Self> {
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
