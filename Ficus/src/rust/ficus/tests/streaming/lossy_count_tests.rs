use ficus::features::streaming::counters::core::StreamingCounter;
use ficus::features::streaming::counters::lossy_count::LossyCount;
use std::fmt::Debug;
use std::hash::Hash;

#[test]
pub fn lossy_count_test_1() {
    execute_streaming_counter_test(
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        (0..10).into_iter().map(|x| (x + 1, 0.1)).collect(),
        || LossyCount::<i32>::new(0.01),
    );
}

#[test]
pub fn lossy_count_test_2() {
    execute_streaming_counter_test(
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        vec![(9, 0.09090909090909091), (10, 0.09090909090909091), (11, 0.09090909090909091)],
        || LossyCount::<i32>::new(0.25),
    );
}

#[test]
pub fn lossy_count_test_3() {
    execute_streaming_counter_test(
        vec![
            1, 1, 2, 3, 1, 2, 1, 2, 2, 1, 1, 2, 2, 1, 1, 2, 1, 2, 2, 2, 2, 1, 1, 1, 1, 1, 2, 2, 1, 2, 1,
        ],
        vec![(1, 0.5161290322580645), (2, 0.45161290322580644)],
        || LossyCount::<i32>::new(0.1),
    )
}

fn execute_streaming_counter_test<T: Hash + Eq + Clone + Ord + Debug, TCounter: StreamingCounter<T>>(
    sequence: Vec<T>,
    expected_result: Vec<(T, f64)>,
    counter_factory: impl Fn() -> TCounter,
) {
    let mut counter = counter_factory();

    for value in sequence {
        counter.observe(value);
    }

    let mut frequencies = counter.all_frequencies();
    frequencies.sort_by(|first, second| first.key().cmp(second.key()));

    assert_eq!(
        expected_result
            .into_iter()
            .map(|expected_freq| (expected_freq.0, expected_freq.1.to_string()))
            .collect::<Vec<(T, String)>>(),
        frequencies
            .iter()
            .map(|e| (e.key().clone(), e.approx_frequency().to_string()))
            .collect::<Vec<(T, String)>>()
    );
}
