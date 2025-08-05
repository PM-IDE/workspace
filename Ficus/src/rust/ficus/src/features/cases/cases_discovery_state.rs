use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::event_log::xes::xes_trace::XesTraceImpl;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct CasesDiscoveryState {
  log: XesEventLogImpl,
  stack: VecDeque<XesTraceImpl>,
  depth: Option<usize>,
}

impl CasesDiscoveryState {
  pub fn new(inline_nested: bool) -> Self {
    Self {
      log: XesEventLogImpl::empty(),
      stack: VecDeque::new(),
      depth: match inline_nested {
        true => Some(0),
        false => None,
      },
    }
  }

  pub fn handle_start_event(&mut self, event: &XesEventImpl) {
    if let Some(depth_value) = self.depth {
      self.depth = Some(depth_value + 1);

      if depth_value > 0 {
        self
          .stack
          .back_mut()
          .expect("Must contain trace")
          .push(Rc::new(RefCell::new(event.clone())));

        return;
      }
    }

    let mut sub_trace = XesTraceImpl::empty();
    sub_trace.push(Rc::new(RefCell::new(event.clone())));

    self.stack.push_back(sub_trace);
  }

  pub fn handle_end_event(&mut self, event: &XesEventImpl) {
    if let Some(depth_value) = self.depth {
      if depth_value == 0 {
        return;
      }

      self.depth = Some(depth_value - 1);

      if depth_value > 1 {
        self
          .stack
          .back_mut()
          .expect("Must contain trace")
          .push(Rc::new(RefCell::new(event.clone())));

        return;
      }
    }

    match self.stack.pop_back() {
      None => {}
      Some(mut sub_trace) => {
        sub_trace.push(Rc::new(RefCell::new(event.clone())));
        self.log.push(Rc::new(RefCell::new(sub_trace)));
      }
    }
  }

  pub fn handle_default_event(&mut self, event: &XesEventImpl) {
    if let Some(last_trace) = self.stack.back_mut() {
      last_trace.push(Rc::new(RefCell::new(event.clone())));
    }
  }

  pub fn handle_trace_end(&mut self) {
    loop {
      if self.stack.is_empty() {
        break;
      }

      self
        .log
        .push(Rc::new(RefCell::new(self.stack.pop_back().expect("Can not be empty"))));
    }
  }

  pub fn log(self) -> XesEventLogImpl {
    self.log
  }
}
