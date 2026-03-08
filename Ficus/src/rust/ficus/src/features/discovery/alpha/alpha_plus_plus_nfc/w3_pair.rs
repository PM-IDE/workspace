use crate::{
  event_log::core::event_log::EventLog,
  features::discovery::alpha::providers::{alpha_plus_nfc_provider::AlphaPlusNfcRelationsProvider, alpha_provider::AlphaRelationsProvider},
  utils::{hash_utils::compare_based_on_hashes, sets::two_sets::TwoSets},
};
use std::{
  collections::HashSet,
  fmt::Display,
  hash::{Hash, Hasher},
  rc::Rc,
};

pub(crate) struct W3Pair {
  two_sets: TwoSets<Rc<str>>,
}

impl W3Pair {
  pub fn new(first: Rc<str>, second: Rc<str>) -> Self {
    Self {
      two_sets: TwoSets::new_one_element(first, second),
    }
  }

  pub fn try_new<TLog: EventLog>(
    first: Rc<str>,
    second: Rc<str>,
    w3_relations: &HashSet<(&str, &str)>,
    provider: &AlphaPlusNfcRelationsProvider<TLog>,
  ) -> Option<Self> {
    let new_pair = Self::new(first, second);
    match new_pair.valid(w3_relations, provider) {
      true => Some(new_pair),
      false => None,
    }
  }

  pub fn valid<TLog: EventLog>(&self, w3_relations: &HashSet<(&str, &str)>, provider: &AlphaPlusNfcRelationsProvider<TLog>) -> bool {
    for first in self.two_sets.first_set().iter() {
      for second in self.two_sets.second_set().iter() {
        if !(w3_relations.contains(&(first, second))) {
          return false;
        }
      }
    }

    for first_el in self.two_sets.first_set() {
      for second_el in self.two_sets.first_set() {
        if !provider.unrelated_relation(first_el, second_el) {
          return false;
        }
      }
    }

    for first_el in self.two_sets.second_set() {
      for second_el in self.two_sets.second_set() {
        if !provider.unrelated_relation(first_el, second_el) {
          return false;
        }
      }
    }

    true
  }

  pub fn merge(&self, other: &Self) -> Self {
    Self {
      two_sets: self.two_sets.merge(&other.two_sets),
    }
  }

  pub fn two_sets(&self) -> TwoSets<Rc<str>> {
    self.two_sets.clone()
  }
}

impl Hash for W3Pair {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.two_sets.hash(state)
  }
}

impl PartialEq for W3Pair {
  fn eq(&self, other: &Self) -> bool {
    compare_based_on_hashes(self, other)
  }
}

impl Eq for W3Pair {}

impl Clone for W3Pair {
  fn clone(&self) -> Self {
    Self {
      two_sets: self.two_sets.clone(),
    }
  }
}

impl Display for W3Pair {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.two_sets.to_string().as_str())
  }
}
