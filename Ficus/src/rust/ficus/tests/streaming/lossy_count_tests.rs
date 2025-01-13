use ficus::features::streaming::counters::core::StreamingCounter;
use ficus::features::streaming::counters::lossy_count::LossyCount;

#[test]
pub fn simple_test_1() {
    let mut lossy_counter = LossyCount::<i32>::new(0.01);

    let sequence = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for value in sequence {
        lossy_counter.observe(value);
    }

    assert_eq!(
        (0..10).into_iter().map(|_| "0.1".to_string()).collect::<Vec<String>>(),
        lossy_counter.all_frequencies().iter().map(|e| e.approx_frequency().to_string()).collect::<Vec<String>>()
    );
}