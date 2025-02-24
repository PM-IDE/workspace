use crate::event_log::core::event_log::EventLog;
use crate::features::discovery::alpha::providers::alpha_plus_nfc_provider::AlphaPlusNfcRelationsProvider;
use crate::features::discovery::alpha::providers::alpha_provider::AlphaRelationsProvider;
use crate::utils::hash_utils::compare_based_on_hashes;
use crate::utils::sets::two_sets::TwoSets;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

pub(crate) struct W3Pair<'a> {
  two_sets: TwoSets<&'a String>,
}

impl<'a> W3Pair<'a> {
  pub fn new(first: &'a String, second: &'a String) -> Self {
    Self {
      two_sets: TwoSets::new_one_element(first, second),
    }
  }

  pub fn try_new<TLog: EventLog>(
    first: &'a String,
    second: &'a String,
    w3_relations: &HashSet<(&String, &String)>,
    provider: &AlphaPlusNfcRelationsProvider<TLog>,
  ) -> Option<Self> {
    let new_pair = Self::new(first, second);
    match new_pair.valid(w3_relations, provider) {
      true => Some(new_pair),
      false => None,
    }
  }

  pub fn valid<TLog: EventLog>(
    &self,
    w3_relations: &HashSet<(&String, &String)>,
    provider: &AlphaPlusNfcRelationsProvider<TLog>,
  ) -> bool {
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

  pub fn two_sets(&self) -> TwoSets<&'a String> {
    self.two_sets.clone()
  }
}

impl<'a> Hash for W3Pair<'a> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.two_sets.hash(state)
  }
}

impl<'a> PartialEq for W3Pair<'a> {
  fn eq(&self, other: &Self) -> bool {
    compare_based_on_hashes(self, other)
  }
}

impl<'a> Eq for W3Pair<'a> {}

impl<'a> Clone for W3Pair<'a> {
  fn clone(&self) -> Self {
    Self {
      two_sets: self.two_sets.clone(),
    }
  }
}

impl<'a> ToString for W3Pair<'a> {
  fn to_string(&self) -> String {
    self.two_sets.to_string()
  }
}
