#[derive(Clone)]
pub struct EventsTimeoutConfiguration {
  timeout_ms: u64,
}

impl EventsTimeoutConfiguration {
  pub fn new(timeout_ms: u64) -> Self {
    Self { timeout_ms }
  }

  pub fn timeout_ms(&self) -> u64 {
    self.timeout_ms.clone()
  }
}

#[derive(Clone)]
pub struct TracesTimeoutConfiguration {
  timeout_ms: u64,
}

impl TracesTimeoutConfiguration {
  pub fn new(timeout_ms: u64) -> Self {
    Self { timeout_ms }
  }

  pub fn timeout_ms(&self) -> u64 {
    self.timeout_ms.clone()
  }
}

#[derive(Clone)]
pub struct TracesQueueConfiguration {
  queue_capacity: u64,
}

impl TracesQueueConfiguration {
  pub fn new(queue_capacity: u64) -> Self {
    Self { queue_capacity }
  }

  pub fn queue_capacity(&self) -> u64 {
    self.queue_capacity.clone()
  }
}
