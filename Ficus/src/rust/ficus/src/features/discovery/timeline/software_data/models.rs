use crate::features::discovery::timeline::discovery::TraceThread;
use getset::{Getters, MutGetters};
use std::collections::HashMap;
use derive_new::new;

#[derive(Clone, Debug, Getters, MutGetters)]
pub struct SoftwareData {
  #[getset(get = "pub", get_mut = "pub")] event_classes: HashMap<String, usize>,
  #[getset(get = "pub", get_mut = "pub")] thread_diagram_fragment: Vec<TraceThread>,
  #[getset(get = "pub", get_mut = "pub")] suspensions: Vec<ExecutionSuspensionEvent>,
  #[getset(get = "pub", get_mut = "pub")] method_events: Vec<MethodEvent>,
  #[getset(get = "pub", get_mut = "pub")] thread_events: Vec<ThreadEvent>,
  #[getset(get = "pub", get_mut = "pub")] http_events: Vec<HTTPEvent>,
  #[getset(get = "pub", get_mut = "pub")] contention_events: Vec<ContentionEvent>,
  #[getset(get = "pub", get_mut = "pub")] exception_events: Vec<ExceptionEvent>,
  #[getset(get = "pub", get_mut = "pub")] pool_events: Vec<ArrayPoolEvent>,
  #[getset(get = "pub", get_mut = "pub")] socket_events: Vec<SocketEvent>,
  #[getset(get = "pub", get_mut = "pub")] allocation_events: Vec<AllocationEvent>,
  #[getset(get = "pub", get_mut = "pub")] assembly_events: Vec<AssemblyEvent>,
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
      method_events: vec![],
      contention_events: vec![],
      pool_events: vec![],
      socket_events: vec![],
      allocation_events: vec![],
      assembly_events: vec![],
    }
  }
}

#[derive(Clone, Debug, Getters, new)]
pub struct AllocationEvent {
  #[getset(get = "pub")] type_name: String,
  #[getset(get = "pub")] objects_count: usize,
  #[getset(get = "pub")] allocated_bytes: usize,
}

#[derive(Clone, Debug, Getters, new)]
pub struct ExecutionSuspensionEvent {
  #[getset(get = "pub")] start_time: u64,
  #[getset(get = "pub")] end_time: u64,
  #[getset(get = "pub")] reason: String,
}

#[derive(Clone, Debug)]
pub enum MethodEvent {
  InliningSuccess(String),
  InliningFailed(String, String),
  Load(String),
  Unload(String),
}

#[derive(Clone, Debug, Getters, new)]
pub struct ThreadEvent {
  #[getset(get = "pub")] thread_id: u64,
  #[getset(get = "pub")] kind: ThreadEventKind
}

#[derive(Clone, Debug)]
pub enum ThreadEventKind {
  Created,
  Terminated,
}

#[derive(Clone, Debug)]
pub enum AssemblyEventKind {
  Load,
  Unload,
}

#[derive(Clone, Debug, Getters, new)]
pub struct AssemblyEvent {
  #[getset(get = "pub")] name: String,
  #[getset(get = "pub")] kind: AssemblyEventKind
}

#[derive(Clone, Debug, Getters, new)]
pub struct ArrayPoolEvent {
  #[getset(get = "pub")] buffer_id: u64,
  #[getset(get = "pub")] event_kind: ArrayPoolEventKind,
}

#[derive(Clone, Debug)]
pub enum ArrayPoolEventKind {
  Created,
  Rented,
  Returned,
  Trimmed,
}

#[derive(Clone, Debug, Getters, new)]
pub struct ExceptionEvent {
  #[getset(get = "pub")] exception_type: String,
}

#[derive(Clone, Debug, Getters, new)]
pub struct HTTPEvent {
  #[getset(get = "pub")] host: String,
  #[getset(get = "pub")] port: String,
  #[getset(get = "pub")] scheme: String,
  #[getset(get = "pub")] path: String,
  #[getset(get = "pub")] query: String,
}

#[derive(Clone, Debug, Getters, new)]
pub struct ContentionEvent {
  #[getset(get = "pub")] start_time: u64,
  #[getset(get = "pub")] end_time: u64,
}

#[derive(Clone, Debug, Getters, new)]
pub struct SocketEvent {
  #[getset(get = "pub")] address: String,
}