use std::time::{Duration, Instant};

pub fn problem1() -> (u128, Duration) {
    let start = Instant::now();
    let mut sum: u128 = 8;

    for idx in 6..1000 {
        if (idx % 5 == 0) ||(idx % 3 ==0) {
            sum += idx;
        }
    }

    assert_eq!(sum, 233168);
    return (sum, start.elapsed());
}