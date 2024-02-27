use std::time::{Duration, Instant};
use rug::Integer;
use rug::ops::Pow;
use crate::enums::value::Value;

pub fn problem16() -> (Value, Duration) {
    let start = Instant::now();
    let mut value= 0u32;

    Integer::from(2)
        .pow(1000)
        .to_string()
        .chars()
        .for_each(|c| value += c.to_digit(10).unwrap());

    assert_eq!(value, 1366);
    return (Value::U16(value as u16), start.elapsed());
}