use super::{node::Node, suffix_tree_patterns::SuffixTree, suffix_tree_slice::SuffixTreeSlice};
use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

#[derive(Copy, Clone)]
struct BuildState {
  pub(self) pos: usize,
  pub(self) node_index: Option<usize>,
}

impl<'a, TElement> SuffixTree<'a, TElement>
where
  TElement: Eq + PartialEq + Hash + Copy,
{
  pub fn new(slice: &'a dyn SuffixTreeSlice<TElement>) -> Self {
    Self {
      slice,
      nodes: Rc::new(RefCell::new(vec![Node::create_default()])),
    }
  }

  pub fn dump_nodes(&self) -> Vec<(usize, usize, Option<usize>, Option<usize>)> {
    let mut dump = vec![];
    for node in self.nodes.borrow().iter() {
      dump.push((node.left, node.right, node.parent, node.link));
    }

    dump
  }

  pub fn build_tree(&mut self) {
    let mut state = BuildState {
      pos: 0,
      node_index: Some(0),
    };

    for pos in 0..self.slice.len() {
      loop {
        let next_state = self.go(state, pos, pos + 1);
        if next_state.node_index.is_some() {
          state = next_state;
          break;
        }

        let mid = self.split(state).unwrap();
        let leaf_index = self.nodes.borrow().len();
        self.nodes.borrow_mut().push(Node {
          left: pos,
          right: self.slice.len(),
          link: None,
          parent: Some(mid),
          children: HashMap::new(),
        });

        self
          .nodes
          .borrow_mut()
          .get_mut(mid)
          .unwrap()
          .update_child(&self.slice.get(pos), leaf_index);

        state.node_index = Some(self.get_link(mid));
        state.pos = self.nodes.borrow().get(state.node_index.unwrap()).unwrap().edge_len();

        if mid == 0 {
          break;
        }
      }
    }
  }

  fn go(&mut self, mut current_state: BuildState, mut left: usize, right: usize) -> BuildState {
    let mut nodes = self.nodes.borrow_mut();
    while left < right {
      let current_node = nodes.get_mut(current_state.node_index.unwrap()).unwrap();
      if current_state.pos == current_node.edge_len() {
        current_state = BuildState {
          node_index: current_node.go(&self.slice.get(left)),
          pos: 0,
        };

        if current_state.node_index.is_none() {
          return current_state;
        }

        continue;
      }

      if !self.slice.equals(current_node.left + current_state.pos, left) {
        return BuildState { node_index: None, pos: 0 };
      }

      let current_interval_len = right - left;
      let diff = current_node.edge_len() - current_state.pos;

      if current_interval_len < diff {
        return BuildState {
          node_index: current_state.node_index,
          pos: current_state.pos + current_interval_len,
        };
      }

      left += diff;
      current_state.pos = current_node.edge_len();
    }

    current_state
  }

  fn split(&mut self, current_state: BuildState) -> Option<usize> {
    let current_index = current_state.node_index.unwrap();
    let current_node_left;
    let current_node_parent;
    let edge_len;

    {
      let nodes = self.nodes.borrow();
      let current_node = nodes.get(current_index).unwrap();
      current_node_left = current_node.left;
      current_node_parent = current_node.parent;
      edge_len = current_node.edge_len();
    }

    if current_state.pos == edge_len {
      return Some(current_index);
    }

    if current_state.pos == 0 {
      return current_node_parent;
    }

    let index = self.nodes.borrow().len();
    let new_node = Node {
      parent: current_node_parent,
      left: current_node_left,
      right: current_node_left + current_state.pos,
      children: HashMap::new(),
      link: None,
    };

    self.nodes.borrow_mut().push(new_node);

    self.nodes.borrow_mut()[current_node_parent.unwrap()].update_child(&self.slice.get(current_node_left), index);

    let element = self.slice.get(current_node_left + current_state.pos);
    self.nodes.borrow_mut()[index].update_child(&element, current_index);

    self.nodes.borrow_mut()[current_index].parent = Some(index);
    self.nodes.borrow_mut()[current_index].left += current_state.pos;

    Some(index)
  }

  fn get_link(&mut self, node_index: usize) -> usize {
    let node_parent;
    let node_right;
    let node_left;
    let node_link;

    {
      let nodes = self.nodes.borrow();
      let node = nodes.get(node_index).unwrap();
      node_parent = node.parent;
      node_right = node.right;
      node_left = node.left;
      node_link = node.link;
    }

    if node_link.is_some() {
      return node_link.unwrap();
    }

    if node_parent.is_none() {
      return 0usize;
    }

    let to = self.get_link(node_parent.unwrap());

    let state;
    {
      let nodes = self.nodes.borrow();
      state = BuildState {
        node_index: Some(to),
        pos: nodes[to].edge_len(),
      };
    }

    let left = node_left + (if node_parent.unwrap() == 0 { 1 } else { 0 });
    let next = self.go(state, left, node_right);
    let link = self.split(next);

    self.nodes.borrow_mut().get_mut(node_index).unwrap().link = link;

    link.unwrap()
  }
}
