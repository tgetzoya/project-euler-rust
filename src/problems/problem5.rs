use std::time::{Duration, Instant};
use crate::utils;

pub fn problem5() -> (u128, Duration) {
    let start = Instant::now();

    let value = (11..20).reduce(utils::numbers::lcm).unwrap();

    assert_eq!(value, 232792560);
    return (value, start.elapsed());
}