use crate::utils::hash_utils::compare_based_on_hashes;
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct TwoSets<T>
where
  T: Hash + Eq + ToString + Ord + Clone,
{
  first_set: BTreeSet<T>,
  second_set: BTreeSet<T>,
}

impl<T> TwoSets<T>
where
  T: Hash + Eq + ToString + Ord + Clone,
{
  pub fn empty() -> Self {
    Self {
      first_set: BTreeSet::new(),
      second_set: BTreeSet::new(),
    }
  }

  pub fn new(first_set: BTreeSet<T>, second_set: BTreeSet<T>) -> Self {
    Self { first_set, second_set }
  }

  pub fn new_one_element(first: T, second: T) -> Self {
    Self {
      first_set: BTreeSet::from_iter(vec![first]),
      second_set: BTreeSet::from_iter(vec![second]),
    }
  }

  pub fn is_full_subset(&self, other: &Self) -> bool {
    self.first_set.is_subset(&other.first_set) && self.second_set.is_subset(&other.second_set)
  }

  pub fn merge(&self, other: &TwoSets<T>) -> Self {
    Self {
      first_set: self.first_set.iter().chain(other.first_set.iter()).map(|c| c.clone()).collect(),
      second_set: self.second_set.iter().chain(other.second_set.iter()).map(|c| c.clone()).collect(),
    }
  }

  pub fn first_set(&self) -> &BTreeSet<T> {
    &self.first_set
  }

  pub fn first_set_mut(&mut self) -> &mut BTreeSet<T> {
    &mut self.first_set
  }

  pub fn second_set(&self) -> &BTreeSet<T> {
    &self.second_set
  }

  pub fn second_set_mut(&mut self) -> &mut BTreeSet<T> {
    &mut self.second_set
  }

  pub fn is_first_subset(&self, other: &Self) -> bool {
    self.first_set.is_subset(&other.first_set)
  }

  pub fn is_second_subset(&self, other: &Self) -> bool {
    self.second_set.is_subset(&other.second_set)
  }
}

impl<T> Hash for TwoSets<T>
where
  T: Hash + Eq + ToString + Ord + Clone,
{
  fn hash<H: Hasher>(&self, state: &mut H) {
    for item in &self.first_set {
      item.hash(state);
    }

    for item in &self.second_set {
      item.hash(state);
    }
  }
}

impl<T> PartialEq for TwoSets<T>
where
  T: Hash + Eq + ToString + Ord + Clone,
{
  fn eq(&self, other: &Self) -> bool {
    compare_based_on_hashes(self, other)
  }
}

impl<T> Eq for TwoSets<T> where T: Hash + Eq + ToString + Ord + Clone {}

impl<T> Clone for TwoSets<T>
where
  T: Hash + Eq + ToString + Ord + Clone,
{
  fn clone(&self) -> Self {
    Self {
      first_set: self.first_set.iter().map(|c| c.clone()).collect(),
      second_set: self.second_set.iter().map(|c| c.clone()).collect(),
    }
  }
}

impl<T> ToString for TwoSets<T>
where
  T: Hash + Eq + ToString + Ord + Clone,
{
  fn to_string(&self) -> String {
    let mut repr = String::new();
    repr.push('(');

    let mut write_set = |set: &BTreeSet<T>, repr: &mut String| {
      repr.push('{');
      for item in set {
        repr.push_str(item.to_string().as_str());
        repr.push(',');
      }

      if set.len() > 0 {
        repr.remove(repr.len() - 1);
      }

      repr.push('}');
    };

    write_set(&self.first_set, &mut repr);

    repr.push_str(", ");

    write_set(&self.second_set, &mut repr);

    repr.push(')');
    repr
  }
}
