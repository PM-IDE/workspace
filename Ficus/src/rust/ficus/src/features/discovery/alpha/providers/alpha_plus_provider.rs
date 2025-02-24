use crate::features::analysis::log_info::event_log_info::EventLogInfo;
use crate::features::discovery::alpha::providers::alpha_provider::AlphaRelationsProvider;
use crate::features::discovery::relations::triangle_relation::TriangleRelation;
use std::collections::HashSet;

pub trait AlphaPlusRelationsProvider: AlphaRelationsProvider {
  fn triangle_relation(&self, first: &str, second: &str) -> bool;
  fn romb_relation(&self, first: &str, second: &str) -> bool;

  fn one_length_loop_transitions(&self) -> &HashSet<String>;
}

pub struct AlphaPlusRelationsProviderImpl<'a> {
  pub log_info: &'a dyn EventLogInfo,
  triangle_relation: &'a dyn TriangleRelation,
  one_length_loop_transitions: &'a HashSet<String>,
}

impl<'a> AlphaPlusRelationsProviderImpl<'a> {
  pub fn new(
    log_info: &'a dyn EventLogInfo,
    triangle_relation: &'a dyn TriangleRelation,
    one_length_loop_transitions: &'a HashSet<String>,
  ) -> Self {
    Self {
      log_info,
      triangle_relation,
      one_length_loop_transitions,
    }
  }
}

impl<'a> AlphaRelationsProvider for AlphaPlusRelationsProviderImpl<'a> {
  fn causal_relation(&self, first: &str, second: &str) -> bool {
    self.direct_relation(first, second) && (!self.direct_relation(second, first) || self.romb_relation(first, second))
  }

  fn parallel_relation(&self, first: &str, second: &str) -> bool {
    self.direct_relation(first, second) && self.direct_relation(second, first) && !self.romb_relation(first, second)
  }

  fn direct_relation(&self, first: &str, second: &str) -> bool {
    self.log_info.dfg_info().is_in_directly_follows_relation(first, second)
  }

  fn unrelated_relation(&self, first: &str, second: &str) -> bool {
    !self.direct_relation(first, second) && !self.direct_relation(second, first)
  }

  fn log_info(&self) -> &dyn EventLogInfo {
    self.log_info
  }
}

impl<'a> AlphaPlusRelationsProvider for AlphaPlusRelationsProviderImpl<'a> {
  fn triangle_relation(&self, first: &str, second: &str) -> bool {
    self.triangle_relation.get(first, second).is_some()
  }

  fn romb_relation(&self, first: &str, second: &str) -> bool {
    self.triangle_relation(first, second) && self.triangle_relation(second, first)
  }

  fn one_length_loop_transitions(&self) -> &HashSet<String> {
    self.one_length_loop_transitions
  }
}
