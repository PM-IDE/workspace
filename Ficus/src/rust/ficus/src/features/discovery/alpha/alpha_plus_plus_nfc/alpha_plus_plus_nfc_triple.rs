use crate::{
  event_log::core::event_log::EventLog,
  features::discovery::alpha::providers::{
    alpha_plus_nfc_provider::AlphaPlusNfcRelationsProvider, alpha_plus_provider::AlphaPlusRelationsProvider,
    alpha_provider::AlphaRelationsProvider,
  },
  utils::{hash_utils::compare_based_on_hashes, sets::two_sets::TwoSets},
};
use std::rc::Rc;
pub use std::{
  collections::BTreeSet,
  fmt::Display,
  hash::{Hash, Hasher},
};

pub(crate) struct AlphaPlusPlusNfcTriple {
  a_classes: BTreeSet<Rc<str>>,
  b_classes: BTreeSet<Rc<str>>,
  c_classes: BTreeSet<Rc<str>>,
}

impl AlphaPlusPlusNfcTriple {
  pub fn new(a_class: Rc<str>, b_class: Rc<str>, c_class: Rc<str>) -> Self {
    Self {
      a_classes: BTreeSet::from_iter(vec![a_class]),
      b_classes: BTreeSet::from_iter(vec![b_class]),
      c_classes: BTreeSet::from_iter(vec![c_class]),
    }
  }

  pub fn try_new<TLog: EventLog>(
    a_class: Rc<str>,
    b_class: Rc<str>,
    c_class: Rc<str>,
    provider: &AlphaPlusNfcRelationsProvider<'_, TLog>,
  ) -> Option<Self> {
    let candidate = Self::new(a_class, b_class, c_class);
    match candidate.valid(provider) {
      true => Some(candidate),
      false => None,
    }
  }

  pub fn try_merge<TLog: EventLog>(first: &Self, second: &Self, provider: &AlphaPlusNfcRelationsProvider<'_, TLog>) -> Option<Self> {
    let merge_sets =
      |first: &BTreeSet<Rc<str>>, second: &BTreeSet<Rc<str>>| -> BTreeSet<Rc<str>> { first.iter().chain(second.iter()).cloned().collect() };

    let new_triple = Self {
      a_classes: merge_sets(&first.a_classes, &second.a_classes),
      b_classes: merge_sets(&first.b_classes, &second.b_classes),
      c_classes: merge_sets(&first.c_classes, &second.c_classes),
    };

    match new_triple.valid(provider) {
      true => Some(new_triple),
      false => None,
    }
  }

  pub fn valid<TLog: EventLog>(&self, provider: &AlphaPlusNfcRelationsProvider<'_, TLog>) -> bool {
    for a_class in &self.a_classes {
      for b_class in &self.b_classes {
        for c_class in &self.c_classes {
          if !(provider.direct_relation(a_class, c_class) && !provider.triangle_relation(c_class, a_class)) {
            return false;
          }

          if !(provider.direct_relation(c_class, b_class) && !provider.triangle_relation(c_class, b_class)) {
            return false;
          }

          if provider.parallel_relation(a_class, b_class) {
            return false;
          }

          if !provider.unrelated_relation(a_class, a_class) || !provider.unrelated_relation(b_class, b_class) {
            return false;
          }
        }
      }
    }

    true
  }

  pub fn two_sets(&self) -> TwoSets<Rc<str>> {
    let first = self.a_classes.iter().chain(self.c_classes.iter());
    let second = self.b_classes.iter().chain(self.c_classes.iter());

    TwoSets::new(first.cloned().collect(), second.cloned().collect())
  }

  pub fn a_classes(&self) -> &BTreeSet<Rc<str>> {
    &self.a_classes
  }

  pub fn b_classes(&self) -> &BTreeSet<Rc<str>> {
    &self.b_classes
  }
}

impl Hash for AlphaPlusPlusNfcTriple {
  fn hash<H: Hasher>(&self, state: &mut H) {
    let mut hash_classes = |set: &BTreeSet<Rc<str>>| {
      for class in set {
        state.write(class.as_bytes());
      }
    };

    hash_classes(&self.a_classes);
    hash_classes(&self.b_classes);
    hash_classes(&self.c_classes);
  }
}

impl PartialEq for AlphaPlusPlusNfcTriple {
  fn eq(&self, other: &Self) -> bool {
    compare_based_on_hashes(self, other)
  }
}

impl Clone for AlphaPlusPlusNfcTriple {
  fn clone(&self) -> Self {
    Self {
      a_classes: self.a_classes.clone(),
      b_classes: self.b_classes.clone(),
      c_classes: self.c_classes.clone(),
    }
  }
}

impl Eq for AlphaPlusPlusNfcTriple {}

impl Display for AlphaPlusPlusNfcTriple {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut repr = String::new();
    repr.push('(');

    let mut push_set = |set: &BTreeSet<Rc<str>>| {
      repr.push('{');

      for class in set.iter() {
        repr.push_str(class);
        repr.push(',')
      }

      if !set.is_empty() {
        repr.remove(repr.len() - 1);
      }

      repr.push_str("}, ");
    };

    push_set(&self.a_classes);
    push_set(&self.b_classes);
    push_set(&self.c_classes);

    repr.remove(repr.len() - 1);
    repr.remove(repr.len() - 1);

    repr.push(')');

    write!(f, "{}", repr)
  }
}
