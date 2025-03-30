#[derive(Clone)]
pub struct ActivityStartEndTimeData {
  start_time: u64,
  end_time: u64,
}

impl ActivityStartEndTimeData {
  pub fn new(start_time: u64, end_time: u64) -> Self {
    Self {
      start_time,
      end_time,
    }
  }

  pub fn start_time(&self) -> u64 {
    self.start_time
  }

  pub fn end_time(&self) -> u64 {
    self.end_time
  }
}

#[derive(Clone, Debug)]
pub struct CorrespondingTraceData {
  trace_id: u64,
  event_index: u64,
  belongs_to_root_sequence: bool,
}

impl CorrespondingTraceData {
  pub fn new(trace_id: u64, event_index: u64, belongs_to_root_sequence: bool) -> Self {
    Self {
      trace_id,
      event_index,
      belongs_to_root_sequence
    }
  }
  
  pub fn belongs_to_root_sequence(&self) -> bool {
    self.belongs_to_root_sequence
  }

  pub fn set_belongs_to_root_sequence(&mut self, value: bool) { self.belongs_to_root_sequence = value }

  pub fn trace_id(&self) -> u64 {
    self.trace_id
  }

  pub fn event_index(&self) -> u64 {
    self.event_index
  }
}