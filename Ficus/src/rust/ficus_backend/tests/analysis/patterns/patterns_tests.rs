use std::fmt::Debug;

use crate::test_core::simple_events_logs_provider::{
    create_log_for_max_repeats2, create_log_from_taxonomy_of_patterns, create_maximal_repeats_log, create_no_tandem_array_log,
    create_one_tandem_array_log, create_single_trace_test_log1, create_single_trace_test_log2,
};
use ficus_backend::event_log::xes::xes_event_log::XesEventLogImpl;
use ficus_backend::{
    event_log::core::{
        event::{event::Event, event_hasher::NameEventHasher},
        event_log::EventLog,
        trace::trace::Trace,
    },
    features::analysis::patterns::{
        contexts::PatternsDiscoveryStrategy,
        repeats::{find_maximal_repeats, find_near_super_maximal_repeats, find_super_maximal_repeats},
        tandem_arrays::{
            find_maximal_tandem_arrays_with_length, find_primitive_tandem_arrays_with_length, SubArrayInTraceInfo, TandemArrayInfo,
        },
    },
};

#[test]
fn test_tandem_arrays_from_paper() {
    execute_test_with_positions(
        create_log_from_taxonomy_of_patterns,
        |log| get_first_trace_tuples(&find_maximal_tandem_arrays_with_length(log, 10).borrow()),
        &[(2, 3, 4), (3, 3, 4), (4, 3, 3), (2, 6, 2), (3, 6, 2)],
    );
}

fn execute_test_with_positions<TLogCreator, TPatternsFinder, TValue>(
    log_creator: TLogCreator,
    patterns_finder: TPatternsFinder,
    expected: &[TValue],
) where
    TLogCreator: Fn() -> XesEventLogImpl,
    TPatternsFinder: Fn(&Vec<Vec<u64>>) -> Vec<TValue>,
    TValue: PartialEq + Debug,
{
    let log = log_creator();
    let hashes = log.to_hashes_event_log(&NameEventHasher::new());
    let patterns = patterns_finder(&hashes);

    assert_eq!(patterns.as_slice(), expected);
}

#[test]
fn test_tandem_arrays_from_paper_string() {
    execute_test_with_string_dump(
        create_log_from_taxonomy_of_patterns,
        |log| to_sub_arrays(&find_maximal_tandem_arrays_with_length(log, 10).borrow()),
        &["abc", "abcabc", "bca", "bcabca", "cab"],
    );
}

fn execute_test_with_string_dump<TLogCreator, TPatternsFinder>(
    log_creator: TLogCreator,
    patterns_finder: TPatternsFinder,
    expected: &[&str],
) where
    TLogCreator: Fn() -> XesEventLogImpl,
    TPatternsFinder: Fn(&Vec<Vec<u64>>) -> Vec<Vec<SubArrayInTraceInfo>>,
{
    let log = log_creator();
    let hashes = log.to_hashes_event_log(&NameEventHasher::new());
    let patterns = patterns_finder(&hashes);

    let mut dump = dump_repeats_to_string(&patterns, &log);
    dump.sort();

    assert_eq!(dump, expected);
}

fn to_sub_arrays(arrays: &Vec<Vec<TandemArrayInfo>>) -> Vec<Vec<SubArrayInTraceInfo>> {
    arrays
        .iter()
        .map(|trace_arrays| trace_arrays.iter().map(|arr| *arr.get_sub_array_info()).collect())
        .collect()
}

fn get_first_trace_tuples(tandem_arrays: &Vec<Vec<TandemArrayInfo>>) -> Vec<(usize, usize, usize)> {
    tandem_arrays[0].iter().map(|array| array.dump()).collect()
}

#[test]
fn test_no_tandem_arrays() {
    let log = create_no_tandem_array_log();
    let hashes = log.to_hashes_event_log(&NameEventHasher::new());
    let tandem_arrays = find_maximal_tandem_arrays_with_length(&hashes, 10);

    assert_eq!(get_first_trace_tuples(&tandem_arrays.borrow()), []);
}

#[test]
fn test_no_tandem_arrays_string() {
    execute_test_with_string_dump(
        create_no_tandem_array_log,
        |log| to_sub_arrays(&find_maximal_tandem_arrays_with_length(log, 10).borrow()),
        Vec::<&str>::new().as_slice(),
    );
}

#[test]
fn test_one_tandem_array() {
    execute_test_with_positions(
        create_one_tandem_array_log,
        |log| get_first_trace_tuples(&find_maximal_tandem_arrays_with_length(log, 10).borrow()),
        &[(0, 2, 2)],
    );
}

#[test]
fn test_one_tandem_array_string() {
    execute_test_with_string_dump(
        create_one_tandem_array_log,
        |log| to_sub_arrays(&find_maximal_tandem_arrays_with_length(log, 10).borrow()),
        &["ab"],
    );
}

#[test]
fn test_tandem_arrays2() {
    execute_test_with_positions(
        create_log_for_max_repeats2,
        |log| get_first_trace_tuples(&find_primitive_tandem_arrays_with_length(log, 10).borrow()),
        &[(0, 4, 2)],
    );
}

#[test]
fn test_tandem_arrays2_string() {
    let log = create_log_for_max_repeats2();
    let hashes = log.to_hashes_event_log(&NameEventHasher::new());

    let tandem_arrays = find_primitive_tandem_arrays_with_length(&hashes, 10);

    assert_eq!(dump_repeats_to_string(&to_sub_arrays(&tandem_arrays.borrow()), &log), ["dabc"]);
}

#[test]
fn test_maximal_repeats_single_merged_trace1() {
    execute_test_with_positions(
        create_single_trace_test_log1,
        |log| dump_repeats(&find_maximal_repeats(log, &PatternsDiscoveryStrategy::FromSingleMergedTrace)),
        &[(0, 0, 3)],
    );
}

#[test]
fn test_maximal_repeats_single_merged_trace1_string() {
    execute_test_with_string_dump(
        create_single_trace_test_log1,
        |log| find_maximal_repeats(&log, &PatternsDiscoveryStrategy::FromSingleMergedTrace).clone(),
        &["abc"],
    );
}

#[test]
fn test_maximal_repeats_single_merged_trace2() {
    execute_test_with_positions(
        create_single_trace_test_log2,
        |log| dump_repeats(&find_maximal_repeats(log, &PatternsDiscoveryStrategy::FromSingleMergedTrace)),
        &[(0, 3, 6)],
    );
}

#[test]
fn test_maximal_repeats_single_merged_trace2_string() {
    execute_test_with_string_dump(
        create_single_trace_test_log2,
        |log| find_maximal_repeats(&log, &PatternsDiscoveryStrategy::FromSingleMergedTrace).clone(),
        &["abc"],
    );
}

#[test]
fn test_maximal_repeats_single_merged_trace3() {
    execute_test_with_positions(
        create_maximal_repeats_log,
        |log| dump_repeats(&find_maximal_repeats(log, &PatternsDiscoveryStrategy::FromSingleMergedTrace)),
        &[
            (0, 0, 1),
            (0, 0, 2),
            (0, 1, 3),
            (0, 1, 4),
            (0, 1, 5),
            (0, 2, 7),
            (0, 3, 5),
            (0, 4, 5),
            (0, 4, 6),
            (0, 5, 7),
            (0, 5, 8),
            (0, 5, 9),
            (0, 6, 7),
            (0, 6, 8),
            (0, 6, 10),
            (0, 7, 8),
            (0, 8, 10),
            (1, 0, 3),
            (1, 0, 4),
            (1, 7, 9),
            (2, 0, 4),
            (2, 6, 10),
            (2, 7, 10),
            (2, 8, 10),
            (3, 0, 3),
            (3, 2, 4),
            (4, 3, 6),
            (4, 4, 6),
            (4, 9, 10),
            (4, 17, 19),
        ],
    );
}

#[test]
fn test_maximal_repeats_single_merged_trace3_string() {
    execute_test_with_string_dump(
        create_maximal_repeats_log,
        |log| find_maximal_repeats(&log, &PatternsDiscoveryStrategy::FromSingleMergedTrace).clone(),
        &[
            "a", "aa", "aaa", "ab", "abc", "abcd", "ad", "b", "bb", "bbbc", "bbc", "bbcc", "bbcd", "bc", "bcc", "bcda", "bcdbb", "bd", "c",
            "cb", "cc", "cd", "cdc", "d", "da", "dab", "dabc", "db", "dc", "e",
        ],
    );
}

#[test]
fn test_super_maximal_repeats_single_merged_trace() {
    execute_test_with_positions(
        create_maximal_repeats_log,
        |log| dump_repeats(&find_super_maximal_repeats(log, &PatternsDiscoveryStrategy::FromSingleMergedTrace)),
        &[
            (0, 1, 5),
            (0, 2, 7),
            (0, 5, 9),
            (0, 6, 10),
            (1, 0, 4),
            (1, 7, 9),
            (2, 0, 4),
            (2, 6, 10),
            (3, 0, 3),
            (3, 2, 4),
            (4, 3, 6),
            (4, 9, 10),
            (4, 17, 19),
        ],
    );
}

#[test]
fn test_super_maximal_repeats_single_merged_trace_string() {
    execute_test_with_string_dump(
        create_maximal_repeats_log,
        |log| find_super_maximal_repeats(&log, &PatternsDiscoveryStrategy::FromSingleMergedTrace).clone(),
        &[
            "aaa", "abcd", "ad", "bbbc", "bbcc", "bbcd", "bcda", "bcdbb", "bd", "cb", "cdc", "dabc", "e",
        ],
    );
}

#[test]
fn test_near_super_maximal_repeats_single_merged_trace() {
    execute_test_with_positions(
        create_maximal_repeats_log,
        |log| {
            dump_repeats(&find_near_super_maximal_repeats(
                log,
                &PatternsDiscoveryStrategy::FromSingleMergedTrace,
            ))
        },
        &[
            (0, 0, 2),
            (0, 1, 5),
            (0, 2, 7),
            (0, 4, 6),
            (0, 5, 7),
            (0, 5, 9),
            (0, 6, 10),
            (1, 0, 3),
            (1, 0, 4),
            (1, 7, 9),
            (2, 0, 4),
            (2, 6, 10),
            (2, 7, 10),
            (2, 8, 10),
            (3, 2, 4),
            (4, 3, 6),
            (4, 4, 6),
            (4, 9, 10),
            (4, 17, 19),
        ],
    );
}

#[test]
fn test_near_super_maximal_repeats_single_merged_trace_string() {
    execute_test_with_string_dump(
        create_maximal_repeats_log,
        |log| find_near_super_maximal_repeats(&log, &PatternsDiscoveryStrategy::FromSingleMergedTrace).clone(),
        &[
            "aa", "abcd", "ad", "bb", "bbbc", "bbcc", "bbcd", "bcc", "bcda", "bcdbb", "bd", "cb", "cc", "cdc", "dab", "dabc", "db", "dc",
            "e",
        ],
    );
}

fn dump_repeats(repeats: &Vec<Vec<SubArrayInTraceInfo>>) -> Vec<(usize, usize, usize)> {
    let mut result = vec![];
    let mut index = 0;

    for trace_repeats in repeats {
        for repeat in trace_repeats {
            result.push((index, repeat.start_index, repeat.start_index + repeat.length));
        }

        index += 1;
    }

    result
}

fn dump_repeats_to_string(repeats: &Vec<Vec<SubArrayInTraceInfo>>, log: &XesEventLogImpl) -> Vec<String> {
    let mut result = vec![];
    let mut index = 0;

    for trace_repeats in repeats {
        for repeat in trace_repeats {
            let trace = log.traces().get(index).unwrap().borrow();
            let events = trace.events();
            let mut string = String::new();

            for event in &events[repeat.start_index..(repeat.start_index + repeat.length)] {
                string.push_str(event.borrow().name());
            }

            result.push(string);
        }

        index += 1;
    }

    result
}
