use chrono::Utc;
use ficus::features::streaming::counters::sliding_window::SlidingWindow;
use std::ops::{Add, Sub};
use std::time::Duration;
use ficus::features::streaming::counters::core::ValueUpdateKind;

#[test]
pub fn test_timed_window() {
    let mut window = SlidingWindow::new_time(Duration::from_secs(300));

    let now = Utc::now();
    let mut start_time = now.sub(Duration::from_secs(600));
    let delta = Duration::from_secs(60);

    for i in 0..10 {
        window.add(i, ValueUpdateKind::Replace(i), start_time);
        start_time = start_time.add(delta);
    }

    window.invalidate();
    let mut retained = window.all().iter().map(|p| (*p.0, *p.1.unwrap())).collect::<Vec<(i32, i32)>>();
    retained.sort_by(|f, s| f.0.cmp(&s.0));

    assert_eq!(retained, vec![(6, 6), (7, 7), (8, 8), (9, 9)])
}
