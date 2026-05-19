use crate::{
  features::discovery::alpha::providers::alpha_provider::AlphaRelationsProvider,
  utils::{hash_utils::compare_based_on_hashes, sets::two_sets::TwoSets},
};
use std::{
  fmt::Display,
  hash::{Hash, Hasher},
  sync::Arc,
};

#[derive(Debug, Default)]
pub struct AlphaSet {
  two_sets: TwoSets<Arc<str>>,
}

impl AlphaSet {
  pub fn new(left_class: Arc<str>, right_class: Arc<str>) -> Self {
    Self {
      two_sets: TwoSets::new_one_element(left_class, right_class),
    }
  }

  pub fn is_left_subset(&self, other: &Self) -> bool {
    self.two_sets.is_first_subset(&other.two_sets)
  }

  pub fn is_right_subset(&self, other: &Self) -> bool {
    self.two_sets.is_second_subset(&other.two_sets)
  }

  pub fn is_full_subset(&self, other: &Self) -> bool {
    self.is_left_subset(other) && self.is_right_subset(other)
  }

  pub fn contains_left(&self, class: &str) -> bool {
    self.two_sets.first_set().contains(class)
  }

  pub fn contains_right(&self, class: &str) -> bool {
    self.two_sets.second_set().contains(class)
  }

  pub fn left_classes(&self) -> Vec<&Arc<str>> {
    self.two_sets.first_set().iter().collect()
  }

  pub fn right_classes(&self) -> Vec<&Arc<str>> {
    self.two_sets.second_set().iter().collect()
  }

  pub fn insert_left_class(&mut self, class: Arc<str>) {
    self.two_sets.first_set_mut().insert(class);
  }

  pub fn insert_right_class(&mut self, class: Arc<str>) {
    self.two_sets.second_set_mut().insert(class);
  }

  pub fn can_extend(&self, other: &Self, provider: &impl AlphaRelationsProvider) -> bool {
    for left_class in self.two_sets.first_set().iter().chain(other.two_sets.first_set().iter()) {
      for right_class in self.two_sets.second_set().iter().chain(other.two_sets.second_set().iter()) {
        if !provider.causal_relation(left_class, right_class) {
          return false;
        }
      }
    }

    for first_left_class in self.two_sets.first_set().iter().chain(other.two_sets.first_set().iter()) {
      for second_left_class in self.two_sets.first_set().iter().chain(other.two_sets.first_set().iter()) {
        if !provider.unrelated_relation(first_left_class, second_left_class) {
          return false;
        }
      }
    }

    for first_right_class in self.two_sets.second_set().iter().chain(other.two_sets.second_set().iter()) {
      for second_right_class in self.two_sets.second_set().iter().chain(other.two_sets.second_set().iter()) {
        if !provider.unrelated_relation(first_right_class, second_right_class) {
          return false;
        }
      }
    }

    true
  }

  pub fn extend(&self, other: &Self) -> AlphaSet {
    Self {
      two_sets: self.two_sets.merge(&other.two_sets),
    }
  }
}

impl Hash for AlphaSet {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.two_sets.hash(state)
  }
}

impl PartialEq for AlphaSet {
  fn eq(&self, other: &Self) -> bool {
    compare_based_on_hashes(self, other)
  }
}

impl Eq for AlphaSet {}

impl Display for AlphaSet {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.two_sets.to_string().as_str())
  }
}

impl Clone for AlphaSet {
  fn clone(&self) -> Self {
    Self {
      two_sets: self.two_sets.clone(),
    }
  }
}
