use std::time::{Duration, Instant};

use crate::enums::value::Value;
use crate::utils;

pub fn problem5() -> (Value, Duration) {
    let start = Instant::now();

    let value = (11..20).reduce(utils::numbers::lcm).unwrap();

    assert_eq!(value, 232792560);
    return (Value::U32(value as u32), start.elapsed());
}