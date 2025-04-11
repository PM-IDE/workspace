use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::event_log::xes::xes_trace::XesTraceImpl;
use crate::features::analysis::patterns::activity_instances::ActivityInTraceInfo;
use crate::features::analysis::patterns::pattern_info::UnderlyingPatternKind;
use crate::features::analysis::patterns::repeat_sets::{ActivityNode, SubArrayWithTraceIndex};
use crate::features::analysis::patterns::tandem_arrays::{find_maximal_tandem_arrays_with_length, TandemArrayInfo};
use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

pub fn find_loops_strict(log: &XesEventLogImpl, hashed_log: &Vec<Vec<u64>>, max_array_length: usize) -> Vec<Vec<ActivityInTraceInfo>> {
  let instances = find_max_loops(log, hashed_log, max_array_length);
  remove_overlapping_loops(instances)
}

fn find_max_loops(log: &XesEventLogImpl, hashed_log: &Vec<Vec<u64>>, max_array_length: usize) -> Vec<Vec<ActivityInTraceInfo>> {
  find_maximal_tandem_arrays_with_length(&hashed_log, max_array_length, true)
    .into_iter()
    .enumerate()
    .map(|(trace_index, trace_arrays)|
      trace_arrays
        .into_iter()
        .map(|array| {
          let trace = log.traces().get(trace_index).unwrap();
          let hashed_trace = hashed_log.get(trace_index).unwrap();

          create_strict_loop_activity_instance(&array, trace_index, &trace.borrow(), hashed_trace)
        })
        .into_group_map_by(|activity| activity.start_pos)
        .into_iter()
        .map(|(_, activities_by_start_pos)| {
          activities_by_start_pos.into_iter().max_by(|f, s| f.length.cmp(&s.length)).unwrap()
        })
        .collect()
    )
    .collect()
}

fn remove_overlapping_loops(mut instances: Vec<Vec<ActivityInTraceInfo>>) -> Vec<Vec<ActivityInTraceInfo>> {
  instances.iter_mut().for_each(|trace| trace.sort_by(|first, second| first.start_pos.cmp(&second.start_pos)));

  let mut filtered_instances = vec![];
  for trace_instances in instances {
    let mut filtered_trace_instances = vec![];
    let mut covered_range = None;

    for activity in trace_instances {
      match covered_range {
        Some(to_index) => if activity.start_pos >= to_index {
          covered_range = Some(activity.start_pos + activity.length);
          filtered_trace_instances.push(activity);
        },
        None => {
          covered_range = Some(activity.start_pos + activity.length);
          filtered_trace_instances.push(activity);
        }
      }
    }

    filtered_instances.push(filtered_trace_instances);
  }
  
  filtered_instances
}

fn create_strict_loop_activity_instance(
  array: &TandemArrayInfo,
  trace_index: usize,
  trace: &XesTraceImpl,
  hashed_trace: &Vec<u64>
) -> ActivityInTraceInfo {
  let repeat_count = *array.get_repeat_count();
  let array = array.get_sub_array_info();

  let mut name = trace.events()[array.start_index..array.start_index + array.length]
    .iter()
    .map(|e| e.borrow().name().clone())
    .collect::<HashSet<String>>()
    .into_iter()
    .collect::<Vec<String>>();

  name.sort();

  ActivityInTraceInfo {
    start_pos: array.start_index,
    length: array.length * repeat_count,
    node: Rc::new(RefCell::new(ActivityNode::new(
      Some(SubArrayWithTraceIndex::new(array.clone(), trace_index)),
      HashSet::from_iter(hashed_trace[array.start_index..array.start_index + array.length].iter().map(|x| *x)),
      vec![],
      0,
      Rc::new(Box::new(format!("Loop[{}]", name.join("::")))),
      UnderlyingPatternKind::StrictLoop,
    ))),
  }
}