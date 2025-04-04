use std::{
  cell::RefCell,
  collections::{HashMap, HashSet},
  rc::Rc,
};

use ndarray::Array1;

use crate::{
  event_log::core::{event::event::Event, event_log::EventLog, trace::trace::Trace},
  features::{
    analysis::patterns::{activity_instances::ActivityInTraceInfo, repeat_sets::ActivityNode},
    clustering::common::create_cluster_name,
  },
  pipelines::aliases::TracesActivities,
};

pub(super) fn merge_activities(
  log: &impl EventLog,
  traces_activities: &mut TracesActivities,
  processed: &Vec<Rc<RefCell<ActivityNode>>>,
  labels: &Array1<Option<usize>>,
) {
  let mut activity_names_to_clusters = HashMap::new();
  let mut clusters_to_activities: HashMap<usize, Vec<Rc<RefCell<ActivityNode>>>> = HashMap::new();

  for (activity, label) in processed.iter().zip(labels.iter()) {
    if let Some(label) = label {
      activity_names_to_clusters.insert(activity.borrow().name().to_owned(), *label);

      if let Some(cluster_activities) = clusters_to_activities.get_mut(label) {
        cluster_activities.push(activity.clone());
      } else {
        clusters_to_activities.insert(*label, vec![activity.clone()]);
      }
    }
  }

  let mut new_activity_name_parts = HashSet::new();
  let mut new_cluster_activities = HashMap::new();

  for (cluster, cluster_activities) in &clusters_to_activities {
    if cluster_activities.len() < 2 {
      continue;
    }

    let mut new_event_classes_set = HashSet::new();

    for activity in cluster_activities {
      for event_class in activity.borrow().event_classes() {
        new_event_classes_set.insert(*event_class);
      }

      if let Some(repeat_set) = activity.borrow().repeat_set().as_ref() {
        let trace = log.traces().get(repeat_set.trace_index).unwrap();
        let events = trace.borrow();
        let events = events.events();
        let sub_array = repeat_set.sub_array;
        for event in &events[sub_array.start_index..(sub_array.start_index + sub_array.length)] {
          new_activity_name_parts.insert(event.borrow().name().to_owned());
        }
      }
    }

    let mut new_activity_name_parts = new_activity_name_parts.iter().map(|x| x.to_owned()).collect::<Vec<String>>();
    new_activity_name_parts.sort_by(|first, second| first.cmp(second));

    let mut new_activity_name = String::new();
    new_activity_name.push_str(create_cluster_name(*cluster).as_str());

    let new_node = ActivityNode::new(
      None,
      new_event_classes_set,
      vec![],
      cluster_activities[0].borrow().level(),
      Rc::new(Box::new(new_activity_name)),
      cluster_activities.first().unwrap().borrow().underlying_pattern_kind().clone()
    );

    new_cluster_activities.insert(*cluster, Rc::new(RefCell::new(new_node)));
  }

  for trace_activities in traces_activities.iter_mut() {
    for i in 0..trace_activities.len() {
      let activity = trace_activities.get(i).unwrap();
      if !activity_names_to_clusters.contains_key(activity.node.borrow().name()) {
        continue;
      }

      let cluster_label = activity_names_to_clusters.get(activity.node.borrow().name()).unwrap();
      if let Some(new_activity_node) = new_cluster_activities.get(cluster_label) {
        let current_activity_in_trace = trace_activities.get(i).unwrap();

        *trace_activities.get_mut(i).unwrap() = ActivityInTraceInfo {
          node: new_activity_node.clone(),
          start_pos: current_activity_in_trace.start_pos,
          length: current_activity_in_trace.length,
        };
      }
    }
  }

  for trace_activities in traces_activities.iter_mut() {
    if trace_activities.len() < 2 {
      continue;
    }

    let mut index = 1;
    let mut last_seen_activity = trace_activities.first().unwrap().node.borrow().name().to_owned();

    loop {
      if index >= trace_activities.len() {
        break;
      }

      let activity_name = trace_activities.get(index).unwrap().node.borrow().name().to_owned();
      if last_seen_activity == activity_name {
        let start_index = trace_activities.get(index).unwrap().start_pos;
        let length = trace_activities.get(index).unwrap().length;
        let prev_start_pos = trace_activities.get(index - 1).unwrap().start_pos;
        let prev_length = trace_activities.get(index - 1).unwrap().length;

        if prev_start_pos + prev_length == start_index {
          trace_activities.remove(index);
          let prev_activity = trace_activities.get_mut(index - 1).unwrap();
          prev_activity.length += length;
        } else {
          index += 1;
        }
      } else {
        index += 1;
      }

      last_seen_activity = activity_name;
    }
  }
}
