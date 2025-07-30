use crate::features::discovery::timeline::discovery::TraceThread;
use derive_new::new;
use getset::{Getters, MutGetters};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

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
  suspensions: Vec<ExecutionSuspensionEvent>,

  #[getset(get = "pub", get_mut = "pub")]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  method_inlinings_events: Vec<MethodInliningEvent>,

  #[getset(get = "pub", get_mut = "pub")]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  thread_events: Vec<ThreadEvent>,

  #[getset(get = "pub", get_mut = "pub")]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  http_events: Vec<HTTPEvent>,

  #[getset(get = "pub", get_mut = "pub")]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  contention_events: Vec<ContentionEvent>,

  #[getset(get = "pub", get_mut = "pub")]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  exception_events: Vec<ExceptionEvent>,

  #[getset(get = "pub", get_mut = "pub")]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pool_events: Vec<ArrayPoolEvent>,

  #[getset(get = "pub", get_mut = "pub")]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  socket_events: Vec<SocketEvent>,

  #[getset(get = "pub", get_mut = "pub")]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  allocation_events: Vec<AllocationEvent>,

  #[getset(get = "pub", get_mut = "pub")]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  assembly_events: Vec<AssemblyEvent>,

  #[getset(get = "pub", get_mut = "pub")]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  method_load_unload_events: Vec<MethodLoadUnloadEvent>,

  #[getset(get = "pub", get_mut = "pub")]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  histograms: Vec<HistogramData>,

  #[getset(get = "pub", get_mut = "pub")]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  simple_counters: Vec<SimpleCounterData>,

  #[getset(get = "pub", get_mut = "pub")]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  activities_duration: Vec<ActivityDurationData>,
}

impl SoftwareData {
  pub fn empty() -> Self {
    Self {
      event_classes: HashMap::new(),
      thread_diagram_fragment: vec![],
      suspensions: vec![],
      exception_events: vec![],
      http_events: vec![],
      thread_events: vec![],
      method_inlinings_events: vec![],
      contention_events: vec![],
      pool_events: vec![],
      socket_events: vec![],
      allocation_events: vec![],
      assembly_events: vec![],
      method_load_unload_events: vec![],
      histograms: vec![],
      simple_counters: vec![],
      activities_duration: vec![],
    }
  }
}

#[derive(Clone, Debug, Getters, new, Serialize, Deserialize)]
pub struct AllocationEvent {
  #[getset(get = "pub")] type_name: String,
  #[getset(get = "pub")] objects_count: usize,
  #[getset(get = "pub")] allocated_bytes: usize,
}

#[derive(Clone, Debug, Getters, new, Serialize, Deserialize)]
pub struct ExecutionSuspensionEvent {
  #[getset(get = "pub")] start_time: u64,
  #[getset(get = "pub")] end_time: u64,
  #[getset(get = "pub")] reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MethodInliningEvent {
  InliningSuccess(MethodInliningData),
  InliningFailed(MethodInliningData, String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MethodLoadUnloadEvent {
  Load(MethodNameParts),
  Unload(MethodNameParts),
}

#[derive(Clone, Debug, Getters, new, Serialize, Deserialize)]
pub struct MethodInliningData {
  #[getset(get = "pub")] inlinee_info: MethodNameParts,
  #[getset(get = "pub")] inliner_info: MethodNameParts,
}

#[derive(Clone, Debug, Getters, new, Serialize, Deserialize)]
pub struct MethodNameParts {
  #[getset(get = "pub")] name: String,
  #[getset(get = "pub")] namespace: String,
  #[getset(get = "pub")] signature: String,
}

#[derive(Clone, Debug, Getters, new, Serialize, Deserialize)]
pub struct ThreadEvent {
  #[getset(get = "pub")] thread_id: u64,
  #[getset(get = "pub")] kind: ThreadEventKind,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ThreadEventKind {
  Created,
  Terminated,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AssemblyEventKind {
  Load,
  Unload,
}

#[derive(Clone, Debug, Getters, new, Serialize, Deserialize)]
pub struct AssemblyEvent {
  #[getset(get = "pub")] name: String,
  #[getset(get = "pub")] kind: AssemblyEventKind,
}

#[derive(Clone, Debug, Getters, new, Serialize, Deserialize)]
pub struct ArrayPoolEvent {
  #[getset(get = "pub")] buffer_id: u64,
  #[getset(get = "pub")] buffer_size_bytes: u64,
  #[getset(get = "pub")] event_kind: ArrayPoolEventKind,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ArrayPoolEventKind {
  Created,
  Rented,
  Returned,
  Trimmed,
}

#[derive(Clone, Debug, Getters, new, Serialize, Deserialize)]
pub struct ExceptionEvent {
  #[getset(get = "pub")] exception_type: String,
}

#[derive(Clone, Debug, Getters, new, Serialize, Deserialize)]
pub struct HTTPEvent {
  #[getset(get = "pub")] host: String,
  #[getset(get = "pub")] port: String,
  #[getset(get = "pub")] scheme: String,
  #[getset(get = "pub")] path_and_query: String,
}

#[derive(Clone, Debug, Getters, new, Serialize, Deserialize)]
pub struct ContentionEvent {
  #[getset(get = "pub")] start_time: u64,
  #[getset(get = "pub")] end_time: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SocketEvent {
  ConnectStart(SocketConnectAcceptStartMetadata),
  ConnectStop,
  AcceptStart(SocketConnectAcceptStartMetadata),
  AcceptStop,
  ConnectFailed(SocketConnectAcceptFailedMetadata),
  AcceptFailed(SocketConnectAcceptFailedMetadata),
}

#[derive(Clone, Debug, Getters, new, Serialize, Deserialize)]
pub struct SocketConnectAcceptStartMetadata {
  #[getset(get = "pub")] address: String,
}

#[derive(Clone, Debug, Getters, new, Serialize, Deserialize)]
pub struct SocketConnectAcceptFailedMetadata {
  #[getset(get = "pub")] error_code: String,
  #[getset(get = "pub")] error_message: String,
}

#[derive(Clone, Debug, Getters, MutGetters, new, Serialize, Deserialize)]
pub struct HistogramData {
  #[getset(get = "pub")] name: String,
  #[getset(get = "pub")] units: String,
  #[getset(get = "pub", get_mut = "pub")] entries: Vec<HistogramEntry>,
}

#[derive(Clone, Debug, Getters, new, Serialize, Deserialize)]
pub struct HistogramEntry {
  #[getset(get = "pub")] name: String,
  #[getset(get = "pub")] value: f64,
}

#[derive(Clone, Debug, Getters, MutGetters, new, Serialize, Deserialize)]
pub struct SimpleCounterData {
  #[getset(get = "pub")] name: String,
  #[getset(get = "pub")] value: f64,
  #[getset(get = "pub")] units: String,
}

#[derive(Clone, Debug, Getters, MutGetters, new, Serialize, Deserialize)]
pub struct ActivityDurationData {
  #[getset(get = "pub")] name: String,
  #[getset(get = "pub")] duration: f64,
  #[getset(get = "pub")] units: String,
}