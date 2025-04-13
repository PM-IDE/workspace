use std::collections::HashMap;
use getset::{Getters, MutGetters};
use crate::features::discovery::timeline::discovery::TraceThread;

#[derive(Clone, Debug, Getters, MutGetters)]
pub struct SoftwareData {
  #[getset(get="pub", get_mut="pub")] event_classes: HashMap<String, usize>,
  #[getset(get="pub", get_mut="pub")] thread_diagram_fragment: Vec<TraceThread>,
  #[getset(get="pub", get_mut="pub")] suspensions: Vec<ExecutionSuspensionEvent>,
  #[getset(get="pub", get_mut="pub")] method_events: Vec<MethodEvent>,
  #[getset(get="pub", get_mut="pub")] thread_events: Vec<ThreadEvent>,
  #[getset(get="pub", get_mut="pub")] http_events: Vec<HTTPEvent>,
  #[getset(get="pub", get_mut="pub")] contention_events: Vec<ContentionEvent>,
  #[getset(get="pub", get_mut="pub")] exception_events: Vec<ExceptionEvent>,
  #[getset(get="pub", get_mut="pub")] pool_events: Vec<ArrayPoolEvent>,
  #[getset(get="pub", get_mut="pub")] socket_events: Vec<SocketEvent>,
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
    }
  }
}

#[derive(Clone, Debug, Getters)]
pub struct ExecutionSuspensionEvent {
  #[getset(get="pub")] start_time: u64,
  #[getset(get="pub")] end_time: u64,
  #[getset(get="pub")] reason: String,
}

impl ExecutionSuspensionEvent {
  pub fn new(start_time: u64, end_time: u64, reason: String) -> Self {
    Self {
      start_time,
      end_time,
      reason,
    }
  }
}

#[derive(Clone, Debug)]
pub enum MethodEvent {
  Success(String),
  Failed(String, String),
  Load(String),
  Unload(String),
}

#[derive(Clone, Debug)]
pub enum ThreadEvent {
  Created(u64),
  Terminated(u64),
}

#[derive(Clone, Debug)]
pub enum AssemblyEvent {
  Load(String),
  Unload(String),
}

#[derive(Clone, Debug, Getters)]
pub struct ArrayPoolEvent {
  #[getset(get="pub")] buffer_id: u64,
  #[getset(get="pub")] event_kind: ArrayPoolEventKind,
}

#[derive(Clone, Debug)]
pub enum ArrayPoolEventKind {
  Created,
  Rented,
  Returned,
  Trimmed,
}

#[derive(Clone, Debug, Getters)]
pub struct ExceptionEvent {
  #[getset(get="pub")] exception_type: String,
}

#[derive(Clone, Debug, Getters)]
pub struct HTTPEvent {
  #[getset(get="pub")] host: String,
  #[getset(get="pub")] port: String,
  #[getset(get="pub")] scheme: String,
  #[getset(get="pub")] path: String,
  #[getset(get="pub")] query: String,
}

impl HTTPEvent {
  pub fn new(host: String, port: String, scheme: String, path: String, query: String) -> Self {
    Self {
      host,
      port,
      scheme,
      path,
      query,
    }
  }
}

#[derive(Clone, Debug, Getters)]
pub struct ContentionEvent {
  #[getset(get="pub")] start_time: u64,
  #[getset(get="pub")] end_time: u64,
}

impl ContentionEvent {
  pub fn new(start_time: u64, end_time: u64) -> Self {
    Self {
      start_time,
      end_time,
    }
  }
}

#[derive(Clone, Debug, Getters)]
pub struct SocketEvent {
  #[getset(get="pub")] address: String,
}