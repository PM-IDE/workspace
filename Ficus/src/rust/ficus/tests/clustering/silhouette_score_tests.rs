use approx::assert_abs_diff_eq;
use ficus::features::clustering::traces::common::calculate_distance;
use ficus::utils::distance::distance::{DistanceWrapper, FicusDistance};
use ficus::utils::silhouette::silhouette_score;
use linfa::metrics::SilhouetteScore;
use linfa::prelude::Transformer;
use linfa::DatasetBase;
use linfa_clustering::Dbscan;
use linfa_nn::distance::L2Dist;
use linfa_nn::CommonNearestNeighbour::KdTree;
use ndarray::{Array1, Array2};

#[test]
pub fn test_silhouette_score() {
  let array = vec![
    vec![0., 1., 1., 1., 1., 0., 0., 0., 0., 1., 0., 1., 0.],
    vec![0., 1., 1., 1., 1., 0., 0., 0., 0., 1., 0., 1., 0.],
    vec![0., 1., 1., 1., 1., 0., 1., 1., 0., 1., 0., 1., 0.],
    vec![0., 0., 1., 0., 1., 0., 0., 1., 0., 1., 0., 1., 0.],
    vec![0., 1., 1., 1., 1., 0., 0., 0., 0., 1., 0., 1., 0.],
    vec![0., 1., 0., 0., 0., 0., 0., 0., 0., 1., 1., 1., 0.],
    vec![0., 1., 0., 0., 1., 0., 0., 0., 0., 1., 1., 1., 0.],
    vec![0., 0., 0., 0., 1., 0., 1., 1., 0., 1., 1., 1., 0.],
    vec![0., 1., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 1.],
    vec![0., 1., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 1.],
    vec![0., 1., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 1.],
  ];

  const MIN_POINTS: usize = 2;
  const DISTANCE: DistanceWrapper = DistanceWrapper::L2(L2Dist);
  const TOLERANCE: f64 = 0.1;
  
  let plain_dataset = array.iter().flat_map(|x| x).map(|x| *x).collect::<Vec<f64>>();

  let dataset = DatasetBase::from(Array2::from_shape_vec((array.len(), array[0].len()), plain_dataset).unwrap());

  let labels = Dbscan::params_with(MIN_POINTS, DISTANCE, KdTree)
    .tolerance(TOLERANCE)
    .transform(dataset.records())
    .unwrap();
  
  let labels = labels.iter().map(|l| if l.is_none() { 0 } else { l.unwrap() + 1 }).collect::<Vec<usize>>();
  
  let our_score = silhouette_score(labels.clone(), |first, second| calculate_distance(FicusDistance::L2, &dataset, first, second));

  let dataset = dataset.with_targets(Array1::from_iter(labels));
  let expected_score = dataset.silhouette_score().unwrap();
  
  assert_abs_diff_eq!(our_score, expected_score);
}