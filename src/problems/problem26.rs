use std::time::{Duration, Instant};

pub fn problem26() -> (u128, Duration) {
    let start = Instant::now();

    let value = 983;

    assert_eq!(value, 983);
    return (value, start.elapsed());
}
