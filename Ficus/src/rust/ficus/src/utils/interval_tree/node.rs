use std::hash::Hash;

use super::interval::Interval;

pub struct Node<TElement, TData>
where
  TElement: PartialEq + Ord + Copy + Hash,
  TData: PartialEq + Eq + Copy,
{
  pub left_child: Option<usize>,
  pub right_child: Option<usize>,
  pub center: TElement,
  pub intervals: Vec<Interval<TElement, TData>>,
}

impl<TElement, TData> Node<TElement, TData>
where
  TElement: PartialEq + Ord + Copy + Hash,
  TData: PartialEq + Eq + Copy,
{
  pub fn new(center: TElement, intervals: Vec<Interval<TElement, TData>>) -> Node<TElement, TData> {
    Node {
      left_child: None,
      right_child: None,
      center,
      intervals,
    }
  }
}
