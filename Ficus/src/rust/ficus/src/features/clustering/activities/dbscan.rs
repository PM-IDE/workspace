use linfa::traits::Transformer;
use linfa_clustering::Dbscan;
use linfa_nn::KdTree;

use crate::{
  event_log::core::event_log::EventLog,
  features::clustering::{
    common::{create_colors_vector, transform_to_ficus_dataset},
    error::ClusteringResult,
  },
  utils::{dataset::dataset::LabeledDataset, distance::distance::DistanceWrapper},
};
use crate::features::clustering::common::adjust_dbscan_labels;
use super::{activities_common::create_dataset, activities_params::ActivitiesClusteringParams, merging::merge_activities};

pub fn clusterize_activities_dbscan<TLog: EventLog>(params: &mut ActivitiesClusteringParams<TLog>, min_points: usize) -> ClusteringResult {
  let (dataset, processed, classes_names) = create_dataset(&params.vis_params)?;
  let clusters = Dbscan::params_with(min_points, DistanceWrapper::new(params.distance), KdTree)
    .tolerance(params.tolerance)
    .transform(dataset.records())
    .unwrap();

  merge_activities(
    params.vis_params.common_vis_params.log,
    params.vis_params.traces_activities,
    &processed.iter().map(|x| x.0.clone()).collect(),
    &clusters,
  );

  let ficus_dataset = transform_to_ficus_dataset(
    &dataset,
    processed.iter().map(|x| x.0.borrow().name().as_ref().as_ref().to_owned()).collect(),
    classes_names,
  );

  let labels = adjust_dbscan_labels(clusters);

  let colors = create_colors_vector(&labels, params.vis_params.common_vis_params.colors_holder);
  Ok(LabeledDataset::new(ficus_dataset, labels, colors))
}
