use std::time::{Duration, Instant};

use crate::enums::value::Value;

pub fn problem7() -> (Value, Duration) {
    let start = Instant::now();

    let mut value: u32 = 0;

    /* 2 is a prime number, and it's already accounted for. */
    let mut step = 1;

    let mut primes: Vec<u32> = Vec::new();
    primes.push(2);

    for idx in (3..u32::MAX).step_by(2) {
        if primes.iter().find(|&&y| idx % y == 0).is_none() {
            step += 1;
            primes.push(idx);

            if step == 10_001 {
                value = idx;
                break;
            }
        }
    }

    assert_eq!(value, 104743);
    return (Value::U32(value), start.elapsed());
}