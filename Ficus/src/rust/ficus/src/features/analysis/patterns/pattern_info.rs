use crate::{
  context_key, event_log::xes::xes_event::XesEventImpl, features::analysis::patterns::entry_points::PatternsKind,
  utils::graph::graph::DefaultGraph,
};
use lazy_static::lazy_static;
use std::{cell::RefCell, rc::Rc};

const UNDERLYING_PATTERN_KIND: &str = "UNDERLYING_PATTERN_KIND";

context_key! { UNDERLYING_PATTERN_KIND, UnderlyingPatternKind }

#[derive(Debug, Clone, Copy)]
pub enum UnderlyingPatternKind {
  StrictLoop,
  PrimitiveTandemArray,
  MaximalTandemArray,
  MaximalRepeat,
  SuperMaximalRepeat,
  NearSuperMaximalRepeat,
  Unknown,
}

impl From<PatternsKind> for UnderlyingPatternKind {
  fn from(value: PatternsKind) -> Self {
    match value {
      PatternsKind::PrimitiveTandemArrays(_) => Self::PrimitiveTandemArray,
      PatternsKind::MaximalTandemArrays(_) => Self::MaximalTandemArray,
      PatternsKind::MaximalRepeats => Self::MaximalRepeat,
      PatternsKind::SuperMaximalRepeats => Self::SuperMaximalRepeat,
      PatternsKind::NearSuperMaximalRepeats => Self::NearSuperMaximalRepeat,
    }
  }
}

#[derive(Clone, Debug)]
pub struct UnderlyingPatternInfo {
  pattern_kind: UnderlyingPatternKind,
  underlying_sequence: Vec<Rc<RefCell<XesEventImpl>>>,
  base_pattern: Option<Vec<Rc<RefCell<XesEventImpl>>>>,
}

impl UnderlyingPatternInfo {
  pub fn new(
    pattern_kind: UnderlyingPatternKind,
    underlying_sequence: Vec<Rc<RefCell<XesEventImpl>>>,
    base_pattern: Option<Vec<Rc<RefCell<XesEventImpl>>>>,
  ) -> Self {
    Self {
      pattern_kind,
      underlying_sequence,
      base_pattern,
    }
  }

  pub fn pattern_kind(&self) -> &UnderlyingPatternKind {
    &self.pattern_kind
  }

  pub fn underlying_sequence(&self) -> &Vec<Rc<RefCell<XesEventImpl>>> {
    &self.underlying_sequence
  }

  pub fn base_pattern(&self) -> Option<&Vec<Rc<RefCell<XesEventImpl>>>> {
    self.base_pattern.as_ref()
  }
}

#[derive(Clone, Debug)]
pub struct UnderlyingPatternGraphInfo {
  pattern_kind: UnderlyingPatternKind,
  base_pattern: Option<Vec<String>>,
  graph: Box<DefaultGraph>,
}

impl UnderlyingPatternGraphInfo {
  pub fn new(pattern_kind: UnderlyingPatternKind, base_pattern: Option<Vec<String>>, graph: DefaultGraph) -> Self {
    Self {
      pattern_kind,
      base_pattern,
      graph: Box::new(graph),
    }
  }

  pub fn pattern_kind(&self) -> UnderlyingPatternKind {
    self.pattern_kind
  }

  pub fn base_pattern(&self) -> Option<&Vec<String>> {
    self.base_pattern.as_ref()
  }

  pub fn graph(&self) -> &DefaultGraph {
    self.graph.as_ref()
  }
}
