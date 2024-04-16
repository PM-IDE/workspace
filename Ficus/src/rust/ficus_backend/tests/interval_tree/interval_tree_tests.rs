use ficus_backend::utils::interval_tree::{interval::Interval, interval_tree::IntervalTree};

#[test]
fn interval_tree_test() {
    let intervals = vec![
        Interval::<i32, usize>::new(1, 4),
        Interval::new(5, 6),
        Interval::new(9, 10),
        Interval::new(2, 3),
        Interval::new(-1, 3),
        Interval::new(-5, 10),
    ];

    let mut tree = IntervalTree::new(&intervals, |left, right| *left..*right);

    assert_eq!(tree.search_overlaps_for_point(5), [Interval::new(-5, 10), Interval::new(5, 6)]);

    assert_eq!(
        tree.search_overlaps_for_point(2),
        [
            Interval::new(-5, 10),
            Interval::new(-1, 3),
            Interval::new(1, 4),
            Interval::new(2, 3),
        ]
    );

    assert_eq!(
        tree.search_overlaps_for_interval(1, 3),
        [
            Interval::new(-5, 10),
            Interval::new(-1, 3),
            Interval::new(1, 4),
            Interval::new(2, 3),
        ]
    );

    assert_eq!(
        tree.search_overlaps_for_interval(1, 10),
        [
            Interval::new(-5, 10),
            Interval::new(-1, 3),
            Interval::new(1, 4),
            Interval::new(2, 3),
            Interval::new(5, 6),
            Interval::new(9, 10),
        ]
    );

    assert_eq!(tree.search_envelopes(1, 4), [Interval::new(1, 4), Interval::new(2, 3),]);
}
