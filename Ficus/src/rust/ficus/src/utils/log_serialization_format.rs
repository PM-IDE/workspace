use std::str::FromStr;

pub enum LogSerializationFormat {
  Xes,
  Bxes,
}

impl LogSerializationFormat {
  pub fn extension(&self) -> &str {
    match &self {
      LogSerializationFormat::Xes => "xes",
      LogSerializationFormat::Bxes => "bxes",
    }
  }
}

impl FromStr for LogSerializationFormat {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Xes" => Ok(Self::Xes),
      "Bxes" => Ok(Self::Bxes),
      _ => Err(()),
    }
  }
}
