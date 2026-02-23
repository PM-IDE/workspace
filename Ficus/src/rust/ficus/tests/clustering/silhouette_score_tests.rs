use approx::assert_abs_diff_eq;
use ficus::{
  features::clustering::traces::common::calculate_distance,
  utils::{
    distance::distance::{DistanceWrapper, FicusDistance},
    silhouette::{SilhouetteScoreError, silhouette_score},
  },
};
use linfa::{DatasetBase, metrics::SilhouetteScore, prelude::Transformer};
use linfa_clustering::Dbscan;
use linfa_nn::{CommonNearestNeighbour::KdTree, distance::L2Dist};
use ndarray::{Array1, Array2};

#[test]
pub fn test_silhouette_score() {
  execute_silhouette_score_test(vec![
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
  ])
}

#[test]
pub fn test_single_label() {
  let labels = vec![0; 123];
  assert_eq!(
    silhouette_score(&labels, |_, _| 0.).err().unwrap(),
    SilhouetteScoreError::InappropriateLabelsCount
  )
}

#[test]
pub fn test_all_different_labels() {
  let labels = (0..123).into_iter().collect();
  assert_eq!(
    silhouette_score(&labels, |_, _| 0.).err().unwrap(),
    SilhouetteScoreError::InappropriateLabelsCount
  )
}

#[test]
pub fn test_silhouette_score_empty_labels() {
  let result = silhouette_score(&vec![], |_, _| 0.);
  assert_eq!(result.err().unwrap(), SilhouetteScoreError::NotEnoughSamples)
}

fn execute_silhouette_score_test(raw_dataset: Vec<Vec<f64>>) {
  const MIN_POINTS: usize = 2;
  const DISTANCE: DistanceWrapper = DistanceWrapper::L2(L2Dist);
  const TOLERANCE: f64 = 0.1;

  let plain_dataset = raw_dataset.iter().flat_map(|x| x).map(|x| *x).collect::<Vec<f64>>();

  let dataset = DatasetBase::from(Array2::from_shape_vec((raw_dataset.len(), raw_dataset[0].len()), plain_dataset).unwrap());

  let labels = Dbscan::params_with(MIN_POINTS, DISTANCE, KdTree)
    .tolerance(TOLERANCE)
    .transform(dataset.records())
    .unwrap();

  let labels = labels
    .iter()
    .map(|l| if l.is_none() { 0 } else { l.unwrap() + 1 })
    .collect::<Vec<usize>>();

  let our_score = silhouette_score(&labels.clone(), |first, second| {
    calculate_distance(FicusDistance::L2, &dataset, first, second)
  })
  .ok()
  .unwrap();

  let dataset = dataset.with_targets(Array1::from_iter(labels));
  let expected_score = dataset.silhouette_score().unwrap();

  assert_abs_diff_eq!(our_score, expected_score);
}
