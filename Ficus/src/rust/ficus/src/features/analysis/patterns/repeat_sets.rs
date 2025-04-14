use crate::features::analysis::patterns::pattern_info::UnderlyingPatternKind;
use crate::utils::hash_utils::calculate_poly_hash_for_collection;
use getset::Getters;
use std::sync::atomic::{AtomicU64, Ordering};
use std::{
  cell::RefCell,
  collections::{HashMap, HashSet},
  rc::Rc,
};

use super::tandem_arrays::SubArrayInTraceInfo;

#[derive(Clone, Copy, Debug)]
pub struct SubArrayWithTraceIndex {
  pub sub_array: SubArrayInTraceInfo,
  pub trace_index: usize,
}

impl SubArrayWithTraceIndex {
  pub fn new(sub_array: SubArrayInTraceInfo, trace_index: usize) -> Self {
    Self { sub_array, trace_index }
  }

  pub fn dump(&self) -> (usize, usize, usize) {
    (self.sub_array.start_index, self.sub_array.length, self.trace_index)
  }
}

pub fn build_repeat_sets(log: &Vec<Vec<u64>>, patterns: &Vec<Vec<SubArrayInTraceInfo>>) -> Vec<SubArrayWithTraceIndex> {
  let mut repeat_sets = HashMap::new();
  let mut set = HashSet::new();
  let mut vec: Vec<u64> = vec![];
  let mut trace_index = 0;

  for (trace, trace_patterns) in log.into_iter().zip(patterns.iter()) {
    for pattern in trace_patterns {
      let start = pattern.start_index;
      let end = start + pattern.length;

      set.clear();
      for element in &trace[start..end] {
        set.insert(*element);
      }

      vec.clear();
      vec.extend(&set);
      vec.sort();

      let hash = calculate_poly_hash_for_collection(vec.as_slice());

      if !repeat_sets.contains_key(&hash) {
        repeat_sets.insert(hash, SubArrayWithTraceIndex::new(*pattern, trace_index));
      }
    }

    trace_index += 1;
  }

  let mut result = vec![];
  for repeat_set in repeat_sets.values().into_iter() {
    result.push(*repeat_set);
  }

  result.sort_by(|first, second| {
    if first.trace_index == second.trace_index {
      if first.sub_array.start_index != second.sub_array.start_index {
        first.sub_array.start_index.cmp(&second.sub_array.start_index)
      } else {
        first.sub_array.length.cmp(&second.sub_array.length)
      }
    } else {
      first.trace_index.cmp(&second.trace_index)
    }
  });

  result
}

#[derive(Debug, Getters)]
pub struct ActivityNode {
  #[getset(get = "pub")] id: Rc<Box<String>>,
  #[getset(get = "pub")] repeat_set: Option<SubArrayWithTraceIndex>,
  #[getset(get = "pub")] event_classes: HashSet<u64>,
  #[getset(get = "pub")] children: Vec<Rc<RefCell<ActivityNode>>>,
  #[getset(get = "pub")] level: usize,
  #[getset(get = "pub")] name: Rc<Box<String>>,
  #[getset(get = "pub")] pattern_kind: UnderlyingPatternKind,
}

impl ActivityNode {
  pub fn new(
    repeat_set: Option<SubArrayWithTraceIndex>,
    event_classes: HashSet<u64>,
    children: Vec<Rc<RefCell<ActivityNode>>>,
    level: usize,
    name: Rc<Box<String>>,
    pattern_kind: UnderlyingPatternKind,
  ) -> Self {
    static NEXT_ID: AtomicU64 = AtomicU64::new(0);

    Self {
      id: Rc::new(Box::new(format!("Activity_{}", NEXT_ID.fetch_add(1, Ordering::SeqCst)))),
      repeat_set,
      event_classes,
      children,
      level,
      name,
      pattern_kind,
    }
  }

  pub fn len(&self) -> usize {
    self.event_classes.len()
  }

  fn contains_other(&self, other_node: &ActivityNode) -> bool {
    self.event_classes.is_superset(&other_node.event_classes)
  }
}

pub fn build_repeat_set_tree_from_repeats<TNameCreator>(
  log: &Vec<Vec<u64>>,
  repeats: &Vec<SubArrayWithTraceIndex>,
  activity_level: usize,
  pattern_kind: UnderlyingPatternKind,
  name_creator: TNameCreator,
) -> Vec<Rc<RefCell<ActivityNode>>>
where
  TNameCreator: Fn(&SubArrayWithTraceIndex) -> String,
{
  if repeats.len() == 0 {
    return vec![];
  }

  let extract_events_set = |repeat_set: &SubArrayWithTraceIndex| -> HashSet<u64> {
    let trace = log.get(repeat_set.trace_index).unwrap();
    let mut set = HashSet::new();
    let array = repeat_set.sub_array;
    for index in array.start_index..(array.start_index + array.length) {
      set.insert(trace[index]);
    }

    set
  };

  let create_activity_node = |repeat_set: &SubArrayWithTraceIndex| {
    let events_set = extract_events_set(repeat_set);
    Rc::new(RefCell::new(ActivityNode::new(
      Some(*repeat_set),
      events_set,
      vec![],
      activity_level,
      Rc::new(Box::new(name_creator(repeat_set))),
      pattern_kind,
    )))
  };

  let mut activity_nodes = repeats
    .iter()
    .map(|repeat| create_activity_node(&repeat))
    .collect::<Vec<Rc<RefCell<ActivityNode>>>>();

  activity_nodes.sort_by(|first, second| second.borrow().len().cmp(&first.borrow().len()));
  let max_length = activity_nodes[0].borrow().len();
  let mut top_level_nodes = vec![Rc::clone(&activity_nodes[0])];
  let mut next_length_index = 1;
  let mut current_length = max_length;

  for i in 1..activity_nodes.len() {
    let node_ptr = &activity_nodes[i];
    if node_ptr.borrow().len() != max_length {
      next_length_index = i;
      current_length = node_ptr.borrow().len();
      break;
    }

    top_level_nodes.push(Rc::clone(node_ptr));
  }

  if top_level_nodes.len() == activity_nodes.len() {
    return top_level_nodes;
  }

  let mut nodes_by_level: Vec<Vec<Rc<RefCell<ActivityNode>>>> = vec![vec![]];

  for i in next_length_index..activity_nodes.len() {
    let current_node_ptr = activity_nodes.get(i).unwrap();
    let current_node = current_node_ptr.borrow();

    if current_node.len() < current_length {
      current_length = current_node.len();
      nodes_by_level.push(vec![]);
    }

    let mut found_any_match = false;

    for level_index in (0..(nodes_by_level.len() - 1)).rev() {
      for activity_node in nodes_by_level.get(level_index).unwrap() {
        let mut activity_node = activity_node.borrow_mut();
        if activity_node.contains_other(&current_node) {
          activity_node.children.push(Rc::clone(current_node_ptr));
          found_any_match = true;
        }
      }
    }

    if !found_any_match {
      for top_level_node_ptr in top_level_nodes.iter() {
        let mut top_level_node = top_level_node_ptr.borrow_mut();
        if top_level_node.contains_other(&current_node) && !Rc::ptr_eq(top_level_node_ptr, current_node_ptr) {
          top_level_node.children.push(Rc::clone(current_node_ptr));
          found_any_match = true;
        }
      }
    }

    nodes_by_level.last_mut().unwrap().push(Rc::clone(current_node_ptr));
    if !found_any_match {
      top_level_nodes.push(Rc::clone(current_node_ptr));
    }
  }

  top_level_nodes
}
