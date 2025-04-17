use getset::Getters;
use std::fmt::{Debug, Display, Formatter, Write};
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug)]
pub enum DiscoverRootSequenceGraphError {
  NoArtificialStartEndEvents,
  FailedToReplaySequence,
  NotSingleCandidateForNextNode,
}

#[derive(Clone, Copy)]
pub enum RootSequenceKind {
  FindBest,
  LCS,
  PairwiseLCS,
  Trace,
}

impl FromStr for RootSequenceKind {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "FindBest" => Ok(Self::FindBest),
      "LCS" => Ok(Self::LCS),
      "PairwiseLCS" => Ok(Self::PairwiseLCS),
      "Trace" => Ok(Self::Trace),
      _ => Err(())
    }
  }
}

impl Display for DiscoverRootSequenceGraphError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      DiscoverRootSequenceGraphError::NoArtificialStartEndEvents => f.write_str("All traces in event log must have artificial start-end events"),
      DiscoverRootSequenceGraphError::FailedToReplaySequence => f.write_str("Failed to replay sequence of events on part of a graph"),
      DiscoverRootSequenceGraphError::NotSingleCandidateForNextNode => f.write_str("There were several or zero candidates for next node during replay")
    }
  }
}

#[derive(Clone, Debug, Copy)]
pub struct EventCoordinates {
  trace_id: u64,
  event_index: u64,
}

impl EventCoordinates {
  pub fn new(trace_id: u64, event_index: u64) -> Self {
    Self {
      trace_id,
      event_index,
    }
  }

  pub fn trace_id(&self) -> u64 {
    self.trace_id
  }

  pub fn event_index(&self) -> u64 {
    self.event_index
  }
}

#[derive(Clone, Debug)]
pub struct NodeAdditionalDataContainer<T: Clone + Debug> {
  value: T,
  original_event_coordinates: EventCoordinates,
}

impl<T: Clone + Debug> NodeAdditionalDataContainer<T> {
  pub fn new(value: T, trace_data: EventCoordinates) -> Self {
    Self {
      value,
      original_event_coordinates: trace_data,
    }
  }

  pub fn value_mut(&mut self) -> &mut T {
    &mut self.value
  }

  pub fn value(&self) -> &T {
    &self.value
  }

  pub fn original_event_coordinates(&self) -> &EventCoordinates {
    &self.original_event_coordinates
  }

  pub fn set_new_event_coordinates(&mut self, new_coords: EventCoordinates) {
    self.original_event_coordinates = new_coords;
  }
}

#[derive(Clone, Debug)]
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
  belongs_to_root_sequence: bool,
}

impl CorrespondingTraceData {
  pub fn new(belongs_to_root_sequence: bool) -> Self {
    Self {
      belongs_to_root_sequence,
    }
  }

  pub fn belongs_to_root_sequence(&self) -> bool {
    self.belongs_to_root_sequence
  }

  pub fn set_belongs_to_root_sequence(&mut self, value: bool) { self.belongs_to_root_sequence = value }
}

#[derive(Clone, Debug, Getters)]
pub struct EventWithUniqueId<T: PartialEq + Clone> {
  #[getset(get = "pub")] event: T,
  #[getset(get = "pub")] id: u64,
}

impl<T: PartialEq + Clone> EventWithUniqueId<T> {
  pub fn new(event: T) -> Self {
    static NEXT_ID: AtomicU64 = AtomicU64::new(0);

    Self {
      event,
      id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
    }
  }
}

impl<T: PartialEq + Clone> PartialEq for EventWithUniqueId<T> {
  fn eq(&self, other: &Self) -> bool {
    self.event().eq(other.event())
  }
}