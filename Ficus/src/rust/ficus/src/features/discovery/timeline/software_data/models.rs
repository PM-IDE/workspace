use crate::features::discovery::timeline::{discovery::TraceThread, software_data::extraction_config::TimeKind};
use derive_new::new;
use enum_display::EnumDisplay;
use getset::{Getters, MutGetters};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, rc::Rc};

#[derive(Clone, Debug, Getters, MutGetters, Serialize, Deserialize, Default)]
pub struct SoftwareData {
  #[getset(get = "pub", get_mut = "pub")]
  #[serde(skip_serializing_if = "HashMap::is_empty")]
  event_classes: HashMap<Rc<str>, usize>,

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

  #[getset(get = "pub", get_mut = "pub")]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  ocel_data: Vec<OcelData>,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct OcelProducedObjectAfterConsume {
  #[getset(get = "pub")]
  id: Rc<str>,
  #[getset(get = "pub")]
  r#type: Option<Rc<str>>,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct ObjectTypeWithData<T> {
  #[getset(get = "pub")]
  r#type: Option<Rc<str>>,
  #[getset(get = "pub")]
  data: T,
}

#[derive(Clone, Debug, Serialize, Deserialize, EnumDisplay)]
pub enum OcelObjectAction {
  Allocate(ObjectTypeWithData<()>),
  Consume(ObjectTypeWithData<()>),
  AllocateMerged(ObjectTypeWithData<Vec<Rc<str>>>),
  ConsumeWithProduce(Vec<OcelProducedObjectAfterConsume>),
}

#[derive(Clone, Debug, Getters, new, Serialize, Deserialize)]
pub struct OcelData {
  #[getset(get = "pub")]
  object_id: Rc<str>,
  #[getset(get = "pub")]
  action: OcelObjectAction,
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
  name: Rc<str>,
  #[getset(get = "pub")]
  units: Rc<str>,
  #[getset(get = "pub")]
  group: Option<Rc<str>>,
}

#[derive(Clone, Debug, Getters, new, Serialize, Deserialize)]
pub struct HistogramEntry {
  #[getset(get = "pub")]
  name: Rc<str>,
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
  kind: DurationKind,
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
      TimeKind::UtcStamp => Self::Nanos,
    }
  }
}
