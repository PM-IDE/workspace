use ficus::features::streaming::counters::core::{StreamingCounter, ValueUpdateKind};
use ficus::features::streaming::counters::lossy_count::LossyCount;
use std::fmt::Debug;
use std::hash::Hash;

#[test]
pub fn lossy_count_test_1() {
    execute_streaming_counter_test(
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        (0..10).into_iter().map(|x| (x + 1, 1)).collect(),
        || LossyCount::<i32, Option<bool>>::new(0.01),
    );
}

#[test]
pub fn lossy_count_test_2() {
    execute_streaming_counter_test(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], vec![(9, 1), (10, 1), (11, 1)], || {
        LossyCount::<i32, Option<bool>>::new(0.25)
    });
}

#[test]
pub fn lossy_count_test_3() {
    execute_streaming_counter_test(
        vec![
            1, 1, 2, 3, 1, 2, 1, 2, 2, 1, 1, 2, 2, 1, 1, 2, 1, 2, 2, 2, 2, 1, 1, 1, 1, 1, 2, 2, 1, 2, 1,
        ],
        vec![(1, 16), (2, 14)],
        || LossyCount::<i32, Option<bool>>::new(0.1),
    )
}

fn execute_streaming_counter_test<TKey: Hash + Eq + Clone + Ord + Debug, TValue: Clone, TCounter: StreamingCounter<TKey, TValue>>(
    sequence: Vec<TKey>,
    expected_result: Vec<(TKey, u64)>,
    counter_factory: impl Fn() -> TCounter,
) {
    let mut counter = counter_factory();

    for value in sequence {
        counter.observe(value, ValueUpdateKind::DoNothing);
    }

    let mut frequencies = counter.all_frequencies();
    frequencies.sort_by(|first, second| first.key().cmp(second.key()));

    assert_eq!(
        expected_result
            .into_iter()
            .map(|expected_freq| (expected_freq.0, expected_freq.1))
            .collect::<Vec<(TKey, u64)>>(),
        frequencies
            .iter()
            .map(|e| (e.key().clone(), e.absolute_count()))
            .collect::<Vec<(TKey, u64)>>()
    );
}
