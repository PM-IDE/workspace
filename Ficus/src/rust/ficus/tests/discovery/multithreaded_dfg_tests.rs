use ficus::event_log::core::event::event::{Event, EventPayloadValue};
use ficus::event_log::core::event_log::EventLog;
use ficus::event_log::core::trace::trace::Trace;
use ficus::event_log::xes::xes_event::XesEventImpl;
use ficus::event_log::xes::xes_event_log::XesEventLogImpl;
use ficus::event_log::xes::xes_trace::XesTraceImpl;
use ficus::features::discovery::multithreaded_dfg::dfg::{discover_multithreaded_dfg, MultithreadedTracePartsCreationStrategy};
use std::cell::RefCell;
use std::rc::Rc;

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
  let mut log = XesEventLogImpl::empty();

  for trace in raw_traces {
    let mut xes_trace = XesTraceImpl::empty();
    for (thread, thread_index) in trace.iter().zip(0..trace.len()) {
      for event in thread {
        let mut xes_event = XesEventImpl::new_with_max_date(event.to_string());
        xes_event.add_or_update_payload(TEST_THREAD_ID_ATTRIBUTE.to_string(), EventPayloadValue::Uint64(thread_index as u64));

        xes_trace.push(Rc::new(RefCell::new(xes_event)));
      }
    }

    log.push(Rc::new(RefCell::new(xes_trace)));
  }

  log
}
