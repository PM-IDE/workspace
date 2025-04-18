use derive_new::new;
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

  #[getset(get = "pub", set = "pub")] contention: Option<ExtractionConfig<ContentionExtractionConfig>>,
  #[getset(get = "pub", set = "pub")] socket: Option<ExtractionConfig<SocketExtractionConfig>>,
  #[getset(get = "pub", set = "pub")] thread: Option<ExtractionConfig<ThreadExtractionConfig>>,

  #[getset(get = "pub", set = "pub")] array_pool_array_created: Option<ExtractionConfig<ArrayPoolExtractionConfig>>,
  #[getset(get = "pub", set = "pub")] array_pool_array_rented: Option<ExtractionConfig<ArrayPoolExtractionConfig>>,
  #[getset(get = "pub", set = "pub")] array_pool_array_returned: Option<ExtractionConfig<ArrayPoolExtractionConfig>>,

  #[getset(get = "pub", set = "pub")] array_pool_array_trimmed: Option<ExtractionConfig<ArrayPoolExtractionConfig>>,

  #[getset(get = "pub", set = "pub")] assembly_load: Option<ExtractionConfig<AssemblyExtractionConfig>>,
  #[getset(get = "pub", set = "pub")] assembly_unload: Option<ExtractionConfig<AssemblyExtractionConfig>>,
}

impl SoftwareDataExtractionConfig {
  pub fn empty() -> Self {
    Self {
      allocation: None,
      exceptions: None,
      http: None,
      method_inlining_success: None,
      method_inlining_failed: None,
      contention: None,
      socket: None,
      thread: None,
      array_pool_array_created: None,
      array_pool_array_rented: None,
      array_pool_array_returned: None,
      array_pool_array_trimmed: None,
      assembly_load: None,
      assembly_unload: None,
    }
  }
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
  #[getset(get = "pub")] path_attr: String,
  #[getset(get = "pub")] query_attr: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, new)]
pub struct MethodInliningSuccessExtractionConfig {}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct MethodInliningSucceededConfig {
  #[getset(get = "pub")] method_name_attr: String,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct MethodInliningFailedConfig {
  #[getset(get = "pub")] method_name_attr: String,
  #[getset(get = "pub")] reason_attr: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, new)]
pub struct ContentionExtractionConfig {}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct SocketExtractionConfig {
  #[getset(get = "pub")] address_attr: String,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct ThreadExtractionConfig {
  #[getset(get = "pub")] thread_id_attr: String,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct ArrayPoolExtractionConfig {
  #[getset(get = "pub")] buffer_id: String,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct AssemblyExtractionConfig {
  name_attr: String,
}