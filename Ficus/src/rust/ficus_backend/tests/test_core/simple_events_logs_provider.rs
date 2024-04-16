use ficus_backend::event_log::simple::simple_event_log::SimpleEventLog;

pub fn create_raw_event_log() -> Vec<Vec<&'static str>> {
    vec![vec!["A", "B", "C"], vec!["A", "B", "C"]]
}

pub fn create_simple_event_log() -> SimpleEventLog {
    SimpleEventLog::new(&create_raw_event_log())
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

pub fn create_simple_event_log2() -> SimpleEventLog {
    SimpleEventLog::new(&create_raw_event_log2())
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

pub fn create_simple_event_log3() -> SimpleEventLog {
    SimpleEventLog::new(&create_raw_event_log3())
}

pub fn create_log_from_filter_out_chaotic_events() -> SimpleEventLog {
    let mut raw_log = vec![];

    for _ in 0..10 {
        raw_log.push(vec!["a", "b", "c", "x"]);
        raw_log.push(vec!["a", "b", "x", "c"]);
        raw_log.push(vec!["a", "x", "b", "c"]);
    }

    SimpleEventLog::new(&raw_log)
}

pub fn create_log_from_filter_out_chaotic_events_with_noise() -> SimpleEventLog {
    let mut raw_log = vec![];

    for _ in 0..10 {
        raw_log.push(vec!["d", "v", "d", "d", "a", "d", "b", "c", "x", "d", "d", "d", "d", "d"]);
        raw_log.push(vec!["a", "d", "d", "d", "d", "b", "d", "x", "c", "d"]);
        raw_log.push(vec!["d", "d", "d", "v", "d", "a", "x", "b", "c", "d"]);
    }

    SimpleEventLog::new(&raw_log)
}

pub fn create_log_from_taxonomy_of_patterns() -> SimpleEventLog {
    let raw_log = vec![vec![
        "g", "d", "a", "b", "c", "a", "b", "c", "a", "b", "c", "a", "b", "c", "a", "f", "i", "c", "a",
    ]];

    SimpleEventLog::new(&raw_log)
}

pub fn create_no_tandem_array_log() -> SimpleEventLog {
    let raw_log = vec![vec!["a", "b", "c", "d"]];
    SimpleEventLog::new(&raw_log)
}

pub fn create_one_tandem_array_log() -> SimpleEventLog {
    let raw_log = vec![vec!["a", "b", "a", "b", "c", "d"]];
    SimpleEventLog::new(&raw_log)
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

pub fn create_log_for_max_repeats1() -> SimpleEventLog {
    SimpleEventLog::new(&vec![vec!["a", "a", "b", "c", "d", "b", "b", "c", "d", "a"]])
}

pub fn create_log_for_max_repeats2() -> SimpleEventLog {
    SimpleEventLog::new(&vec![vec!["d", "a", "b", "c", "d", "a", "b", "c", "b", "b"]])
}

pub fn create_log_for_max_repeats3() -> SimpleEventLog {
    SimpleEventLog::new(&vec![vec!["b", "b", "b", "c", "d", "b", "b", "b", "c", "c", "a", "a"]])
}

pub fn create_log_for_max_repeats4() -> SimpleEventLog {
    SimpleEventLog::new(&vec![vec!["a", "a", "a", "d", "a", "b", "b", "c", "c", "c"]])
}

pub fn create_log_for_max_repeats5() -> SimpleEventLog {
    SimpleEventLog::new(&vec![vec![
        "a", "a", "a", "c", "d", "c", "d", "c", "b", "e", "d", "b", "c", "c", "b", "a", "d", "b", "d", "e", "b", "d", "c",
    ]])
}

pub fn create_maximal_repeats_log() -> SimpleEventLog {
    SimpleEventLog::new(&vec![
        vec!["a", "a", "b", "c", "d", "b", "b", "c", "d", "a"],
        vec!["d", "a", "b", "c", "d", "a", "b", "c", "b", "b"],
        vec!["b", "b", "b", "c", "d", "b", "b", "b", "c", "c", "a", "a"],
        vec!["a", "a", "a", "d", "a", "b", "b", "c", "c", "c"],
        vec![
            "a", "a", "a", "c", "d", "c", "d", "c", "b", "e", "d", "b", "c", "c", "b", "a", "d", "b", "d", "e", "b", "d", "c",
        ],
    ])
}

pub fn create_single_trace_test_log1() -> SimpleEventLog {
    SimpleEventLog::new(&vec![vec!["a", "b", "c", "x", "y", "z"], vec!["r", "t", "u", "a", "b", "c"]])
}

pub fn create_single_trace_test_log2() -> SimpleEventLog {
    SimpleEventLog::new(&vec![vec!["x", "y", "z", "a", "b", "c"], vec!["a", "b", "c", "r", "t", "u"]])
}

pub fn create_alpha_sharp_test_log() -> SimpleEventLog {
    SimpleEventLog::new(&vec![
        vec!["A", "C", "D", "D", "F", "G", "H", "I"],
        vec!["B", "C", "E", "E", "F", "H", "G", "I"],
        vec!["A", "D", "E", "D", "E", "G", "H", "I"],
        vec!["A", "E", "D", "G", "H", "I"],
        vec!["B", "E", "D", "H", "G", "I"],
        vec!["B", "D", "E", "H", "G", "I"],
    ])
}

pub fn create_alpha_sharp_test_log2() -> SimpleEventLog {
    SimpleEventLog::new(&vec![vec!["A", "C", "C", "D"], vec!["A", "B", "C", "B", "C", "D"]])
}

pub fn create_alpha_sharp_test_log3() -> SimpleEventLog {
    SimpleEventLog::new(&vec![
        vec!["A", "D"],
        vec!["A", "B", "D"],
        vec!["A", "C", "D"],
        vec!["A", "B", "C", "D"],
    ])
}

pub fn create_alpha_plus_plus_nfc_test_log() -> SimpleEventLog {
    SimpleEventLog::new(&vec![
        vec!["A", "B", "E", "G"],
        vec!["A", "C", "F", "G"],
        vec!["A", "B", "D", "D", "E", "G"],
        vec!["A", "C", "D", "F", "G"],
        vec!["A", "C", "D", "D", "E", "G"],
        vec!["A", "C", "D", "F", "G"],
    ])
}

pub fn create_alpha_plus_plus_nfc_test_log2() -> SimpleEventLog {
    SimpleEventLog::new(&vec![
        vec!["A", "B", "C"],
        vec!["A", "B", "D", "E", "C"],
        vec!["A", "D", "B", "E", "C"],
        vec!["A", "D", "E", "B", "C"],
        vec!["A", "B", "D", "E", "D", "E", "C"],
    ])
}

pub fn create_alpha_plus_plus_nfc_test_log3() -> SimpleEventLog {
    SimpleEventLog::new(&vec![
        vec!["A", "C", "D"],
        vec!["B", "C", "E"],
        vec!["A", "F", "C", "E"],
        vec!["A", "C", "F", "E"],
    ])
}

pub fn create_alpha_plus_plus_nfc_test_log4() -> SimpleEventLog {
    SimpleEventLog::new(&vec![
        vec!["A", "C", "F", "B", "G", "E"],
        vec!["A", "F", "C", "B", "G", "E"],
        vec!["A", "F", "B", "C", "G", "E"],
        vec!["A", "F", "B", "G", "C", "E"],
        vec!["A", "F", "D", "G", "E"],
    ])
}

pub fn create_alpha_plus_plus_nfc_test_log5() -> SimpleEventLog {
    SimpleEventLog::new(&vec![vec!["A", "B", "C"], vec!["A", "B", "D", "E"], vec!["A", "D", "B", "E"]])
}

pub fn create_alpha_plus_plus_nfc_test_log6() -> SimpleEventLog {
    SimpleEventLog::new(&vec![vec!["A", "C", "D"], vec!["B", "C", "E"]])
}

pub fn create_alpha_plus_plus_nfc_test_log7() -> SimpleEventLog {
    SimpleEventLog::new(&vec![
        vec!["A", "C", "F", "D"],
        vec!["A", "F", "C", "D"],
        vec!["B", "C", "G", "E"],
        vec!["B", "G", "C", "E"],
    ])
}

pub fn create_alpha_plus_plus_nfc_test_log8() -> SimpleEventLog {
    SimpleEventLog::new(&vec![vec!["A", "C", "D"], vec!["B", "C", "F", "E"], vec!["B", "F", "C", "E"]])
}

pub fn create_alpha_plus_plus_nfc_test_log9() -> SimpleEventLog {
    SimpleEventLog::new(&vec![vec!["A", "C", "E", "B", "C", "D"]])
}

pub fn create_heuristic_miner_replay_test_log() -> SimpleEventLog {
    SimpleEventLog::new(&vec![vec!["A", "B", "C", "D"], vec!["A", "C", "B", "D"]])
}

pub fn create_alpha_plus_miner_replay_test_log() -> SimpleEventLog {
    SimpleEventLog::new(&vec![vec!["A", "B", "C", "D"], vec!["A", "C", "B", "D"], vec!["E", "F"]])
}
