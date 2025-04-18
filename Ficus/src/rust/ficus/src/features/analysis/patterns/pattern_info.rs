use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::analysis::patterns::entry_points::PatternsKind;
use crate::pipelines::keys::context_key::DefaultContextKey;
use crate::utils::graph::graph::DefaultGraph;
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::rc::Rc;

lazy_static!(
  pub static ref UNDERLYING_PATTERN_KIND_KEY: DefaultContextKey<UnderlyingPatternKind> = DefaultContextKey::new("UNDERLYING_PATTERN_KIND");
);

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
      PatternsKind::NearSuperMaximalRepeats => Self::NearSuperMaximalRepeat
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
  graph: Rc<Box<DefaultGraph>>,
}

impl UnderlyingPatternGraphInfo {
  pub fn new(pattern_kind: UnderlyingPatternKind, base_pattern: Option<Vec<String>>, graph: Rc<Box<DefaultGraph>>) -> Self {
    Self {
      pattern_kind,
      base_pattern,
      graph,
    }
  }

  pub fn pattern_kind(&self) -> UnderlyingPatternKind {
    self.pattern_kind
  }

  pub fn base_pattern(&self) -> Option<&Vec<String>> {
    self.base_pattern.as_ref()
  }

  pub fn graph(&self) -> Rc<Box<DefaultGraph>> {
    self.graph.clone()
  }
}