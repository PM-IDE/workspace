use super::traces_params::TracesClusteringParams;
use crate::features::clustering::common::adjust_dbscan_labels;
use crate::features::clustering::traces::common::{calculate_distance, do_clusterize_log_by_traces, BestSilhouetteLabels};
use crate::utils::silhouette::silhouette_score;
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
use log::warn;

pub fn clusterize_log_by_traces_dbscan<TLog: EventLog>(
  params: &mut TracesClusteringParams<TLog>,
  tolerance: f64,
  min_points: usize,
  put_noise_events_in_one_cluster: bool
) -> Result<(Vec<TLog>, LabeledDataset), ClusteringError> {
  do_clusterize_log_by_traces(params, |params, nn_search_algorithm, dataset| {
    let clusters = Dbscan::params_with(min_points, DistanceWrapper::new(params.distance), nn_search_algorithm)
      .tolerance(tolerance)
      .transform(dataset.records());

    match clusters {
      Ok(clusters) => Ok(adjust_dbscan_labels(clusters, put_noise_events_in_one_cluster)),
      Err(err) => Err(ClusteringError::RawError(err.to_string()))
    }
  })
}

pub fn clusterize_log_by_traces_dbscan_grid_search<TLog: EventLog>(
  params: &mut TracesClusteringParams<TLog>,
  min_points_vec: &Vec<usize>,
  tolerances: &Vec<f64>,
  put_noise_events_in_one_cluster: bool
) -> Result<(Vec<TLog>, LabeledDataset), ClusteringError> {
  do_clusterize_log_by_traces(params, |params, nn_algo, dataset| {
    let mut best_labels = BestSilhouetteLabels::new();

    for min_points in min_points_vec {
      for tolerance in tolerances {
        let clusters = Dbscan::params_with(*min_points, DistanceWrapper::new(params.distance), nn_algo.clone())
          .tolerance(*tolerance)
          .transform(dataset.records());

        let clusters = match clusters {
          Ok(clusters) => clusters,
          Err(err) => return Err(ClusteringError::RawError(err.to_string()))
        };

        let labels = adjust_dbscan_labels(clusters.clone(), put_noise_events_in_one_cluster);
        best_labels.process(labels, &|first, second| calculate_distance(params.distance, dataset, first, second));
      }
    }

    match best_labels.labels() {
      None => Err(ClusteringError::RawError("Best labels were None".to_string())),
      Some(labels) => Ok(labels.clone())
    }
  })
}