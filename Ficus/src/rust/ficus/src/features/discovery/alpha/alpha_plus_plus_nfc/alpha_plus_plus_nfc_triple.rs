use crate::event_log::core::event_log::EventLog;
use crate::features::discovery::alpha::providers::alpha_plus_nfc_provider::AlphaPlusNfcRelationsProvider;
use crate::features::discovery::alpha::providers::alpha_plus_provider::AlphaPlusRelationsProvider;
use crate::features::discovery::alpha::providers::alpha_provider::AlphaRelationsProvider;
use crate::utils::hash_utils::compare_based_on_hashes;
use crate::utils::sets::two_sets::TwoSets;
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};

pub(crate) struct AlphaPlusPlusNfcTriple<'a> {
  a_classes: BTreeSet<&'a String>,
  b_classes: BTreeSet<&'a String>,
  c_classes: BTreeSet<&'a String>,
}

impl<'a> AlphaPlusPlusNfcTriple<'a> {
  pub fn new(a_class: &'a String, b_class: &'a String, c_class: &'a String) -> Self {
    Self {
      a_classes: BTreeSet::from_iter(vec![a_class]),
      b_classes: BTreeSet::from_iter(vec![b_class]),
      c_classes: BTreeSet::from_iter(vec![c_class]),
    }
  }

  pub fn try_new<TLog: EventLog>(
    a_class: &'a String,
    b_class: &'a String,
    c_class: &'a String,
    provider: &AlphaPlusNfcRelationsProvider<'a, TLog>,
  ) -> Option<Self> {
    let candidate = Self::new(a_class, b_class, c_class);
    match candidate.valid(provider) {
      true => Some(candidate),
      false => None,
    }
  }

  pub fn try_merge<TLog: EventLog>(first: &Self, second: &Self, provider: &AlphaPlusNfcRelationsProvider<'a, TLog>) -> Option<Self> {
    let merge_sets = |first: &BTreeSet<&'a String>, second: &BTreeSet<&'a String>| -> BTreeSet<&'a String> {
      first.iter().chain(second.iter()).map(|class| *class).collect()
    };

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

  pub fn valid<TLog: EventLog>(&self, provider: &AlphaPlusNfcRelationsProvider<'a, TLog>) -> bool {
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

  pub fn two_sets(&self) -> TwoSets<&'a String> {
    let first = self.a_classes.iter().chain(self.c_classes.iter());
    let second = self.b_classes.iter().chain(self.c_classes.iter());

    TwoSets::new(first.map(|c| *c).collect(), second.map(|c| *c).collect())
  }

  pub fn a_classes(&self) -> &BTreeSet<&'a String> {
    &self.a_classes
  }

  pub fn b_classes(&self) -> &BTreeSet<&'a String> {
    &self.b_classes
  }

  pub fn c_classes(&self) -> &BTreeSet<&'a String> {
    &self.c_classes
  }
}

impl<'a> Hash for AlphaPlusPlusNfcTriple<'a> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    let mut hash_classes = |set: &BTreeSet<&'a String>| {
      for class in set {
        state.write(class.as_bytes());
      }
    };

    hash_classes(&self.a_classes);
    hash_classes(&self.b_classes);
    hash_classes(&self.c_classes);
  }
}

impl<'a> PartialEq for AlphaPlusPlusNfcTriple<'a> {
  fn eq(&self, other: &Self) -> bool {
    compare_based_on_hashes(self, other)
  }
}

impl<'a> Clone for AlphaPlusPlusNfcTriple<'a> {
  fn clone(&self) -> Self {
    Self {
      a_classes: self.a_classes.clone(),
      b_classes: self.b_classes.clone(),
      c_classes: self.c_classes.clone(),
    }
  }
}

impl<'a> Eq for AlphaPlusPlusNfcTriple<'a> {}

impl<'a> ToString for AlphaPlusPlusNfcTriple<'a> {
  fn to_string(&self) -> String {
    let mut repr = String::new();
    repr.push('(');

    let mut push_set = |set: &BTreeSet<&'a String>| {
      repr.push('{');

      for class in set.iter() {
        repr.push_str(class.as_str());
        repr.push(',')
      }

      if set.len() > 0 {
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
    repr
  }
}
