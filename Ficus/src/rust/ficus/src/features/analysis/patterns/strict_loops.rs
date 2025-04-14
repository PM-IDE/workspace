use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::event_log::xes::xes_trace::XesTraceImpl;
use crate::features::analysis::patterns::activity_instances::ActivityInTraceInfo;
use crate::features::analysis::patterns::pattern_info::UnderlyingPatternKind;
use crate::features::analysis::patterns::repeat_sets::{ActivityNode, SubArrayWithTraceIndex};
use crate::features::analysis::patterns::tandem_arrays::{try_extract_tandem_array, TandemArrayInfo};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

pub fn find_loops_strict(log: &XesEventLogImpl, hashed_log: &Vec<Vec<u64>>, max_array_length: usize) -> Vec<Vec<ActivityInTraceInfo>> {
  find_tandem_arrays_strict(&hashed_log, max_array_length)
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
        .collect()
    )
    .collect()
}


fn find_tandem_arrays_strict(hashed_log: &Vec<Vec<u64>>, max_array_length: usize) -> Vec<Vec<TandemArrayInfo>> {
  let mut result = vec![];
  for trace in hashed_log {
    let mut index = 0;
    let mut trace_arrays = vec![];
    'this_loop: loop {
      if index >= trace.len() {
        break;
      }

      for length in (1..max_array_length).rev() {
        if index + length >= trace.len() {
          continue;
        }

        if let Some(array) = try_extract_tandem_array(trace, index, length) {
          trace_arrays.push(array);
          index += *array.get_repeat_count() * array.get_sub_array_info().length;
          continue 'this_loop;
        }
      }

      index += 1;
    }

    result.push(trace_arrays);
  }

  result
}

fn create_strict_loop_activity_instance(
  array: &TandemArrayInfo,
  trace_index: usize,
  trace: &XesTraceImpl,
  hashed_trace: &Vec<u64>,
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

  ActivityInTraceInfo::new(
    Rc::new(RefCell::new(ActivityNode::new(
      Some(SubArrayWithTraceIndex::new(array.clone(), trace_index)),
      HashSet::from_iter(hashed_trace[array.start_index..array.start_index + array.length].iter().map(|x| *x)),
      vec![],
      0,
      Rc::new(Box::new(format!("Loop[{}]", name.join("::")))),
      UnderlyingPatternKind::StrictLoop,
    ))),
    array.start_index,
    array.length * repeat_count,
  )
}