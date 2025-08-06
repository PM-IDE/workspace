use crate::features::mutations::mutations::{ARTIFICIAL_END_EVENT_NAME, ARTIFICIAL_START_EVENT_NAME};
use derive_new::new;
use fancy_regex::Regex;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, Setters, Getters, Serialize, Deserialize)]
pub struct SoftwareDataExtractionConfig {
  #[getset(get = "pub", set = "pub")]
  method_start: Option<ExtractionConfig<MethodStartEndConfig>>,
  #[getset(get = "pub", set = "pub")]
  method_end: Option<ExtractionConfig<MethodStartEndConfig>>,
  #[getset(get = "pub", set = "pub")]
  allocation: Option<ExtractionConfig<AllocationExtractionConfig>>,

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
  pub fn empty() -> Self {
    Self {
      method_start: None,
      method_end: None,
      allocation: None,
      raw_control_flow_regexes: vec![],
      pie_chart_extraction_configs: vec![],
      simple_counter_configs: vec![],
      activities_duration_configs: vec![],
    }
  }

  pub fn control_flow_regexes(&self) -> Result<Option<Vec<Regex>>, String> {
    if self.raw_control_flow_regexes.is_empty() {
      return Ok(None);
    }

    let mut result = vec![];
    for regex in &self.raw_control_flow_regexes {
      match Regex::new(regex) {
        Ok(regex) => result.push(regex),
        Err(err) => {
          return Err(format!("Failed to parse regex: error {}, raw regex {}", err.to_string(), regex));
        }
      }
    }

    result.push(Regex::new(ARTIFICIAL_START_EVENT_NAME).unwrap());
    result.push(Regex::new(ARTIFICIAL_END_EVENT_NAME).unwrap());

    Ok(Some(result))
  }
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct AllocationExtractionConfig {
  #[getset(get = "pub")]
  type_name_attr: String,
  #[getset(get = "pub")]
  allocated_count_attr: String,
  #[getset(get = "pub")]
  object_size_bytes_attr: Option<String>,
  #[getset(get = "pub")]
  total_allocated_bytes_attr: Option<String>,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct MethodStartEndConfig {
  #[getset(get = "pub")]
  method_attrs: MethodCommonAttributesConfig,
  #[getset(get = "pub")]
  prefix: Option<String>,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct MethodCommonAttributesConfig {
  #[getset(get = "pub")]
  name_attr: String,
  #[getset(get = "pub")]
  namespace_attr: String,
  #[getset(get = "pub")]
  signature_attr: String,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct ExtractionConfig<TConcreteInfo: Clone + Debug> {
  #[getset(get = "pub")]
  event_class_regex: String,
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
  count_attr: Option<String>,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct GenericExtractionConfigBase {
  #[getset(get = "pub")]
  name: String,
  #[getset(get = "pub")]
  units: String,
  #[getset(get = "pub")]
  group: Option<String>,
}

#[serde(rename_all = "snake_case")]
#[derive(Clone, Debug, Serialize, Deserialize, new)]
pub enum NameCreationStrategy {
  SingleAttribute(SingleAttribute),
  ManyAttributes(ManyAttributes),
}

impl NameCreationStrategy {
  pub fn fallback_value(&self) -> String {
    match self {
      NameCreationStrategy::SingleAttribute(s) => s.fallback_value().to_string(),
      NameCreationStrategy::ManyAttributes(m) => m.fallback_value().to_string(),
    }
  }
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct SingleAttribute {
  #[getset(get = "pub")]
  name: String,
  #[getset(get = "pub")]
  fallback_value: String,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct ManyAttributes {
  #[getset(get = "pub")]
  attributes: Vec<String>,
  #[getset(get = "pub")]
  separator: String,
  #[getset(get = "pub")]
  fallback_value: String,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct SimpleCountExtractionConfig {
  #[getset(get = "pub")]
  base: GenericExtractionConfigBase,
  #[getset(get = "pub")]
  count_attr: Option<String>,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct ActivityDurationExtractionConfig {
  #[getset(get = "pub")]
  base: GenericExtractionConfigBase,
  #[getset(get = "pub")]
  start_event_regex: String,
  #[getset(get = "pub")]
  end_event_regex: String,
  #[getset(get = "pub")]
  time_attribute: Option<TimeAttributeConfig>,
  #[getset(get = "pub")]
  activity_id_attr: Option<NameCreationStrategy>,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct TimeAttributeConfig {
  #[getset(get = "pub")]
  time_attribute: String,
  #[getset(get = "pub")]
  kind: TimeKind
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
}
