use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

use super::interval::Interval;
use super::node::Node;

pub struct IntervalTree<TElement, TRangeCreator, TElementIterator, TData>
where
  TElement: PartialEq + Ord + Copy + Hash,
  TRangeCreator: Fn(&TElement, &TElement) -> TElementIterator,
  TElementIterator: Iterator<Item=TElement>,
  TData: PartialEq + Eq + Copy,
{
  nodes: Box<Vec<Node<TElement, TData>>>,
  boundaries: Box<Vec<TElement>>,
  range_creator: TRangeCreator,
}

enum ChildOrientation {
  Left,
  Right,
}

impl<TElement, TRangeCreator, TElementIterator, TData> IntervalTree<TElement, TRangeCreator, TElementIterator, TData>
where
  TElement: PartialEq + Ord + Copy + Hash,
  TRangeCreator: Fn(&TElement, &TElement) -> TElementIterator,
  TElementIterator: Iterator<Item=TElement>,
  TData: PartialEq + Eq + Copy,
{
  pub fn new(
    intervals: &Vec<Interval<TElement, TData>>,
    range_creator: TRangeCreator,
  ) -> IntervalTree<TElement, TRangeCreator, TElementIterator, TData> {
    let mut nodes: Box<Vec<Node<TElement, TData>>> = Box::new(vec![]);
    let mut boundaries = Box::new(vec![]);

    let mut queue: VecDeque<(Option<(usize, ChildOrientation)>, Vec<&Interval<TElement, TData>>)> = VecDeque::new();

    let mut current_intervals = vec![];
    for interval in intervals {
      current_intervals.push(interval);
    }

    queue.push_back((None, current_intervals));

    while !queue.is_empty() {
      let (parent_child, mut current_intervals) = queue.pop_front().unwrap();
      current_intervals.sort_by(|first, second| first.left.cmp(&second.left));

      let center = current_intervals[current_intervals.len() / 2].left;
      let mut left_intervals = vec![];
      let mut right_intervals = vec![];
      let mut node_intervals = vec![];

      for interval in current_intervals {
        boundaries.push(interval.left);
        boundaries.push(interval.right);

        if interval.right < center {
          left_intervals.push(interval);
        } else if interval.left > center {
          right_intervals.push(interval);
        } else {
          node_intervals.push(*interval);
        }
      }

      let node = Node::new(center, node_intervals);
      let node_index = nodes.len();

      if let Some((parent, orientation)) = parent_child {
        match orientation {
          ChildOrientation::Left => nodes[parent].left_child = Some(node_index),
          ChildOrientation::Right => nodes[parent].right_child = Some(node_index),
        }
      }

      nodes.push(node);
      if left_intervals.len() > 0 {
        queue.push_back((Some((node_index, ChildOrientation::Left)), left_intervals));
      }

      if right_intervals.len() > 0 {
        queue.push_back((Some((node_index, ChildOrientation::Right)), right_intervals));
      }
    }

    IntervalTree {
      nodes,
      boundaries,
      range_creator,
    }
  }

  pub fn search_overlaps_for_point(&self, point: TElement) -> Vec<Interval<TElement, TData>> {
    let mut result = HashSet::new();
    self.search_internal(0, point, &mut result);

    Self::to_ordered_vec(result)
  }

  pub fn search_envelopes(&mut self, left: TElement, right: TElement) -> Vec<Interval<TElement, TData>> {
    if left >= right {
      return vec![];
    }

    let mut result = HashSet::new();
    self.search_internal(0, left, &mut result);

    self.boundaries.sort();

    let left_boundary = self.boundaries.binary_search(&left).unwrap_or_else(|value| value);
    let right_boundary = self.boundaries.binary_search(&right).unwrap_or_else(|value| value);

    for element in &self.boundaries[left_boundary..right_boundary] {
      self.search_internal(0, *element, &mut result);
    }

    Self::to_ordered_vec(
      result
        .into_iter()
        .filter(|interval| interval.left >= left && interval.right <= right),
    )
  }

  fn to_ordered_vec<TIterator>(set: TIterator) -> Vec<Interval<TElement, TData>>
  where
    TIterator: IntoIterator<Item=Interval<TElement, TData>>,
  {
    let mut result: Vec<Interval<TElement, TData>> = set.into_iter().collect();
    result.sort_by(|first, second| first.left.cmp(&second.left));

    result
  }

  pub fn search_overlaps_for_interval(&self, left: TElement, right: TElement) -> Vec<Interval<TElement, TData>> {
    let mut result = HashSet::new();
    for element in (self.range_creator)(&left, &right) {
      self.search_internal(0, element, &mut result);
    }

    Self::to_ordered_vec(result)
  }

  fn search_internal(&self, node_index: usize, point: TElement, result: &mut HashSet<Interval<TElement, TData>>) {
    let node = &self.nodes[node_index];
    for interval in &node.intervals {
      if interval.contains(point) {
        result.insert(*interval);
      }

      if let Some(left_child) = node.left_child {
        if point < node.center {
          self.search_internal(left_child, point, result);
        }
      }

      if let Some(right_child) = node.right_child {
        if point > node.center {
          self.search_internal(right_child, point, result);
        }
      }
    }
  }

  pub fn dump_nodes(&self) -> Vec<(Option<usize>, Option<usize>, TElement, Vec<Interval<TElement, TData>>)> {
    let mut nodes = vec![];
    for node in self.nodes.as_ref() {
      nodes.push((node.left_child, node.right_child, node.center, node.intervals.to_vec()));
    }

    nodes
  }
}
