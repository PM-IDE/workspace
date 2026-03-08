use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct CaseName {
  pub display_name: Rc<str>,
  pub name_parts: Vec<Rc<str>>,
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
