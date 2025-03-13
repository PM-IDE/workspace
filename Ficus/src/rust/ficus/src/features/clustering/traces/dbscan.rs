use super::traces_params::TracesClusteringParams;
use crate::features::clustering::traces::common::do_clusterize_log_by_traces;
use crate::{
  event_log::core::event_log::EventLog,
  features::clustering::error::ClusteringError,
  utils::{
    dataset::dataset::LabeledDataset,
    distance::distance::DistanceWrapper,
  },
};
use linfa::traits::Transformer;
use linfa_clustering::Dbscan;

pub fn clusterize_log_by_traces_dbscan<TLog: EventLog>(
  params: &mut TracesClusteringParams<TLog>,
  min_points: usize,
) -> Result<(Vec<TLog>, LabeledDataset), ClusteringError> {
  do_clusterize_log_by_traces(params, |params, nn_search_algorithm, dataset| {
    let clusters = Dbscan::params_with(min_points, DistanceWrapper::new(params.distance), nn_search_algorithm)
      .tolerance(params.tolerance)
      .transform(dataset.records());

    match clusters {
      Ok(clusters) => Ok(clusters),
      Err(err) => Err(ClusteringError::RawError(err.to_string()))
    }
  })
}
