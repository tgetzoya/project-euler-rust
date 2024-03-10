use std::time::{Duration, Instant};

use crate::enums::value::Value;
use crate::utils::numbers::to_digits;

pub fn problem30() -> (Value, Duration) {
    let start = Instant::now();

    let value = (2..1_000_000)
        .filter(|idx| {
            to_digits(*idx).iter().map(|x| (*x as u32).pow(5)).sum::<u32>() == *idx
        }).sum::<u32>();

    assert_eq!(value, 443839);
    return (Value::U32(value), start.elapsed());
}