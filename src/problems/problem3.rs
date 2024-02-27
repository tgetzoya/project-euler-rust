use std::time::{Duration, Instant};

use crate::enums::value::Value;
use crate::utils;
use crate::utils::primes::is_prime;

pub fn problem3() -> (Value, Duration) {
    let start = Instant::now();

    let largest_prime = utils::factors::get_factors(600851475143, false)
        .into_iter()
        .filter(|&x| is_prime(x))
        .last()
        .unwrap();

    assert_eq!(largest_prime, 6857);
    return (Value::U16(largest_prime as u16), start.elapsed());
}