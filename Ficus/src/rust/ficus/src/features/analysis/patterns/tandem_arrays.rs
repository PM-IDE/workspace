use crate::utils::hash_utils::calculate_poly_hash_for_collection;
use std::{cell::RefCell, collections::HashSet, rc::Rc};

#[derive(Debug, Clone, Copy)]
pub struct SubArrayInTraceInfo {
  pub start_index: usize,
  pub length: usize,
}

impl SubArrayInTraceInfo {
  pub fn new(start_index: usize, length: usize) -> Self {
    Self { start_index, length }
  }

  pub fn get_start_index(&self) -> &usize {
    &self.start_index
  }

  pub fn get_length(&self) -> &usize {
    &self.length
  }

  pub fn dump(&self) -> (usize, usize) {
    (self.start_index, self.length)
  }
}

#[derive(Debug, Clone, Copy)]
pub struct TandemArrayInfo {
  sub_array: SubArrayInTraceInfo,
  repeat_count: usize,
}

impl TandemArrayInfo {
  pub fn get_sub_array_info(&self) -> &SubArrayInTraceInfo {
    &self.sub_array
  }

  pub fn get_repeat_count(&self) -> &usize {
    &self.repeat_count
  }

  pub fn dump(&self) -> (usize, usize, usize) {
    (self.sub_array.start_index, self.sub_array.length, self.repeat_count)
  }
}

pub fn find_primitive_tandem_arrays(log: &Vec<Vec<u64>>, max_tandem_array_length: usize, include_length_one: bool) -> Vec<Vec<SubArrayInTraceInfo>> {
  find_primitive_tandem_arrays_with_length(log, max_tandem_array_length, include_length_one)
    .borrow()
    .iter()
    .map(|trace_arrays| trace_arrays.into_iter().map(|array| array.sub_array).collect())
    .collect()
}

pub fn find_primitive_tandem_arrays_with_length(
  log: &Vec<Vec<u64>>,
  max_tandem_array_length: usize,
  include_length_one: bool,
) -> Rc<RefCell<Vec<Vec<TandemArrayInfo>>>> {
  let maximal_arrays = find_maximal_tandem_arrays_with_length(log, max_tandem_array_length, include_length_one);
  let primitive_arrays_ptr = Rc::new(RefCell::new(vec![]));
  let primitive_arrays = &mut primitive_arrays_ptr.borrow_mut();

  for (trace_arrays, trace) in maximal_arrays.iter().zip(log) {
    let mut traces_primitive_arrays = Vec::new();
    for array in trace_arrays {
      let mut is_primitive = true;
      for length in 2..((array.sub_array.length + 1) / 2 + 1) {
        if try_extract_tandem_array(trace, array.sub_array.start_index, length).is_some() {
          is_primitive = false;
          break;
        }
      }

      if is_primitive {
        traces_primitive_arrays.push(*array);
      }
    }

    primitive_arrays.push(traces_primitive_arrays);
  }

  Rc::clone(&primitive_arrays_ptr)
}

pub fn find_maximal_tandem_arrays(log: &Vec<Vec<u64>>, max_tandem_array_length: usize, include_length_one: bool) -> Vec<Vec<SubArrayInTraceInfo>> {
  find_maximal_tandem_arrays_with_length(log, max_tandem_array_length, include_length_one)
    .iter()
    .map(|trace_arrays| trace_arrays.into_iter().map(|array| array.sub_array).collect())
    .collect()
}

pub fn find_maximal_tandem_arrays_with_length(
  log: &Vec<Vec<u64>>,
  max_tandem_array_length: usize,
  include_length_one: bool,
) -> Vec<Vec<TandemArrayInfo>> {
  let mut result = vec![];
  let mut visited = HashSet::new();

  for trace in log {
    visited.clear();
    let mut trace_tandem_arrays = Vec::new();

    let start_length = if include_length_one { 1 } else { 2 };

    for length in start_length..max_tandem_array_length.min(trace.len()) {
      for i in 0..(trace.len() - length) {
        let sub_array_hash = calculate_poly_hash_for_collection(&trace[i..(i + length)]);
        if visited.contains(&sub_array_hash) {
          continue;
        }

        visited.insert(sub_array_hash);
        if let Some(tandem_array) = try_extract_tandem_array(trace, i, length) {
          trace_tandem_arrays.push(tandem_array);
        }
      }
    }

    result.push(trace_tandem_arrays);
  }

  result
}

fn try_extract_tandem_array(trace: &Vec<u64>, start_index: usize, length: usize) -> Option<TandemArrayInfo> {
  let mut current_index = start_index + length;
  let mut repeat_count = 1;

  'this_loop: loop {
    if current_index + length - 1 >= trace.len() {
      break;
    }

    for i in 0..length {
      if trace[current_index + i] != trace[start_index + i] {
        break 'this_loop;
      }
    }

    repeat_count += 1;
    current_index += length;
  }

  if repeat_count > 1 {
    let sub_array_info = SubArrayInTraceInfo { start_index, length };
    return Some(TandemArrayInfo {
      sub_array: sub_array_info,
      repeat_count,
    });
  }

  None
}
