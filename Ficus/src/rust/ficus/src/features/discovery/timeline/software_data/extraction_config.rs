use crate::features::mutations::mutations::{ARTIFICIAL_END_EVENT_NAME, ARTIFICIAL_START_EVENT_NAME};
use derive_new::new;
use fancy_regex::Regex;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, rc::Rc};

#[derive(Clone, Debug, Setters, Getters, Serialize, Deserialize, Default)]
pub struct SoftwareDataExtractionConfig {
  #[getset(get = "pub", set = "pub")]
  method_start: Option<ExtractionConfig<MethodStartEndConfig>>,
  #[getset(get = "pub", set = "pub")]
  method_end: Option<ExtractionConfig<MethodStartEndConfig>>,
  #[getset(get = "pub", set = "pub")]
  allocation: Option<ExtractionConfig<AllocationExtractionConfig>>,
  #[getset(get = "pub", set = "pub")]
  ocel: Option<OcelUnitedExtractionConfig>,

  #[getset(get = "pub", set = "pub")]
  raw_control_flow_regexes: Vec<String>,

  #[getset(get = "pub", set = "pub")]
  pie_chart_extraction_configs: Vec<ExtractionConfig<PieChartExtractionConfig>>,
  #[getset(get = "pub", set = "pub")]
  simple_counter_configs: Vec<ExtractionConfig<SimpleCountExtractionConfig>>,
  #[getset(get = "pub", set = "pub")]
  activities_duration_configs: Vec<ActivityDurationExtractionConfig>,
}

impl SoftwareDataExtractionConfig {
  pub fn control_flow_regexes(&self) -> Result<Option<Vec<Regex>>, String> {
    if self.raw_control_flow_regexes.is_empty() {
      return Ok(None);
    }

    let mut result = vec![];
    for regex in &self.raw_control_flow_regexes {
      match Regex::new(regex) {
        Ok(regex) => result.push(regex),
        Err(err) => {
          return Err(format!("Failed to parse regex: error {}, raw regex {}", err, regex));
        }
      }
    }

    result.push(Regex::new(ARTIFICIAL_START_EVENT_NAME).unwrap());
    result.push(Regex::new(ARTIFICIAL_END_EVENT_NAME).unwrap());

    Ok(Some(result))
  }
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct OcelUnitedExtractionConfig {
  #[getset(get = "pub")]
  delimiter: Option<String>,
  #[getset(get = "pub")]
  allocated: Option<ExtractionConfig<OcelObjectExtractionConfigBase>>,
  #[getset(get = "pub")]
  consumed: Option<ExtractionConfig<OcelObjectExtractionConfigBase>>,
  #[getset(get = "pub")]
  allocated_merged: Option<ExtractionConfig<OcelAllocateMergeExtractionConfig>>,
  #[getset(get = "pub")]
  consume_produce: Option<ExtractionConfig<OcelConsumeProduceExtractionConfig>>,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct OcelObjectExtractionConfigBase {
  #[getset(get = "pub")]
  object_type_attr: NameCreationStrategy,
  #[getset(get = "pub")]
  object_id_attr: String,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct OcelAllocateMergeExtractionConfig {
  #[getset(get = "pub")]
  allocated_obj: OcelObjectExtractionConfigBase,
  #[getset(get = "pub")]
  related_object_ids_attr: Rc<str>,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct OcelConsumeProduceExtractionConfig {
  #[getset(get = "pub")]
  object_id_attr: Rc<str>,
  #[getset(get = "pub")]
  related_object_ids_attr: Rc<str>,
  #[getset(get = "pub")]
  related_object_type_attr: Rc<str>,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct AllocationExtractionConfig {
  #[getset(get = "pub")]
  type_name_attr: Rc<str>,
  #[getset(get = "pub")]
  allocated_count_attr: Rc<str>,
  #[getset(get = "pub")]
  object_size_bytes_attr: Option<Rc<str>>,
  #[getset(get = "pub")]
  total_allocated_bytes_attr: Option<Rc<str>>,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct MethodStartEndConfig {
  #[getset(get = "pub")]
  method_attrs: MethodCommonAttributesConfig,
  #[getset(get = "pub")]
  prefix: Option<Rc<str>>,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct MethodCommonAttributesConfig {
  #[getset(get = "pub")]
  name_attr: Rc<str>,
  #[getset(get = "pub")]
  namespace_attr: Rc<str>,
  #[getset(get = "pub")]
  signature_attr: Rc<str>,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct ExtractionConfig<TConcreteInfo: Clone + Debug> {
  #[getset(get = "pub")]
  event_class_regex: Rc<str>,
  #[getset(get = "pub")]
  info: TConcreteInfo,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct PieChartExtractionConfig {
  #[getset(get = "pub")]
  base: GenericExtractionConfigBase,
  #[getset(get = "pub")]
  grouping_attr: Option<NameCreationStrategy>,
  #[getset(get = "pub")]
  count_attr: Option<Rc<str>>,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct GenericExtractionConfigBase {
  #[getset(get = "pub")]
  name: Rc<str>,
  #[getset(get = "pub")]
  units: Rc<str>,
  #[getset(get = "pub")]
  group: Option<Rc<str>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, new)]
#[serde(rename_all = "snake_case")]
pub enum NameCreationStrategy {
  SingleAttribute(SingleAttribute),
  ManyAttributes(ManyAttributes),
}

impl NameCreationStrategy {
  pub fn fallback_value(&self) -> Rc<str> {
    match self {
      NameCreationStrategy::SingleAttribute(s) => s.fallback_value().clone(),
      NameCreationStrategy::ManyAttributes(m) => m.fallback_value().clone(),
    }
  }
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct SingleAttribute {
  #[getset(get = "pub")]
  name: Rc<str>,
  #[getset(get = "pub")]
  fallback_value: Rc<str>,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct ManyAttributes {
  #[getset(get = "pub")]
  attributes: Vec<Rc<str>>,
  #[getset(get = "pub")]
  separator: Rc<str>,
  #[getset(get = "pub")]
  fallback_value: Rc<str>,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct SimpleCountExtractionConfig {
  #[getset(get = "pub")]
  base: GenericExtractionConfigBase,
  #[getset(get = "pub")]
  count_attr: Option<Rc<str>>,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct ActivityDurationExtractionConfig {
  #[getset(get = "pub")]
  base: GenericExtractionConfigBase,
  #[getset(get = "pub")]
  start_event_regex: Rc<str>,
  #[getset(get = "pub")]
  end_event_regex: Rc<str>,
  #[getset(get = "pub")]
  time_attribute: Option<TimeAttributeConfig>,
  #[getset(get = "pub")]
  activity_id_attr: Option<NameCreationStrategy>,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct TimeAttributeConfig {
  #[getset(get = "pub")]
  time_attribute: Rc<str>,
  #[getset(get = "pub")]
  kind: TimeKind,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TimeKind {
  Unknown,

  Nanos,
  Micros,
  Millis,
  Seconds,
  Minutes,
  Hours,
  Days,

  UtcStamp,
}
