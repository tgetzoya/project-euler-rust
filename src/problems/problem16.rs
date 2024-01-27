use std::time::{Duration, Instant};
use rug::Integer;
use rug::ops::Pow;

pub fn problem16() -> (u128, Duration) {
    let start = Instant::now();
    let mut value= 0u32;

    Integer::from(2)
        .pow(1000)
        .to_string()
        .chars()
        .for_each(|c| value += c.to_digit(10).unwrap());

    assert_eq!(value, 1366);
    return (value as u128, start.elapsed());
}