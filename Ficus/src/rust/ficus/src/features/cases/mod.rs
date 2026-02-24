#[derive(Clone, Debug)]
pub struct CaseName {
  pub display_name: String,
  pub name_parts: Vec<String>,
}

impl CaseName {
  pub fn empty() -> Self {
    Self {
      name_parts: vec![],
      display_name: "UNDEFINED".to_string(),
    }
  }
}

pub mod cases_discovery;
mod cases_discovery_state;
