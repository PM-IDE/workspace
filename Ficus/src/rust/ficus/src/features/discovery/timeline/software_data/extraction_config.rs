use crate::features::mutations::mutations::{ARTIFICIAL_END_EVENT_NAME, ARTIFICIAL_START_EVENT_NAME};
use derive_new::new;
use fancy_regex::Regex;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, Setters, Getters, Serialize, Deserialize)]
pub struct SoftwareDataExtractionConfig {
  #[getset(get = "pub", set = "pub")] allocation: Option<ExtractionConfig<AllocationExtractionConfig>>,
  #[getset(get = "pub", set = "pub")] exceptions: Option<ExtractionConfig<ExceptionExtractionConfig>>,
  #[getset(get = "pub", set = "pub")] http: Option<ExtractionConfig<HTTPExtractionConfig>>,

  #[getset(get = "pub", set = "pub")] method_inlining_success: Option<ExtractionConfig<MethodInliningSucceededConfig>>,
  #[getset(get = "pub", set = "pub")] method_inlining_failed: Option<ExtractionConfig<MethodInliningFailedConfig>>,

  #[getset(get = "pub", set = "pub")] method_load: Option<ExtractionConfig<MethodLoadUnloadConfig>>,
  #[getset(get = "pub", set = "pub")] method_unload: Option<ExtractionConfig<MethodLoadUnloadConfig>>,

  #[getset(get = "pub", set = "pub")] socket_connect_start: Option<ExtractionConfig<SocketConnectAcceptStartConfig>>,
  #[getset(get = "pub", set = "pub")] socket_connect_stop: Option<ExtractionConfig<()>>,
  #[getset(get = "pub", set = "pub")] socket_accept_start: Option<ExtractionConfig<SocketConnectAcceptStartConfig>>,
  #[getset(get = "pub", set = "pub")] socket_accept_stop: Option<ExtractionConfig<()>>,

  #[getset(get = "pub", set = "pub")] socket_connect_failed: Option<ExtractionConfig<SocketAcceptConnectFailedConfig>>,
  #[getset(get = "pub", set = "pub")] socket_accept_failed: Option<ExtractionConfig<SocketAcceptConnectFailedConfig>>,

  #[getset(get = "pub", set = "pub")] thread_created: Option<ExtractionConfig<ThreadExtractionConfig>>,

  #[getset(get = "pub", set = "pub")] array_pool_array_created: Option<ExtractionConfig<ArrayPoolExtractionConfig>>,
  #[getset(get = "pub", set = "pub")] array_pool_array_rented: Option<ExtractionConfig<ArrayPoolExtractionConfig>>,
  #[getset(get = "pub", set = "pub")] array_pool_array_returned: Option<ExtractionConfig<ArrayPoolExtractionConfig>>,
  #[getset(get = "pub", set = "pub")] array_pool_array_trimmed: Option<ExtractionConfig<ArrayPoolExtractionConfig>>,

  #[getset(get = "pub", set = "pub")] assembly_load: Option<ExtractionConfig<AssemblyExtractionConfig>>,
  #[getset(get = "pub", set = "pub")] assembly_unload: Option<ExtractionConfig<AssemblyExtractionConfig>>,

  #[getset(get = "pub", set = "pub")] suspend_ee: Option<ExtractionConfig<SuspendEEConfig>>,
  #[getset(get = "pub", set = "pub")] restart_ee: Option<ExtractionConfig<()>>,

  #[getset(get = "pub", set = "pub")] method_start: Option<ExtractionConfig<MethodStartEndConfig>>,
  #[getset(get = "pub", set = "pub")] method_end: Option<ExtractionConfig<MethodStartEndConfig>>,

  #[getset(get = "pub", set = "pub")] raw_control_flow_regexes: Vec<String>,

  #[getset(get = "pub", set = "pub")] histogram_extraction_configs: Vec<ExtractionConfig<HistogramExtractionConfig>>,
  #[getset(get = "pub", set = "pub")] simple_counter_configs: Vec<ExtractionConfig<SimpleCountExtractionConfig>>,
}

impl SoftwareDataExtractionConfig {
  pub fn empty() -> Self {
    Self {
      allocation: None,
      exceptions: None,
      http: None,
      method_inlining_success: None,
      method_inlining_failed: None,
      method_load: None,
      method_unload: None,
      socket_connect_start: None,
      socket_connect_stop: None,
      socket_accept_start: None,
      socket_accept_stop: None,
      socket_connect_failed: None,
      socket_accept_failed: None,
      thread_created: None,
      array_pool_array_created: None,
      array_pool_array_rented: None,
      array_pool_array_returned: None,
      array_pool_array_trimmed: None,
      assembly_load: None,
      assembly_unload: None,
      suspend_ee: None,
      restart_ee: None,
      method_start: None,
      method_end: None,
      raw_control_flow_regexes: vec![],
      histogram_extraction_configs: vec![],
      simple_counter_configs: vec![]
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
pub struct MethodLoadUnloadConfig {
  #[getset(get = "pub")] common_attrs: MethodCommonAttributesConfig
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct MethodStartEndConfig {
  #[getset(get = "pub")] method_attrs: MethodCommonAttributesConfig,
  #[getset(get = "pub")] prefix: Option<String>,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct MethodCommonAttributesConfig {
  #[getset(get = "pub")] name_attr: String,
  #[getset(get = "pub")] namespace_attr: String,
  #[getset(get = "pub")] signature_attr: String,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct SuspendEEConfig {
  #[getset(get = "pub")] reason_attr: String
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct ExtractionConfig<TConcreteInfo: Clone + Debug> {
  #[getset(get = "pub")] event_class_regex: String,
  #[getset(get = "pub")] info: TConcreteInfo,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct AllocationExtractionConfig {
  #[getset(get = "pub")] type_name_attr: String,
  #[getset(get = "pub")] allocated_count_attr: String,
  #[getset(get = "pub")] object_size_bytes_attr: Option<String>,
  #[getset(get = "pub")] total_allocated_bytes_attr: Option<String>,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct ExceptionExtractionConfig {
  #[getset(get = "pub")] type_name_attr: String,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct HTTPExtractionConfig {
  #[getset(get = "pub")] host_attr: String,
  #[getset(get = "pub")] port_attr: String,
  #[getset(get = "pub")] scheme_attr: String,
  #[getset(get = "pub")] path_and_query_attr: String
}

#[derive(Clone, Debug, Serialize, Deserialize, new)]
pub struct MethodInliningSuccessExtractionConfig {}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct MethodInliningSucceededConfig {
  #[getset(get = "pub")] inlining_config: MethodInliningConfig
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct MethodInliningConfig {
  #[getset(get = "pub")] inlinee_method_attrs: MethodCommonAttributesConfig,
  #[getset(get = "pub")] inliner_method_attrs: MethodCommonAttributesConfig,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct MethodInliningFailedConfig {
  #[getset(get = "pub")] inlining_config: MethodInliningConfig,
  #[getset(get = "pub")] fail_reason_attr: String,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct SocketConnectAcceptStartConfig {
  #[getset(get = "pub")] address_attr: String,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct SocketAcceptConnectFailedConfig {
  #[getset(get = "pub")] error_code_attr: String,
  #[getset(get = "pub")] error_message_attr: String,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct ThreadExtractionConfig {
  #[getset(get = "pub")] thread_id_attr: String,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct ArrayPoolExtractionConfig {
  #[getset(get = "pub")] buffer_id_attr: String,
  #[getset(get = "pub")] buffer_size_attr: String,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct AssemblyExtractionConfig {
  #[getset(get = "pub")] assembly_name_attr: String,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct HistogramExtractionConfig {
  #[getset(get = "pub")] name: String,
  #[getset(get = "pub")] grouping_attr: String,
  #[getset(get = "pub")] count_attr: String
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct SimpleCountExtractionConfig {
  #[getset(get = "pub")] count_attr: Option<String>,
  #[getset(get = "pub")] category_attr: Option<String>
}