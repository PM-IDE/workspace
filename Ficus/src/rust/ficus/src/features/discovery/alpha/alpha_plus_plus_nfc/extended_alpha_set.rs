use crate::{
  event_log::core::event_log::EventLog,
  features::discovery::alpha::{
    alpha_set::AlphaSet,
    providers::{alpha_plus_nfc_provider::AlphaPlusNfcRelationsProvider, alpha_provider::AlphaRelationsProvider},
  },
  utils::{hash_utils::compare_based_on_hashes, sets::two_sets::TwoSets},
};
use std::{
  collections::{BTreeSet, HashSet},
  fmt::Display,
  hash::{Hash, Hasher},
  sync::Arc,
};

pub(crate) struct ExtendedAlphaSet {
  alpha_set: AlphaSet,
  left_extension: BTreeSet<Arc<str>>,
  right_extension: BTreeSet<Arc<str>>,
}

impl ExtendedAlphaSet {
  pub fn new_without_extensions(alpha_set: AlphaSet) -> Self {
    Self {
      alpha_set,
      left_extension: BTreeSet::new(),
      right_extension: BTreeSet::new(),
    }
  }

  pub fn new(alpha_set: AlphaSet, left_extension: Arc<str>, right_extension: Arc<str>) -> Self {
    Self {
      alpha_set,
      left_extension: BTreeSet::from_iter(vec![left_extension]),
      right_extension: BTreeSet::from_iter(vec![right_extension]),
    }
  }

  pub fn new_only_left(alpha_set: AlphaSet, left_extension: Arc<str>) -> Self {
    Self {
      alpha_set,
      left_extension: BTreeSet::from_iter(vec![left_extension]),
      right_extension: BTreeSet::new(),
    }
  }

  pub fn new_only_right(alpha_set: AlphaSet, right_extension: Arc<str>) -> Self {
    Self {
      alpha_set,
      left_extension: BTreeSet::new(),
      right_extension: BTreeSet::from_iter(vec![right_extension]),
    }
  }

  pub fn try_new<TLog: EventLog>(
    alpha_set: AlphaSet,
    left_extension: Arc<str>,
    right_extension: Arc<str>,
    provider: &mut AlphaPlusNfcRelationsProvider<TLog>,
    w1_relations: &HashSet<(&Arc<str>, &Arc<str>)>,
    w2_relations: &HashSet<(&Arc<str>, &Arc<str>)>,
  ) -> Option<Self> {
    Self::try_new_internal(provider, w1_relations, w2_relations, move || {
      Self::new(alpha_set, left_extension, right_extension)
    })
  }

  fn try_new_internal<TLog: EventLog>(
    provider: &mut AlphaPlusNfcRelationsProvider<TLog>,
    w1_relations: &HashSet<(&Arc<str>, &Arc<str>)>,
    w2_relations: &HashSet<(&Arc<str>, &Arc<str>)>,
    factory: impl FnOnce() -> Self,
  ) -> Option<Self> {
    let new_set = factory();
    match new_set.valid(provider, w1_relations, w2_relations) {
      true => Some(new_set),
      false => None,
    }
  }

  pub fn try_new_only_left<TLog: EventLog>(
    alpha_set: AlphaSet,
    left_extension: Arc<str>,
    provider: &mut AlphaPlusNfcRelationsProvider<TLog>,
    w1_relations: &HashSet<(&Arc<str>, &Arc<str>)>,
    w2_relations: &HashSet<(&Arc<str>, &Arc<str>)>,
  ) -> Option<Self> {
    Self::try_new_internal(provider, w1_relations, w2_relations, || {
      Self::new_only_left(alpha_set, left_extension)
    })
  }

  pub fn try_new_only_right<TLog: EventLog>(
    alpha_set: AlphaSet,
    right_extension: Arc<str>,
    provider: &mut AlphaPlusNfcRelationsProvider<TLog>,
    w1_relations: &HashSet<(&Arc<str>, &Arc<str>)>,
    w2_relations: &HashSet<(&Arc<str>, &Arc<str>)>,
  ) -> Option<Self> {
    Self::try_new_internal(provider, w1_relations, w2_relations, || {
      Self::new_only_right(alpha_set, right_extension)
    })
  }

  pub fn valid<TLog: EventLog>(
    &self,
    provider: &mut AlphaPlusNfcRelationsProvider<TLog>,
    w1_relations: &HashSet<(&Arc<str>, &Arc<str>)>,
    w2_relations: &HashSet<(&Arc<str>, &Arc<str>)>,
  ) -> bool {
    for a in &self.left_extension {
      if self.alpha_set.contains_left(a) {
        return false;
      }
    }

    for b in &self.right_extension {
      if self.alpha_set.contains_right(b) {
        return false;
      }
    }

    for a_class in self.alpha_set.left_classes() {
      for b in &self.right_extension {
        if !(w1_relations.contains(&(a_class, b)) || w2_relations.contains(&(a_class, b))) {
          return false;
        }
      }
    }

    for b_class in self.alpha_set.right_classes().into_iter().chain(self.right_extension.iter()) {
      for a in &self.left_extension {
        if !(w1_relations.contains(&(a, b_class)) || w2_relations.contains(&(a, b_class))) {
          return false;
        }
      }
    }

    for a_class in self.alpha_set.left_classes() {
      for a in &self.left_extension {
        if !(provider.unrelated_relation(a, a_class) && !provider.right_double_arrow_relation(a, a_class)) {
          return false;
        }
      }
    }

    for b_class in self.alpha_set.right_classes() {
      for b in &self.right_extension {
        if !(provider.unrelated_relation(b_class, b) && !provider.right_double_arrow_relation(b_class, b)) {
          return false;
        }
      }
    }

    true
  }

  pub fn merge(&self, other: &Self) -> Self {
    Self {
      alpha_set: self.alpha_set.extend(&other.alpha_set),
      left_extension: self.left_extension.iter().chain(&other.left_extension).cloned().collect(),
      right_extension: self.right_extension.iter().chain(&other.right_extension).cloned().collect(),
    }
  }

  pub fn two_sets(&self) -> TwoSets<Arc<str>> {
    let first = self.alpha_set.left_classes();
    let first = first.into_iter().chain(&self.left_extension);

    let second = self.alpha_set.right_classes();
    let second = second.into_iter().chain(&self.right_extension);

    TwoSets::new(first.cloned().collect(), second.cloned().collect())
  }

  pub fn alpha_set(&self) -> &AlphaSet {
    &self.alpha_set
  }
}

impl Hash for ExtendedAlphaSet {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.alpha_set.hash(state);
    for left in &self.left_extension {
      state.write(left.as_bytes());
    }

    for right in &self.right_extension {
      state.write(right.as_bytes());
    }
  }
}

impl PartialEq for ExtendedAlphaSet {
  fn eq(&self, other: &Self) -> bool {
    compare_based_on_hashes(self, other)
  }
}

impl Eq for ExtendedAlphaSet {}

impl Display for ExtendedAlphaSet {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut repr = String::new();
    repr.push('(');
    repr.push_str(self.alpha_set.to_string().as_str());
    repr.push_str(", ");

    let mut serialize_set = |set: &BTreeSet<Arc<str>>| {
      repr.push('{');
      for item in set {
        repr.push_str(item);
        repr.push(',');
      }

      if !set.is_empty() {
        repr.remove(repr.len() - 1);
      }

      repr.push_str("}, ");
    };

    serialize_set(&self.left_extension);
    serialize_set(&self.right_extension);

    repr.remove(repr.len() - 1);
    repr.remove(repr.len() - 1);

    repr.push(')');

    write!(f, "{}", repr)
  }
}

impl Clone for ExtendedAlphaSet {
  fn clone(&self) -> Self {
    Self {
      alpha_set: self.alpha_set.clone(),
      left_extension: self.left_extension.iter().cloned().collect(),
      right_extension: self.right_extension.iter().cloned().collect(),
    }
  }
}
