use ficus::{
  event_log::{
    core::{
      event::event::{Event, EventPayloadValue},
      event_log::EventLog,
      trace::trace::Trace,
    },
    xes::{xes_event::XesEventImpl, xes_event_log::XesEventLogImpl, xes_trace::XesTraceImpl},
  },
  features::discovery::multithreaded_dfg::dfg::{MultithreadedTracePartsCreationStrategy, discover_multithreaded_dfg},
};
use std::{cell::RefCell, rc::Rc};

const TEST_THREAD_ID_ATTRIBUTE: &'static str = "TestThreadId";

#[test]
fn test_multithreaded_dfg_discovery() {
  execute_multithreaded_dfg_discovery_test(
    vec![vec![
      vec!["A", "B", "C", "D"],
      vec!["A", "B", "C", "D"],
      vec!["A", "B", "C", "D"],
      vec!["A", "B", "C", "D"],
      vec!["A", "B", "C", "D"],
    ]],
    vec!["[A]--[B](5)", "[B]--[C](5)", "[C]--[D](5)"],
  );
}

#[test]
fn test_multithreaded_dfg_discovery_2() {
  execute_multithreaded_dfg_discovery_test(
    vec![vec![
      vec!["E"],
      vec!["A", "B", "C", "D"],
      vec!["A", "B", "C", "D"],
      vec!["A", "B", "C", "D"],
      vec!["A", "B", "C", "D"],
      vec!["A", "B", "C", "D"],
    ]],
    vec!["[A]--[B](5)", "[B]--[C](5)", "[C]--[D](5)", "[E]--[A](5)"],
  );
}

#[test]
fn test_multithreaded_dfg_discovery_3() {
  execute_multithreaded_dfg_discovery_test(
    vec![vec![
      vec!["A", "B", "C", "D"],
      vec!["A", "B", "C", "D"],
      vec!["A", "B", "C", "D"],
      vec!["E"],
      vec!["A", "B", "C", "D"],
      vec!["A", "B", "C", "D"],
    ]],
    vec!["[A]--[B](5)", "[B]--[C](5)", "[C]--[D](5)", "[D]--[E](3)", "[E]--[A](2)"],
  );
}

fn execute_multithreaded_dfg_discovery_test(raw_log: Vec<Vec<Vec<&str>>>, gold: Vec<&str>) {
  let log = create_multithreaded_event_log(raw_log);
  let graph = discover_multithreaded_dfg(&log, TEST_THREAD_ID_ATTRIBUTE, &MultithreadedTracePartsCreationStrategy::Default);

  assert_eq!(gold.join("\n"), graph.serialize_edges_deterministic(true));
}

fn create_multithreaded_event_log(raw_traces: Vec<Vec<Vec<&str>>>) -> XesEventLogImpl {
  let mut log = XesEventLogImpl::default();

  for trace in raw_traces {
    let mut xes_trace = XesTraceImpl::default();
    for (thread, thread_index) in trace.iter().zip(0..trace.len()) {
      for event in thread {
        let mut xes_event = XesEventImpl::new_with_max_date(Rc::from(event.to_string()));
        xes_event.add_or_update_payload(
          Rc::from(TEST_THREAD_ID_ATTRIBUTE.to_string()),
          EventPayloadValue::Uint64(thread_index as u64),
        );

        xes_trace.push(Rc::new(RefCell::new(xes_event)));
      }
    }

    log.push(Rc::new(RefCell::new(xes_trace)));
  }

  log
}
