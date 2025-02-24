use std::cell::RefCell;
use std::hash::Hash;
use std::rc::Rc;

use super::node::Node;
use super::suffix_tree_slice::SuffixTreeSlice;

pub struct SuffixTree<'a, TElement>
where
  TElement: Eq + Hash + Copy,
{
  pub(super) slice: &'a dyn SuffixTreeSlice<TElement>,
  pub(super) nodes: Rc<RefCell<Vec<Node<TElement>>>>,
}

impl<'a, TElement> SuffixTree<'a, TElement>
where
  TElement: Eq + Hash + Copy,
{
  pub fn find_patterns(&self, pattern: &[TElement]) -> Option<Vec<(usize, usize)>> {
    let mut current_node_index = 0;
    let mut pattern_index = 0;
    let mut suffix_length = 0;

    let nodes = self.nodes.borrow();

    loop {
      if pattern_index == pattern.len() {
        break;
      }

      let current_node = nodes.get(current_node_index).unwrap();
      if !current_node.children.contains_key(&Some(pattern[pattern_index])) {
        return None;
      }

      let child_index = current_node.children.get(&Some(pattern[pattern_index])).unwrap();
      let child_node = nodes.get(*child_index).unwrap();

      for i in child_node.left..child_node.right {
        if pattern_index == pattern.len() {
          break;
        }

        let slice_element = self.slice.get(i);
        if slice_element.is_none() || slice_element.unwrap() != pattern[pattern_index] {
          return None;
        }

        pattern_index += 1;
      }

      current_node_index = *child_index;
      suffix_length += child_node.edge_len();
    }

    let mut patterns = Vec::new();

    suffix_length -= nodes.get(current_node_index).unwrap().edge_len();
    self.dfs_pattern_search(current_node_index, &mut patterns, pattern.len(), suffix_length);

    patterns.sort();

    Some(patterns)
  }

  fn dfs_pattern_search(&self, index: usize, patterns: &mut Vec<(usize, usize)>, pattern_length: usize, mut suffix_length: usize) {
    let nodes = self.nodes.borrow();
    let node = nodes.get(index).unwrap();
    suffix_length += node.edge_len();

    if node.is_leaf() {
      let left = self.slice.len() - suffix_length;
      patterns.push((left, left + pattern_length));

      return;
    }

    for (_, child_node_index) in &node.children {
      self.dfs_pattern_search(*child_node_index, patterns, pattern_length, suffix_length);
    }
  }

  pub(super) fn get_element_for_suffix(&self, suffix_length: usize) -> Option<TElement> {
    if suffix_length + 1 > self.slice.len() {
      None
    } else {
      self.slice.get(self.slice.len() - suffix_length - 1)
    }
  }
}
