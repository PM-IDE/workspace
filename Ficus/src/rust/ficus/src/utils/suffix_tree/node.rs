use std::collections::HashMap;
use std::hash::Hash;

pub struct Node<TElement>
where
  TElement: Eq + PartialEq + Hash + Copy,
{
  pub left: usize,
  pub right: usize,
  pub link: Option<usize>,
  pub parent: Option<usize>,
  pub children: HashMap<Option<TElement>, usize>,
}

impl<TElement> Node<TElement>
where
  TElement: Eq + PartialEq + Hash + Copy,
{
  pub fn create_default() -> Self {
    Self {
      left: 0,
      right: 0,
      link: None,
      parent: None,
      children: HashMap::new(),
    }
  }

  pub fn is_leaf(&self) -> bool {
    self.children.is_empty()
  }

  pub fn edge_len(&self) -> usize {
    self.right - self.left
  }

  pub fn update_child(&mut self, element: &Option<TElement>, new_child: usize) {
    if self.children.contains_key(element) {
      *self.children.get_mut(element).unwrap() = new_child;
    } else {
      self.children.insert(*element, new_child);
    }
  }

  pub fn go(&mut self, element: &Option<TElement>) -> Option<usize> {
    match self.children.get(element) {
      Some(next) => Some(*next),
      None => None,
    }
  }
}
