use std::{
  cell::RefCell,
  collections::{HashMap, HashSet},
  rc::Rc,
  vec,
};

use linfa::DatasetBase;
use ndarray::Array2;

use crate::{
  event_log::core::{
    event::{event::Event, event_hasher::RegexEventHasher},
    event_log::EventLog,
    trace::trace::Trace,
  },
  features::{
    analysis::patterns::{
      activity_instances::{create_vector_of_underlying_events, ActivityInTraceInfo},
      repeat_sets::ActivityNode,
    },
    clustering::{
      common::{scale_raw_dataset_min_max, MyDataset},
      error::ClusteringError,
    },
  },
  pipelines::aliases::TracesActivities,
};

use super::activities_params::{ActivitiesVisualizationParams, ActivityRepresentationSource};

pub(super) type ActivityNodeWithCoords = Vec<(Rc<RefCell<ActivityNode>>, HashMap<String, usize>)>;

pub fn create_dataset<TLog: EventLog>(
  params: &ActivitiesVisualizationParams<TLog>,
) -> Result<(MyDataset, ActivityNodeWithCoords, Vec<String>), ClusteringError> {
  match params.activities_repr_source {
    ActivityRepresentationSource::EventClasses => create_dataset_from_activities_classes(params),
    ActivityRepresentationSource::SubTraces => create_dataset_from_activities_traces(params),
    ActivityRepresentationSource::SubTracesUnderlyingEvents => create_dataset_from_activities_traces_underlying_events(params),
  }
}

pub(super) fn create_dataset_from_activities_traces_underlying_events<TLog: EventLog>(
  params: &ActivitiesVisualizationParams<TLog>,
) -> Result<(MyDataset, ActivityNodeWithCoords, Vec<String>), ClusteringError> {
  create_dataset_internal(
    params.traces_activities,
    params.common_vis_params.class_extractor.clone(),
    |traces_activities, regex_hasher, all_event_classes| {
      Ok(create_activities_repr_from_subtraces(
        traces_activities,
        regex_hasher,
        all_event_classes,
        params,
        |events, map, all_event_classes| {
          let mut sub_trace_events = vec![];
          for event in events {
            for underlying_event in create_vector_of_underlying_events::<TLog>(event) {
              sub_trace_events.push(underlying_event);
            }
          }

          update_event_classes::<TLog>(sub_trace_events.as_slice(), regex_hasher, all_event_classes, map)
        },
      ))
    },
  )
}

pub(super) fn create_dataset_from_activities_traces<TLog: EventLog>(
  params: &ActivitiesVisualizationParams<TLog>,
) -> Result<(MyDataset, ActivityNodeWithCoords, Vec<String>), ClusteringError> {
  create_dataset_internal(
    params.traces_activities,
    params.common_vis_params.class_extractor.clone(),
    |traces_activities, regex_hasher, all_event_classes| {
      Ok(create_activities_repr_from_subtraces(
        traces_activities,
        regex_hasher,
        all_event_classes,
        params,
        |events, map, all_event_classes| update_event_classes::<TLog>(events, regex_hasher, all_event_classes, map),
      ))
    },
  )
}

fn update_event_classes<TLog: EventLog>(
  events: &[Rc<RefCell<<TLog as EventLog>::TEvent>>],
  regex_hasher: Option<&RegexEventHasher>,
  all_event_classes: &mut HashSet<String>,
  map: &mut HashMap<String, usize>,
) {
  for event in events {
    let processed_class_name = if let Some(hasher) = regex_hasher {
      hasher.transform(event.borrow().name()).to_owned()
    } else {
      event.borrow().name().to_owned()
    };

    all_event_classes.insert(processed_class_name.clone());
    *map.entry(processed_class_name.clone()).or_default() += 1;
  }
}

fn create_activities_repr_from_subtraces<TLog: EventLog>(
  traces_activities: &TracesActivities,
  regex_hasher: Option<&RegexEventHasher>,
  all_event_classes: &mut HashSet<String>,
  params: &ActivitiesVisualizationParams<TLog>,
  event_classes_updater: impl Fn(&[Rc<RefCell<TLog::TEvent>>], &mut HashMap<String, usize>, &mut HashSet<String>) -> (),
) -> HashMap<String, (Rc<RefCell<ActivityNode>>, HashMap<String, usize>)> {
  let mut processed = HashMap::new();
  for trace_activities in traces_activities.iter() {
    for activity in trace_activities {
      if processed.contains_key(activity.node().borrow().name()) {
        continue;
      }

      if *activity.node().borrow().level() != params.activity_level {
        continue;
      }

      let node = activity.node().borrow();
      if !processed.contains_key(node.name()) {
        processed.insert(node.name().to_owned(), (activity.node().clone(), HashMap::new()));
      }

      let map: &mut HashMap<String, usize> = &mut processed.get_mut(node.name()).unwrap().1;
      if let Some(repeat_set) = node.repeat_set().as_ref() {
        let array = repeat_set.sub_array;
        let trace = params.common_vis_params.log.traces().get(repeat_set.trace_index).unwrap();
        let events = trace.borrow();
        let events = events.events();

        let start = array.start_index;
        let end = start + array.length;
        event_classes_updater(&events[start..end], map, all_event_classes);
      }
    }
  }

  processed
    .into_iter()
    .map(|x| {
      (
        x.0.as_ref().as_ref().to_owned(),
        (x.1.0, x.1.1.into_iter().map(|x| (x.0, x.1)).collect()),
      )
    })
    .collect()
}

fn create_dataset_internal(
  traces_activities: &TracesActivities,
  class_extractor: Option<String>,
  activities_repr_fullfiller: impl Fn(
    &Vec<Vec<ActivityInTraceInfo>>,
    Option<&RegexEventHasher>,
    &mut HashSet<String>,
  ) -> Result<HashMap<String, (Rc<RefCell<ActivityNode>>, HashMap<String, usize>)>, ClusteringError>,
) -> Result<(MyDataset, ActivityNodeWithCoords, Vec<String>), ClusteringError> {
  let mut all_event_classes = HashSet::new();
  let regex_hasher = match class_extractor.as_ref() {
    Some(class_extractor) => Some(RegexEventHasher::new(class_extractor).ok().unwrap()),
    None => None,
  };

  let processed = activities_repr_fullfiller(traces_activities, regex_hasher.as_ref(), &mut all_event_classes)?;

  let mut all_event_classes = all_event_classes.into_iter().collect::<Vec<String>>();
  all_event_classes.sort();

  let mut processed = processed.iter().map(|x| x.1.clone()).collect::<ActivityNodeWithCoords>();
  processed.sort_by(|first, second| first.0.borrow().name().cmp(&second.0.borrow().name()));

  let mut vector = vec![];
  for activity in &processed {
    for i in 0..all_event_classes.len() {
      let count = if let Some(count) = activity.1.get(&all_event_classes[i]) {
        *count
      } else {
        0
      };

      vector.push(count as f64);
    }
  }

  scale_raw_dataset_min_max(&mut vector, processed.len(), all_event_classes.len());

  let shape = (processed.len(), all_event_classes.len());

  let array = match Array2::from_shape_vec(shape, vector) {
    Ok(score) => score,
    Err(_) => return Err(ClusteringError::FailedToCreateNdArray),
  };

  Ok((
    DatasetBase::from(array),
    processed,
    all_event_classes.iter().map(|x| x.to_string()).collect(),
  ))
}

pub(super) fn create_dataset_from_activities_classes<TLog: EventLog>(
  params: &ActivitiesVisualizationParams<TLog>,
) -> Result<(MyDataset, ActivityNodeWithCoords, Vec<String>), ClusteringError> {
  create_dataset_internal(
    params.traces_activities,
    params.common_vis_params.class_extractor.clone(),
    |traces_activities, regex_hasher, all_event_classes| {
      let mut processed = HashMap::new();
      for trace_activities in traces_activities.iter() {
        for activity in trace_activities {
          if processed.contains_key(activity.node().borrow().name().as_ref().as_ref()) {
            continue;
          }

          if *activity.node().borrow().level() != params.activity_level {
            continue;
          }

          let activity_event_classes = if let Some(repeat_set) = activity.node().borrow().repeat_set().as_ref() {
            let trace = params.common_vis_params.log.traces().get(repeat_set.trace_index).unwrap();
            let trace = trace.borrow();
            let events = trace.events();
            let array = &repeat_set.sub_array;

            let mut abstracted_event_classes = HashSet::new();
            for event in &events[array.start_index..(array.start_index + array.length)] {
              let class_name = if let Some(regex_hasher) = regex_hasher.as_ref() {
                regex_hasher.transform(event.borrow().name()).to_owned()
              } else {
                event.borrow().name().to_owned()
              };

              abstracted_event_classes.insert(class_name);
            }

            let abstracted_event_classes = abstracted_event_classes.into_iter().collect::<Vec<String>>();
            for class in &abstracted_event_classes {
              all_event_classes.insert(class.clone());
            }

            abstracted_event_classes
          } else {
            return Err(ClusteringError::NoRepeatSet);
          };

          processed.insert(
            activity.node().borrow().name().as_ref().as_ref().to_owned(),
            (activity.node().clone(), activity_event_classes.into_iter().map(|x| (x, 1)).collect()),
          );
        }
      }

      Ok(processed)
    },
  )
}
