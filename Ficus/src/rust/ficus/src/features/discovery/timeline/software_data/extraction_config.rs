use derive_new::new;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, Setters, Getters, Serialize, Deserialize)]
pub struct SoftwareDataExtractionConfig {
  #[getset(get = "pub", set = "pub")] allocation: Option<ExtractionInfo<AllocationExtractionInfo>>,
  #[getset(get = "pub", set = "pub")] exceptions: Option<ExtractionInfo<ExceptionExtractionInfo>>,
  #[getset(get = "pub", set = "pub")] http: Option<ExtractionInfo<HTTPExtractionInfo>>,

  #[getset(get = "pub", set = "pub")] method_inlining_success: Option<ExtractionInfo<()>>,
  #[getset(get = "pub", set = "pub")] method_inlining_failed: Option<ExtractionInfo<MethodInliningFailedExtractionInfo>>,

  #[getset(get = "pub", set = "pub")] contention: Option<ExtractionInfo<ContentionExtractionInfo>>,
  #[getset(get = "pub", set = "pub")] socket: Option<ExtractionInfo<SocketExtractionInfo>>,
  #[getset(get = "pub", set = "pub")] thread: Option<ExtractionInfo<ThreadExtractionInfo>>,

  #[getset(get = "pub", set = "pub")] array_pool_array_created: Option<ExtractionInfo<ArrayPoolExtractionInfo>>,
  #[getset(get = "pub", set = "pub")] array_pool_array_rented: Option<ExtractionInfo<ArrayPoolExtractionInfo>>,
  #[getset(get = "pub", set = "pub")] array_pool_array_returned: Option<ExtractionInfo<ArrayPoolExtractionInfo>>,

  #[getset(get = "pub", set = "pub")] array_pool_array_trimmed: Option<ExtractionInfo<ArrayPoolExtractionInfo>>,

  #[getset(get = "pub", set = "pub")] assembly_load: Option<ExtractionInfo<AssemblyExtractionInfo>>,
  #[getset(get = "pub", set = "pub")] assembly_unload: Option<ExtractionInfo<AssemblyExtractionInfo>>,
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
pub struct ExtractionInfo<TConcreteInfo: Clone + Debug> {
  #[getset(get = "pub")] event_class_regex: String,
  #[getset(get = "pub")] info: TConcreteInfo,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct AllocationExtractionInfo {
  #[getset(get = "pub")] type_name_attr: String,
  #[getset(get = "pub")] allocated_count_attr: String,
  #[getset(get = "pub")] object_size_attr: String,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct ExceptionExtractionInfo {
  #[getset(get = "pub")] type_name_attr: String,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct HTTPExtractionInfo {
  #[getset(get = "pub")] host_attr: String,
  #[getset(get = "pub")] port_attr: String,
  #[getset(get = "pub")] scheme_attr: String,
  #[getset(get = "pub")] path_attr: String,
  #[getset(get = "pub")] query_attr: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, new)]
pub struct MethodInliningSuccessExtractionInfo {}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct MethodInliningFailedExtractionInfo {
  #[getset(get = "pub")] reason_attr: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, new)]
pub struct ContentionExtractionInfo {}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct SocketExtractionInfo {
  #[getset(get = "pub")] address_attr: String,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct ThreadExtractionInfo {
  #[getset(get = "pub")] thread_id_attr: String,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct ArrayPoolExtractionInfo {
  #[getset(get = "pub")] buffer_id: String,
}

#[derive(Clone, Debug, Getters, Serialize, Deserialize, new)]
pub struct AssemblyExtractionInfo {
  name_attr: String,
}