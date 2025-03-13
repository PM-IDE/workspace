use linfa::prelude::Predict;
use linfa::prelude::Fit;
use linfa_clustering::KMeans;
use std::{
  cell::RefCell,
  collections::{HashMap, HashSet},
  rc::Rc,
};

use linfa::{traits::Transformer, DatasetBase};
use linfa::metrics::SilhouetteScore;
use linfa_clustering::Dbscan;
use linfa_nn::CommonNearestNeighbour;
use linfa_nn::CommonNearestNeighbour::{KdTree, LinearSearch};
use linfa_nn::distance::Distance;
use ndarray::{Array1, Array2};
use prost::bytes::BufMut;
use crate::{
  event_log::core::{
    event::{event::Event, event_hasher::RegexEventHasher},
    event_log::EventLog,
    trace::trace::Trace,
  },
  features::{
    analysis::patterns::activity_instances::{create_vector_of_immediate_underlying_events, create_vector_of_underlying_events},
    clustering::{
      common::{create_colors_vector, scale_raw_dataset_min_max, transform_to_ficus_dataset, MyDataset},
      error::ClusteringError,
    },
  },
  utils::{
    dataset::dataset::LabeledDataset,
    distance::distance::{DistanceWrapper, FicusDistance},
  },
};

use super::traces_params::{FeatureCountKind, TracesClusteringParams, TracesRepresentationSource};

pub fn clusterize_log_by_traces_kmeans_grid_search<TLog: EventLog>(
  params: &mut TracesClusteringParams<TLog>,
  max_iterations_count: u64
) -> Result<(Vec<TLog>, LabeledDataset), ClusteringError> {
  do_clusterize_log_by_traces(params, |params, _, dataset| {
    let mut best_score = -1.;
    let mut best_labels = None;

    for clusters_count in 2..dataset.targets().len() - 1 {
      let model = KMeans::params_with(clusters_count, rand::thread_rng(), DistanceWrapper::new(params.distance))
        .max_n_iterations(max_iterations_count)
        .tolerance(params.tolerance)
        .fit(&dataset)
        .expect("KMeans fitted");

      let clustered_dataset = model.predict(dataset.clone());
      let score = silhouette_score(clustered_dataset.targets().to_vec(), |first, second| {
        let first_record = dataset.records.row(first);
        let second_record = dataset.records.row(second);

        let distance_wrapper = DistanceWrapper::new(params.distance);
        distance_wrapper.distance(first_record, second_record)
      });

      if score > best_score {
        best_labels = Some(clustered_dataset.targets.clone());
        best_score = score;
      }
    }

    Ok(best_labels.unwrap().iter().map(|l| Some(*l)).collect())
  })
}

fn silhouette_score(labels: Vec<usize>, distance_func: impl Fn(usize, usize) -> f64) -> f64 {
  let mut clusters_to_indices: HashMap<usize, Vec<usize>> = HashMap::new();
  for i in 0..labels.len() {
    let label = *labels.get(i).unwrap();
    if let Some(indices) = clusters_to_indices.get_mut(&label) {
      indices.push(i);
    } else {
      clusters_to_indices.insert(label, vec![i]);
    }
  }

  let mut score = 0.;
  for (current_cluster_index, current_cluster_indices) in &clusters_to_indices {
    for current_label in current_cluster_indices {
      let mut a_x = 0.;
      for other_index_from_this_cluster in current_cluster_indices {
        a_x += distance_func(*current_label, *other_index_from_this_cluster);
      }

      a_x /= current_cluster_indices.len() as f64;

      let mut b_x = None;

      for (other_cluster_index, other_cluster_indices) in &clusters_to_indices {
        if *other_cluster_index == *current_cluster_index {
          continue;
        }

        let mut current_b_x = 0.;
        for other_label_from_other_cluster in other_cluster_indices {
          current_b_x += distance_func(*current_label, *other_label_from_other_cluster);
        }

        current_b_x /= other_cluster_indices.len() as f64;

        b_x = Some(if b_x.is_none() {
          current_b_x
        } else {
          current_b_x.min(b_x.unwrap())
        })
      }

      let b_x = b_x.unwrap_or_else(|| 0.);
      score += (b_x - a_x) / a_x.max(b_x);
    }
  }

  score / labels.len() as f64
}

fn do_clusterize_log_by_traces<TLog: EventLog>(
  params: &mut TracesClusteringParams<TLog>,
  clustering_func: impl Fn(&mut TracesClusteringParams<TLog>, CommonNearestNeighbour, &MyDataset) -> Result<Array1<Option<usize>>, ClusteringError>
) -> Result<(Vec<TLog>, LabeledDataset), ClusteringError> {
  let class_extractor = params.vis_params.class_extractor.as_ref();
  let traces_dataset = create_traces_dataset(
    params.vis_params.log,
    &params.distance,
    class_extractor,
    params.feature_count_kind,
    &params.repr_source,
  );

  let (dataset, objects, features) = traces_dataset?;

  let nn_search_algorithm = match params.distance {
    FicusDistance::Levenshtein | FicusDistance::Length | FicusDistance::LCS => LinearSearch,
    FicusDistance::Cosine | FicusDistance::L1 | FicusDistance::L2 => KdTree
  };

  let clusters = clustering_func(params, nn_search_algorithm, &dataset)?;

  let ficus_dataset = transform_to_ficus_dataset(&dataset, objects, features);

  let labels = clusters
    .into_raw_vec()
    .iter()
    .map(|x| if x.is_none() { 0 } else { x.unwrap() + 1 })
    .collect();

  let mut new_logs: HashMap<usize, TLog> = HashMap::new();
  for (trace, label) in params.vis_params.log.traces().iter().zip(&labels) {
    let trace_copy = trace.borrow().clone();
    if let Some(cluster_log) = new_logs.get_mut(label) {
      cluster_log.push(Rc::new(RefCell::new(trace_copy)));
    } else {
      let mut cluster_log = TLog::empty();
      cluster_log.push(Rc::new(RefCell::new(trace_copy)));

      new_logs.insert(label.to_owned(), cluster_log);
    }
  }

  let new_logs = new_logs.into_iter().map(|x| x.1).collect();
  let colors = create_colors_vector(&labels, &mut params.vis_params.colors_holder);

  Ok((new_logs, LabeledDataset::new(ficus_dataset, labels, colors)))
}

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

fn create_traces_dataset<TLog: EventLog>(
  log: &TLog,
  distance: &FicusDistance,
  class_extractor: Option<&String>,
  feature_count_kind: FeatureCountKind,
  trace_repr_source: &TracesRepresentationSource,
) -> Result<(MyDataset, Vec<String>, Vec<String>), ClusteringError> {
  match distance {
    FicusDistance::Cosine | FicusDistance::L1 | FicusDistance::L2 => {
      create_traces_dataset_default(log, class_extractor, feature_count_kind, trace_repr_source)
    }
    FicusDistance::Levenshtein | FicusDistance::Length | FicusDistance::LCS => {
      create_traces_dataset_levenshtein(log, class_extractor, trace_repr_source)
    },
  }
}

fn create_traces_dataset_default<TLog: EventLog>(
  log: &TLog,
  class_extractor: Option<&String>,
  feature_count_kind: FeatureCountKind,
  trace_repr_source: &TracesRepresentationSource,
) -> Result<(MyDataset, Vec<String>, Vec<String>), ClusteringError> {
  create_traces_dataset_default_internal(log, class_extractor, feature_count_kind, |trace| {
    create_trace_representation::<TLog>(trace, trace_repr_source)
  })
}

fn create_trace_representation<TLog: EventLog>(
  trace: &TLog::TTrace,
  trace_repr_source: &TracesRepresentationSource,
) -> Vec<Rc<RefCell<TLog::TEvent>>> {
  match trace_repr_source {
    TracesRepresentationSource::Events => trace.events().clone(),
    TracesRepresentationSource::UnderlyingEvents => {
      let mut events = vec![];
      for event in trace.events() {
        for event in create_vector_of_immediate_underlying_events::<TLog>(event) {
          events.push(event);
        }
      }

      events
    }
    TracesRepresentationSource::DeepestUnderlyingEvents => {
      let mut events = vec![];
      for event in trace.events() {
        for underlying_event in create_vector_of_underlying_events::<TLog>(event) {
          events.push(underlying_event);
        }
      }

      events
    }
  }
}

fn create_traces_dataset_default_internal<TLog: EventLog>(
  log: &TLog,
  class_extractor: Option<&String>,
  feature_count_kind: FeatureCountKind,
  trace_repr_creator: impl Fn(&TLog::TTrace) -> Vec<Rc<RefCell<TLog::TEvent>>>,
) -> Result<(MyDataset, Vec<String>, Vec<String>), ClusteringError> {
  let regex_hasher = match class_extractor.as_ref() {
    Some(class_extractor) => Some(RegexEventHasher::new(class_extractor).ok().unwrap()),
    None => None,
  };

  let mut processed_traces = vec![];
  for trace in log.traces() {
    let trace = trace.borrow();
    processed_traces.push(trace_repr_creator(&trace));
  }

  let mut all_event_classes = HashSet::new();
  for trace in &processed_traces {
    for event in trace {
      let event = event.borrow();
      let processed_event_name = match regex_hasher.as_ref() {
        Some(regex_hasher) => regex_hasher.transform(event.name()),
        None => event.name(),
      };

      all_event_classes.insert(processed_event_name.to_owned());
    }
  }

  let mut all_event_classes = all_event_classes.into_iter().collect::<Vec<String>>();
  all_event_classes.sort();

  let mut raw_dataset = vec![];
  for trace in &processed_traces {
    let mut events_counts: HashMap<String, usize> = HashMap::new();

    for event in trace {
      let event = event.borrow();
      let processed_event_name = match regex_hasher.as_ref() {
        Some(regex_hasher) => regex_hasher.transform(event.name()).to_owned(),
        None => event.name().to_owned(),
      };

      *events_counts.entry(processed_event_name).or_default() += 1;
    }

    let max_count = *events_counts.values().max().unwrap() as f64;
    for class in &all_event_classes {
      raw_dataset.push(if let Some(count) = events_counts.get(class) {
        match feature_count_kind {
          FeatureCountKind::One => 1,
          FeatureCountKind::Count => *count,
          FeatureCountKind::OneIfMoreThanMaxFromAllFeatures(percent_from_max) => {
            if *count as f64 > percent_from_max * max_count {
              1
            } else {
              0
            }
          }
        }
      } else {
        0
      } as f64);
    }
  }

  scale_raw_dataset_min_max(&mut raw_dataset, processed_traces.len(), all_event_classes.len());

  let shape = (processed_traces.len(), all_event_classes.len());
  let array = match Array2::from_shape_vec(shape, raw_dataset) {
    Ok(score) => score,
    Err(_) => return Err(ClusteringError::FailedToCreateNdArray),
  };

  Ok((
    DatasetBase::from(array),
    (0..processed_traces.len()).into_iter().map(|x| format!("Trace_{}", x)).collect(),
    all_event_classes,
  ))
}

fn create_traces_dataset_levenshtein<TLog: EventLog>(
  log: &TLog,
  class_extractor: Option<&String>,
  trace_repr_source: &TracesRepresentationSource,
) -> Result<(MyDataset, Vec<String>, Vec<String>), ClusteringError> {
  create_traces_dataset_levenshtein_internal(log, class_extractor, |trace| {
    create_trace_representation::<TLog>(trace, trace_repr_source)
  })
}

fn create_traces_dataset_levenshtein_internal<TLog: EventLog>(
  log: &TLog,
  class_extractor: Option<&String>,
  trace_repr_creator: impl Fn(&TLog::TTrace) -> Vec<Rc<RefCell<TLog::TEvent>>>,
) -> Result<(MyDataset, Vec<String>, Vec<String>), ClusteringError> {
  let regex_hasher = match class_extractor.as_ref() {
    Some(class_extractor) => Some(RegexEventHasher::new(class_extractor).ok().unwrap()),
    None => None,
  };

  let mut processed_traces = vec![];
  for trace in log.traces() {
    let trace = trace.borrow();
    processed_traces.push(trace_repr_creator(&trace));
  }

  let mut all_event_classes = HashMap::new();
  let mut max_length = 0;
  for trace in &processed_traces {
    max_length = max_length.max(trace.len() + 1);

    for event in trace {
      let event = event.borrow();
      let processed_event_name = match regex_hasher.as_ref() {
        Some(regex_hasher) => regex_hasher.transform(event.name()),
        None => event.name(),
      };

      if !all_event_classes.contains_key(processed_event_name) {
        all_event_classes.insert(processed_event_name.to_owned(), all_event_classes.len() + 1);
      }
    }
  }

  let mut raw_dataset = vec![];
  for trace in &processed_traces {
    for event in trace {
      let event = event.borrow();
      let processed_event_name = match regex_hasher.as_ref() {
        Some(regex_hasher) => regex_hasher.transform(event.name()),
        None => event.name(),
      };

      raw_dataset.push(*all_event_classes.get(processed_event_name).expect("Should be there") as f64);
    }

    for _ in trace.len()..max_length {
      raw_dataset.push(0f64);
    }
  }

  let shape = (processed_traces.len(), max_length);
  let array = match Array2::from_shape_vec(shape, raw_dataset) {
    Ok(score) => score,
    Err(_) => return Err(ClusteringError::FailedToCreateNdArray),
  };

  Ok((
    DatasetBase::from(array),
    (0..processed_traces.len()).into_iter().map(|x| format!("Trace_{}", x)).collect(),
    (0..max_length).into_iter().map(|x| format!("Symbol_{}", x)).collect(),
  ))
}
