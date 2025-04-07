use std::cell::RefCell;
use std::rc::Rc;
use lazy_static::lazy_static;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::features::analysis::patterns::entry_points::PatternsKind;
use crate::pipelines::keys::context_key::DefaultContextKey;
use crate::utils::graph::graph::DefaultGraph;

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
  Unknown
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
  underlying_sequence: Vec<Rc<RefCell<XesEventImpl>>>
}

impl UnderlyingPatternInfo {
  pub fn new(pattern_kind: UnderlyingPatternKind, underlying_sequence: Vec<Rc<RefCell<XesEventImpl>>>) -> Self {
    Self {
      pattern_kind,
      underlying_sequence
    }
  }

  pub fn pattern_kind(&self) -> &UnderlyingPatternKind {
    &self.pattern_kind
  }

  pub fn underlying_sequence(&self) -> &Vec<Rc<RefCell<XesEventImpl>>> {
    &self.underlying_sequence
  }
}

#[derive(Clone, Debug)]
pub struct UnderlyingPatternGraphInfo {
  pattern_kind: UnderlyingPatternKind,
  base_sequence: Vec<String>,
  graph: Rc<Box<DefaultGraph>>,
}

impl UnderlyingPatternGraphInfo {
  pub fn new(pattern_kind: UnderlyingPatternKind, base_sequence: Vec<String>, graph: Rc<Box<DefaultGraph>>) -> Self {
    Self {
      pattern_kind,
      base_sequence,
      graph
    }
  }
  
  pub fn pattern_kind(&self) -> UnderlyingPatternKind {
    self.pattern_kind
  }
  
  pub fn base_sequence(&self) -> &Vec<String> {
    &self.base_sequence
  }

  pub fn graph(&self) -> Rc<Box<DefaultGraph>> {
    self.graph.clone()
  }
}