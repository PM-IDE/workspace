use crate::{
  event_log::xes::xes_event_log::XesEventLogImpl,
  features::clustering::{
    error::ClusteringError,
    traces::{
      common::{calculate_distance, do_clusterize_log_by_traces, BestSilhouetteLabels},
      traces_params::TracesClusteringParams,
    },
  },
  utils::{dataset::dataset::LabeledDataset, distance::distance::DistanceWrapper},
};
use linfa::prelude::{Fit, Predict};
use linfa_clustering::KMeans;

pub fn clusterize_log_by_traces_kmeans_grid_search(
  params: &mut TracesClusteringParams,
  max_iterations_count: u64,
  tolerance: f64,
) -> Result<(Vec<XesEventLogImpl>, LabeledDataset), ClusteringError> {
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
