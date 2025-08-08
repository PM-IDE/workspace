use crate::features::discovery::timeline::discovery::TraceThread;
use derive_new::new;
use getset::{Getters, MutGetters};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::features::discovery::timeline::software_data::extraction_config::TimeKind;

#[derive(Clone, Debug, Getters, MutGetters, Serialize, Deserialize)]
pub struct SoftwareData {
  #[getset(get = "pub", get_mut = "pub")]
  #[serde(skip_serializing_if = "HashMap::is_empty")]
  event_classes: HashMap<String, usize>,

  #[getset(get = "pub", get_mut = "pub")]
  #[serde(skip)]
  thread_diagram_fragment: Vec<TraceThread>,

  #[getset(get = "pub", get_mut = "pub")]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  histograms: Vec<HistogramData>,

  #[getset(get = "pub", get_mut = "pub")]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  simple_counters: Vec<SimpleCounterData>,

  #[getset(get = "pub", get_mut = "pub")]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  activities_durations: Vec<ActivityDurationData>,
}

impl SoftwareData {
  pub fn empty() -> Self {
    Self {
      event_classes: HashMap::new(),
      thread_diagram_fragment: vec![],
      histograms: vec![],
      simple_counters: vec![],
      activities_durations: vec![],
    }
  }
}

#[derive(Clone, Debug, Getters, MutGetters, new, Serialize, Deserialize)]
pub struct HistogramData {
  #[getset(get = "pub")]
  base: GenericEnhancementBase,
  #[getset(get = "pub", get_mut = "pub")]
  entries: Vec<HistogramEntry>,
}

#[derive(Clone, Debug, Getters, MutGetters, new, Serialize, Deserialize)]
pub struct GenericEnhancementBase {
  #[getset(get = "pub")]
  name: String,
  #[getset(get = "pub")]
  units: String,
  #[getset(get = "pub")]
  group: Option<String>,
}

#[derive(Clone, Debug, Getters, new, Serialize, Deserialize)]
pub struct HistogramEntry {
  #[getset(get = "pub")]
  name: String,
  #[getset(get = "pub")]
  value: f64,
}

#[derive(Clone, Debug, Getters, MutGetters, new, Serialize, Deserialize)]
pub struct SimpleCounterData {
  #[getset(get = "pub")]
  base: GenericEnhancementBase,
  #[getset(get = "pub")]
  value: f64,
}

#[derive(Clone, Debug, Getters, MutGetters, new, Serialize, Deserialize)]
pub struct ActivityDurationData {
  #[getset(get = "pub")]
  base: GenericEnhancementBase,
  #[getset(get = "pub")]
  duration: u64,
  #[getset(get = "pub")]
  kind: DurationKind
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DurationKind {
  Unknown,

  Nanos,
  Micros,
  Millis,
  Seconds,
  Minutes,
  Hours,
  Days,
}

impl From<&TimeKind> for DurationKind {
  fn from(value: &TimeKind) -> Self {
    match value {
      TimeKind::Unknown => Self::Unknown,
      TimeKind::Nanos => Self::Nanos,
      TimeKind::Micros => Self::Micros,
      TimeKind::Millis => Self::Millis,
      TimeKind::Seconds => Self::Seconds,
      TimeKind::Minutes => Self::Minutes,
      TimeKind::Hours => Self::Hours,
      TimeKind::Days => Self::Days,
      TimeKind::UtcStamp => Self::Nanos
    }
  }
}