use crate::event_log::core::event_log::EventLog;
use crate::features::clustering::error::ClusteringError;
use crate::features::clustering::traces::common::{calculate_distance, do_clusterize_log_by_traces, BestSilhouetteLabels};
use crate::features::clustering::traces::traces_params::TracesClusteringParams;
use crate::utils::dataset::dataset::LabeledDataset;
use crate::utils::distance::distance::DistanceWrapper;
use linfa::prelude::{Fit, Predict};
use linfa_clustering::KMeans;

pub fn clusterize_log_by_traces_kmeans_grid_search<TLog: EventLog>(
  params: &mut TracesClusteringParams<TLog>,
  max_iterations_count: u64,
  tolerance: f64,
) -> Result<(Vec<TLog>, LabeledDataset), ClusteringError> {
  do_clusterize_log_by_traces(params, |params, _, dataset| {
    let mut best_labels = BestSilhouetteLabels::new();

    for clusters_count in 2..dataset.targets().len() - 1 {
      let model = KMeans::params_with(clusters_count, rand::thread_rng(), DistanceWrapper::new(params.distance))
        .max_n_iterations(max_iterations_count)
        .tolerance(tolerance)
        .fit(&dataset)
        .expect("KMeans fitted");

      let clustered_dataset = model.predict(dataset.clone());
      let labels = clustered_dataset.targets().to_vec();
      best_labels.process(labels, &|first, second| calculate_distance(params.distance, dataset, first, second));
    }

    match best_labels.labels() {
      None => Err(ClusteringError::RawError("Best labels were None".to_string())),
      Some(labels) => Ok(labels.clone()),
    }
  })
}
