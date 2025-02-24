use std::{cell::RefCell, rc::Rc};

use super::{event_log::EventLog, trace::trace::Trace};

pub struct LogIterator<TLog, TEventProcessor, TResult>
where
  TLog: EventLog,
  TEventProcessor: Fn(&TLog::TEvent) -> TResult,
{
  log: Rc<RefCell<TLog>>,
  event_processor: Rc<TEventProcessor>,
  trace_index: usize,
}

impl<TLog, TEventProcessor, TResult> LogIterator<TLog, TEventProcessor, TResult>
where
  TLog: EventLog,
  TEventProcessor: Fn(&TLog::TEvent) -> TResult,
{
  pub fn new(log: Rc<RefCell<TLog>>, event_processor: Rc<TEventProcessor>) -> Self {
    Self {
      log,
      event_processor,
      trace_index: 0,
    }
  }
}

impl<TLog, TEventProcessor, TResult> Iterator for LogIterator<TLog, TEventProcessor, TResult>
where
  TLog: EventLog,
  TEventProcessor: Fn(&TLog::TEvent) -> TResult,
{
  type Item = TraceIterator<TLog::TTrace, TEventProcessor, TResult>;

  fn next(&mut self) -> Option<Self::Item> {
    let log = self.log.borrow();
    let traces = log.traces();

    if self.trace_index >= traces.len() {
      None
    } else {
      let item = Some(TraceIterator {
        trace: Rc::clone(&traces[self.trace_index]),
        event_processor: Rc::clone(&self.event_processor),
        event_index: 0,
      });

      self.trace_index += 1;

      item
    }
  }
}

pub struct TraceIterator<TTrace, TEventProcessor, TResult>
where
  TTrace: Trace,
  TEventProcessor: Fn(&TTrace::TEvent) -> TResult,
{
  trace: Rc<RefCell<TTrace>>,
  event_processor: Rc<TEventProcessor>,
  event_index: usize,
}

impl<TTrace, TEventProcessor, TResult> Iterator for TraceIterator<TTrace, TEventProcessor, TResult>
where
  TTrace: Trace,
  TEventProcessor: Fn(&TTrace::TEvent) -> TResult,
{
  type Item = TResult;

  fn next(&mut self) -> Option<Self::Item> {
    let trace = self.trace.borrow();
    let events = trace.events();
    if self.event_index >= events.len() {
      None
    } else {
      let item = (&self.event_processor)(&events[self.event_index].borrow());
      self.event_index += 1;

      Some(item)
    }
  }
}

impl<TTrace, TEventProcessor, TResult> TraceIterator<TTrace, TEventProcessor, TResult>
where
  TTrace: Trace,
  TEventProcessor: Fn(&TTrace::TEvent) -> TResult,
{
  pub fn new(trace: Rc<RefCell<TTrace>>, event_processor: Rc<TEventProcessor>) -> Self {
    Self {
      trace,
      event_processor,
      event_index: 0,
    }
  }

  pub fn step_back(&mut self) {
    if self.event_index > 0 {
      self.event_index -= 1;
    }
  }
}
