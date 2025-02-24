use chrono::format::Numeric::Second;
use chrono::{DateTime, Days, Duration, SecondsFormat, Utc};
use ficus::event_log::core::event::event::Event;
use ficus::event_log::core::event_log::EventLog;
use ficus::event_log::core::trace::trace::Trace;
use ficus::event_log::xes::xes_event_log::XesEventLogImpl;
use rand::distributions::Alphanumeric;
use rand::{Rng, RngCore};
use std::iter;
use std::ops::Add;

pub fn create_raw_event_log() -> Vec<Vec<&'static str>> {
  vec![vec!["A", "B", "C"], vec!["A", "B", "C"]]
}

pub fn create_simple_event_log() -> XesEventLogImpl {
  ficus::event_log::xes::simple::create_simple_event_log(&create_raw_event_log())
}

pub fn create_raw_event_log2() -> Vec<Vec<&'static str>> {
  vec![
    vec!["A", "B", "C", "D", "E"],
    vec!["B", "C", "E", "A", "A", "A"],
    vec!["A", "E", "C", "B", "B", "B", "E", "A"],
    vec!["A", "B", "C", "C", "A"],
    vec!["B", "C", "E", "A", "A", "A"],
  ]
}

pub fn create_simple_event_log2() -> XesEventLogImpl {
  ficus::event_log::xes::simple::create_simple_event_log(&create_raw_event_log2())
}

pub fn create_raw_event_log3() -> Vec<Vec<&'static str>> {
  vec![
    vec!["A", "B", "C", "D", "E"],
    vec!["B", "C", "E", "A", "A", "A"],
    vec!["A", "E", "C", "B", "B", "B", "E", "A"],
    vec!["A", "B", "C", "C", "A"],
    vec!["B", "C", "E", "A", "A", "A"],
    vec!["A", "B", "C", "D", "E"],
    vec!["A", "B", "C", "C", "A"],
    vec!["A", "B", "C", "C", "A"],
    vec!["A", "E", "C", "B", "B", "B", "E", "A"],
  ]
}

pub fn create_simple_event_log3() -> XesEventLogImpl {
  ficus::event_log::xes::simple::create_simple_event_log(&create_raw_event_log3())
}

pub fn create_log_from_filter_out_chaotic_events() -> XesEventLogImpl {
  let mut raw_log = vec![];

  for _ in 0..10 {
    raw_log.push(vec!["a", "b", "c", "x"]);
    raw_log.push(vec!["a", "b", "x", "c"]);
    raw_log.push(vec!["a", "x", "b", "c"]);
  }

  ficus::event_log::xes::simple::create_simple_event_log(&raw_log)
}

pub fn create_log_from_filter_out_chaotic_events_with_noise() -> XesEventLogImpl {
  let mut raw_log = vec![];

  for _ in 0..10 {
    raw_log.push(vec!["d", "v", "d", "d", "a", "d", "b", "c", "x", "d", "d", "d", "d", "d"]);
    raw_log.push(vec!["a", "d", "d", "d", "d", "b", "d", "x", "c", "d"]);
    raw_log.push(vec!["d", "d", "d", "v", "d", "a", "x", "b", "c", "d"]);
  }

  ficus::event_log::xes::simple::create_simple_event_log(&raw_log)
}

pub fn create_log_from_taxonomy_of_patterns() -> XesEventLogImpl {
  let raw_log = vec![vec![
    "g", "d", "a", "b", "c", "a", "b", "c", "a", "b", "c", "a", "b", "c", "a", "f", "i", "c", "a",
  ]];

  ficus::event_log::xes::simple::create_simple_event_log(&raw_log)
}

pub fn create_no_tandem_array_log() -> XesEventLogImpl {
  let raw_log = vec![vec!["a", "b", "c", "d"]];
  ficus::event_log::xes::simple::create_simple_event_log(&raw_log)
}

pub fn create_one_tandem_array_log() -> XesEventLogImpl {
  let raw_log = vec![vec!["a", "b", "a", "b", "c", "d"]];
  ficus::event_log::xes::simple::create_simple_event_log(&raw_log)
}

pub fn create_max_repeats_trace_1() -> &'static [u8] {
  "aabcdbbcda".as_bytes()
}

pub fn create_max_repeats_trace_2() -> &'static [u8] {
  "dabcdabcbb".as_bytes()
}

pub fn create_max_repeats_trace_3() -> &'static [u8] {
  "bbbcdbbbccaa".as_bytes()
}

pub fn create_max_repeats_trace_4() -> &'static [u8] {
  "aaadabbccc".as_bytes()
}

pub fn create_max_repeats_trace_5() -> &'static [u8] {
  "aaacdcdcbedbccbadbdebdc".as_bytes()
}

fn generate_string(len: usize) -> String {
  const CHARSET: &[u8] = b"abcdefgtyu";
  let mut rng = rand::thread_rng();
  let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
  iter::repeat_with(one_char).take(len).collect()
}

pub fn create_long_repeats_trace() -> String {
  generate_string(1_000_000)
}

pub fn create_log_for_max_repeats1() -> XesEventLogImpl {
  ficus::event_log::xes::simple::create_simple_event_log(&vec![vec!["a", "a", "b", "c", "d", "b", "b", "c", "d", "a"]])
}

pub fn create_log_for_max_repeats2() -> XesEventLogImpl {
  ficus::event_log::xes::simple::create_simple_event_log(&vec![vec!["d", "a", "b", "c", "d", "a", "b", "c", "b", "b"]])
}

pub fn create_log_for_max_repeats3() -> XesEventLogImpl {
  ficus::event_log::xes::simple::create_simple_event_log(&vec![vec!["b", "b", "b", "c", "d", "b", "b", "b", "c", "c", "a", "a"]])
}

pub fn create_log_for_max_repeats4() -> XesEventLogImpl {
  ficus::event_log::xes::simple::create_simple_event_log(&vec![vec!["a", "a", "a", "d", "a", "b", "b", "c", "c", "c"]])
}

pub fn create_log_for_max_repeats5() -> XesEventLogImpl {
  ficus::event_log::xes::simple::create_simple_event_log(&vec![vec![
    "a", "a", "a", "c", "d", "c", "d", "c", "b", "e", "d", "b", "c", "c", "b", "a", "d", "b", "d", "e", "b", "d", "c",
  ]])
}

pub fn create_maximal_repeats_log() -> XesEventLogImpl {
  ficus::event_log::xes::simple::create_simple_event_log(&vec![
    vec!["a", "a", "b", "c", "d", "b", "b", "c", "d", "a"],
    vec!["d", "a", "b", "c", "d", "a", "b", "c", "b", "b"],
    vec!["b", "b", "b", "c", "d", "b", "b", "b", "c", "c", "a", "a"],
    vec!["a", "a", "a", "d", "a", "b", "b", "c", "c", "c"],
    vec![
      "a", "a", "a", "c", "d", "c", "d", "c", "b", "e", "d", "b", "c", "c", "b", "a", "d", "b", "d", "e", "b", "d", "c",
    ],
  ])
}

pub fn create_single_trace_test_log1() -> XesEventLogImpl {
  ficus::event_log::xes::simple::create_simple_event_log(&vec![vec!["a", "b", "c", "x", "y", "z"], vec!["r", "t", "u", "a", "b", "c"]])
}

pub fn create_single_trace_test_log2() -> XesEventLogImpl {
  ficus::event_log::xes::simple::create_simple_event_log(&vec![vec!["x", "y", "z", "a", "b", "c"], vec!["a", "b", "c", "r", "t", "u"]])
}

pub fn create_alpha_sharp_test_log() -> XesEventLogImpl {
  ficus::event_log::xes::simple::create_simple_event_log(&vec![
    vec!["A", "C", "D", "D", "F", "G", "H", "I"],
    vec!["B", "C", "E", "E", "F", "H", "G", "I"],
    vec!["A", "D", "E", "D", "E", "G", "H", "I"],
    vec!["A", "E", "D", "G", "H", "I"],
    vec!["B", "E", "D", "H", "G", "I"],
    vec!["B", "D", "E", "H", "G", "I"],
  ])
}

pub fn create_alpha_sharp_test_log2() -> XesEventLogImpl {
  ficus::event_log::xes::simple::create_simple_event_log(&vec![vec!["A", "C", "C", "D"], vec!["A", "B", "C", "B", "C", "D"]])
}

pub fn create_alpha_sharp_test_log3() -> XesEventLogImpl {
  ficus::event_log::xes::simple::create_simple_event_log(&vec![
    vec!["A", "D"],
    vec!["A", "B", "D"],
    vec!["A", "C", "D"],
    vec!["A", "B", "C", "D"],
  ])
}

pub fn create_alpha_plus_plus_nfc_test_log() -> XesEventLogImpl {
  ficus::event_log::xes::simple::create_simple_event_log(&vec![
    vec!["A", "B", "E", "G"],
    vec!["A", "C", "F", "G"],
    vec!["A", "B", "D", "D", "E", "G"],
    vec!["A", "C", "D", "F", "G"],
    vec!["A", "C", "D", "D", "E", "G"],
    vec!["A", "C", "D", "F", "G"],
  ])
}

pub fn create_alpha_plus_plus_nfc_test_log2() -> XesEventLogImpl {
  ficus::event_log::xes::simple::create_simple_event_log(&vec![
    vec!["A", "B", "C"],
    vec!["A", "B", "D", "E", "C"],
    vec!["A", "D", "B", "E", "C"],
    vec!["A", "D", "E", "B", "C"],
    vec!["A", "B", "D", "E", "D", "E", "C"],
  ])
}

pub fn create_alpha_plus_plus_nfc_test_log3() -> XesEventLogImpl {
  ficus::event_log::xes::simple::create_simple_event_log(&vec![
    vec!["A", "C", "D"],
    vec!["B", "C", "E"],
    vec!["A", "F", "C", "E"],
    vec!["A", "C", "F", "E"],
  ])
}

pub fn create_alpha_plus_plus_nfc_test_log4() -> XesEventLogImpl {
  ficus::event_log::xes::simple::create_simple_event_log(&vec![
    vec!["A", "C", "F", "B", "G", "E"],
    vec!["A", "F", "C", "B", "G", "E"],
    vec!["A", "F", "B", "C", "G", "E"],
    vec!["A", "F", "B", "G", "C", "E"],
    vec!["A", "F", "D", "G", "E"],
  ])
}

pub fn create_alpha_plus_plus_nfc_test_log5() -> XesEventLogImpl {
  ficus::event_log::xes::simple::create_simple_event_log(&vec![vec!["A", "B", "C"], vec!["A", "B", "D", "E"], vec!["A", "D", "B", "E"]])
}

pub fn create_alpha_plus_plus_nfc_test_log6() -> XesEventLogImpl {
  ficus::event_log::xes::simple::create_simple_event_log(&vec![vec!["A", "C", "D"], vec!["B", "C", "E"]])
}

pub fn create_alpha_plus_plus_nfc_test_log7() -> XesEventLogImpl {
  ficus::event_log::xes::simple::create_simple_event_log(&vec![
    vec!["A", "C", "F", "D"],
    vec!["A", "F", "C", "D"],
    vec!["B", "C", "G", "E"],
    vec!["B", "G", "C", "E"],
  ])
}

pub fn create_alpha_plus_plus_nfc_test_log8() -> XesEventLogImpl {
  ficus::event_log::xes::simple::create_simple_event_log(&vec![vec!["A", "C", "D"], vec!["B", "C", "F", "E"], vec!["B", "F", "C", "E"]])
}

pub fn create_alpha_plus_plus_nfc_test_log9() -> XesEventLogImpl {
  ficus::event_log::xes::simple::create_simple_event_log(&vec![vec!["A", "C", "E", "B", "C", "D"]])
}

pub fn create_heuristic_miner_replay_test_log() -> XesEventLogImpl {
  ficus::event_log::xes::simple::create_simple_event_log(&vec![vec!["A", "B", "C", "D"], vec!["A", "C", "B", "D"]])
}

pub fn create_alpha_plus_miner_replay_test_log() -> XesEventLogImpl {
  ficus::event_log::xes::simple::create_simple_event_log(&vec![vec!["A", "B", "C", "D"], vec!["A", "C", "B", "D"], vec!["E", "F"]])
}

pub fn create_cases_discovery_test_log() -> XesEventLogImpl {
  ficus::event_log::xes::simple::create_simple_event_log(&vec![
    vec!["S", "b", "a", "d", "E"],
    vec!["S", "E"],
    vec!["S"],
    vec!["E"],
    vec!["S", "a", "b", "S", "E", "a", "E"],
    vec!["E", "S"],
  ])
}

pub fn create_event_log_with_simple_real_time() -> XesEventLogImpl {
  let mut log = create_simple_event_log2();

  annotate_log_with_real_time(&mut log, DateTime::<Utc>::MIN_UTC, Duration::nanoseconds(1));

  log
}

pub fn annotate_log_with_real_time(log: &mut XesEventLogImpl, start_time: DateTime<Utc>, delta: Duration) {
  for trace in log.traces() {
    let trace = trace.borrow();

    let mut current_stamp = start_time.clone();
    for event in trace.events() {
      let mut event = event.borrow_mut();

      event.set_timestamp(current_stamp);

      current_stamp = current_stamp.add(delta);
    }
  }
}
