use crate::event_log::core::event_log::EventLog;
use crate::features::clustering::error::ClusteringError;
use crate::features::clustering::traces::common::{calculate_distance, do_clusterize_log_by_traces};
use crate::features::clustering::traces::traces_params::TracesClusteringParams;
use crate::utils::dataset::dataset::LabeledDataset;
use crate::utils::distance::distance::DistanceWrapper;
use crate::utils::silhouette::silhouette_score;
use linfa::prelude::{Fit, Predict};
use linfa_clustering::KMeans;
use log::warn;

pub fn clusterize_log_by_traces_kmeans_grid_search<TLog: EventLog>(
  params: &mut TracesClusteringParams<TLog>,
  max_iterations_count: u64,
  tolerance: f64,
) -> Result<(Vec<TLog>, LabeledDataset), ClusteringError> {
  do_clusterize_log_by_traces(params, |params, _, dataset| {
    let mut best_score = -1.;
    let mut best_labels = None;

    for clusters_count in 2..dataset.targets().len() - 1 {
      let model = KMeans::params_with(clusters_count, rand::thread_rng(), DistanceWrapper::new(params.distance))
        .max_n_iterations(max_iterations_count)
        .tolerance(tolerance)
        .fit(&dataset)
        .expect("KMeans fitted");

      let clustered_dataset = model.predict(dataset.clone());
      let score = match silhouette_score(&clustered_dataset.targets().to_vec(), |first, second| {
        calculate_distance(params.distance, dataset, first, second)
      }) {
        Ok(score) => score,
        Err(err) => {
          warn!("Failed to calculate silhouette score, skipping those labels, reason: {}", err.to_string());
          continue
        }
      };

      if score > best_score {
        best_labels = Some(clustered_dataset.targets.to_vec().clone());
        best_score = score;
      }
    }

    Ok(best_labels.unwrap())
  })
}