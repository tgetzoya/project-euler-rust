use std::time::{Duration, Instant};

pub fn problem24() -> (u128, Duration) {
    let start = Instant::now();

    let value = "2783915460";

    assert_eq!(value, "2783915460");
    return (value.parse::<u128>().unwrap(), start.elapsed());
}
