use crate::event_log::core::event::event::Event;
use crate::event_log::core::event::event_hasher::RegexEventHasher;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::features::analysis::patterns::activity_instances::{
  create_vector_of_immediate_underlying_events, create_vector_of_underlying_events,
};
use crate::features::clustering::common::{create_colors_vector, scale_raw_dataset_min_max, transform_to_ficus_dataset, MyDataset};
use crate::features::clustering::error::ClusteringError;
use crate::features::clustering::traces::traces_params::{FeatureCountKind, TracesClusteringParams, TracesRepresentationSource};
use crate::utils::dataset::dataset::LabeledDataset;
use crate::utils::distance::distance::{DistanceWrapper, FicusDistance};
use crate::utils::silhouette::silhouette_score;
use getset::Getters;
use linfa::DatasetBase;
use linfa_nn::distance::Distance;
use linfa_nn::CommonNearestNeighbour;
use linfa_nn::CommonNearestNeighbour::{KdTree, LinearSearch};
use log::warn;
use ndarray::{Array1, Array2};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub fn do_clusterize_log_by_traces<TLog: EventLog>(
  params: &mut TracesClusteringParams<TLog>,
  clustering_func: impl Fn(&mut TracesClusteringParams<TLog>, CommonNearestNeighbour, &MyDataset) -> Result<Vec<usize>, ClusteringError>,
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
    FicusDistance::Cosine | FicusDistance::L1 | FicusDistance::L2 => KdTree,
  };

  let labels = clustering_func(params, nn_search_algorithm, &dataset)?;

  let ficus_dataset = transform_to_ficus_dataset(&dataset, objects, features);

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
    }
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

    let max_count = *events_counts.values().max().unwrap_or(&0) as f64;
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

pub fn calculate_distance(distance: FicusDistance, dataset: &MyDataset, first: usize, second: usize) -> f64 {
  let first_record = dataset.records.row(first);
  let second_record = dataset.records.row(second);

  let distance_wrapper = DistanceWrapper::new(distance);
  distance_wrapper.distance(first_record, second_record)
}

#[derive(Getters)]
pub(crate) struct BestSilhouetteLabels {
  #[getset(get = "pub")]
  labels: Option<Vec<usize>>,
  #[getset(get = "pub")]
  score: f64,
}

impl BestSilhouetteLabels {
  pub fn new() -> Self {
    Self {
      labels: None,
      score: f64::MIN,
    }
  }

  pub fn process(&mut self, labels: Vec<usize>, distance_func: &dyn Fn(usize, usize) -> f64) {
    let score = match silhouette_score(&labels, |first, second| distance_func(first, second)) {
      Ok(score) => score,
      Err(err) => {
        warn!(
          "Failed to calculate silhouette score, skipping those labels, reason: {}",
          err.to_string()
        );
        if self.labels.is_none() {
          self.labels = Some(labels);
        }

        return;
      }
    };

    if score > self.score {
      self.labels = Some(labels);
      self.score = score;
    }
  }
}
