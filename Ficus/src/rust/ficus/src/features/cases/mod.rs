use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct CaseName {
  pub display_name: Arc<str>,
  pub name_parts: Vec<Arc<str>>,
}

impl CaseName {
  pub fn empty() -> Self {
    Self {
      name_parts: vec![],
      display_name: "UNDEFINED".into(),
    }
  }
}

pub mod cases_discovery;
mod cases_discovery_state;
