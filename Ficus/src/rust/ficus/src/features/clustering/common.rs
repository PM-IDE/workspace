use linfa::DatasetBase;
use ndarray::{Array1, ArrayBase, Dim, OwnedRepr};

use crate::{
  event_log::core::event_log::EventLog,
  utils::{
    colors::{Color, ColorsHolder},
    dataset::dataset::FicusDataset,
  },
};

pub(super) type MyDataset = DatasetBase<ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>, Array1<()>>;
pub(super) type ClusteredDataset = DatasetBase<ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>, ArrayBase<OwnedRepr<usize>, Dim<[usize; 1]>>>;

pub struct CommonVisualizationParams<'a, TLog>
where
  TLog: EventLog,
{
  pub log: &'a TLog,
  pub colors_holder: &'a mut ColorsHolder,
  pub class_extractor: Option<String>,
}

pub fn transform_to_ficus_dataset(dataset: &MyDataset, processed: Vec<String>, classes_names: Vec<String>) -> FicusDataset {
  let rows_count = dataset.records().shape()[0];
  let cols_count = dataset.records().shape()[1];

  let mut matrix = vec![];
  for i in 0..rows_count {
    let mut vec = vec![];
    for j in 0..cols_count {
      vec.push(*dataset.records.get([i, j]).unwrap());
    }

    matrix.push(vec);
  }

  FicusDataset::new(matrix, classes_names, processed)
}

pub(super) fn create_colors_vector(labels: &Vec<usize>, colors_holder: &mut ColorsHolder) -> Vec<Color> {
  labels
    .iter()
    .map(|x| colors_holder.get_or_create(&create_cluster_name(*x)))
    .collect()
}

pub fn scale_raw_dataset_min_max(vector: &mut Vec<f64>, objects_count: usize, features_count: usize) {
  for i in 0..features_count {
    let mut max = f64::MIN;
    let mut min = f64::MAX;

    for j in 0..objects_count {
      let index = i + j * features_count;
      max = max.max(vector[index]);
      min = min.min(vector[index]);
    }

    for j in 0..objects_count {
      let index = i + j * features_count;
      vector[index] = if max == min { 1.0 } else { (vector[index] - min) / (max - min) }
    }
  }
}

pub fn create_cluster_name(cluster_index: usize) -> String {
  format!("CLUSTER_{}", cluster_index)
}

pub(super) fn adjust_dbscan_labels(clusters: Array1<Option<usize>>, put_noise_events_in_one_cluster: bool) -> Vec<usize> {
  let mut next_label = clusters.iter().filter(|c| c.is_some()).map(|c| c.as_ref().unwrap().clone()).max().unwrap_or(0);

  clusters
    .into_raw_vec()
    .iter()
    .map(|x| if x.is_none() {
      if !put_noise_events_in_one_cluster {
        next_label += 1; 
      }

      next_label
    } else { x.unwrap() })
    .collect()
}