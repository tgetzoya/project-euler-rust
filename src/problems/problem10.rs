use std::time::{Duration, Instant};

use crate::enums::value::Value;
use crate::utils;

pub fn problem10() -> (Value, Duration) {
    let start = Instant::now();

    let mut value = 2;


    for val in (3..2_000_000).step_by(2).filter(|&x| utils::primes::is_prime(x as u128)) {
        value += val;
    }

    assert_eq!(value, 142913828922);
    return (Value::U64(value), start.elapsed());
}