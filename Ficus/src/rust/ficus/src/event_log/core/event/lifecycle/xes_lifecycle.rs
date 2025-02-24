use super::{braf_lifecycle::XesBrafLifecycle, standard_lifecycle::XesStandardLifecycle};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Lifecycle {
  XesStandardLifecycle(XesStandardLifecycle),
  BrafLifecycle(XesBrafLifecycle),
}

impl ToString for Lifecycle {
  fn to_string(&self) -> String {
    match self {
      Self::XesStandardLifecycle(xes_lifecycle) => xes_lifecycle.to_string(),
      Self::BrafLifecycle(braf_lifecycle) => braf_lifecycle.to_string(),
    }
  }
}

impl FromStr for Lifecycle {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if let Ok(standard_lifecycle) = XesStandardLifecycle::from_str(s) {
      return Ok(Lifecycle::XesStandardLifecycle(standard_lifecycle));
    }

    if let Ok(braf_lifecycle) = XesBrafLifecycle::from_str(s) {
      return Ok(Lifecycle::BrafLifecycle(braf_lifecycle));
    }

    Ok(Lifecycle::XesStandardLifecycle(XesStandardLifecycle::Unspecified))
  }
}
