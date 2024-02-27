use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::enums::value::Value;
use crate::utils::factors::get_factors;

pub fn problem21() -> (Value, Duration) {
    let start = Instant::now();
    let mut value: u128 = 0;

    let mut divisor_map: HashMap<u128, u128> = HashMap::with_capacity(10_000);
    let mut idx_divisor_sum: u128;
    let mut other_divisor_sum: u128;

    for idx in 1..10_000 {
        if divisor_map.contains_key(&idx) {
            idx_divisor_sum = *divisor_map.get(&idx).unwrap() as u128;
        } else {
            idx_divisor_sum = get_factors(idx, false).iter().sum::<u128>() + 1;
            divisor_map.insert(idx, idx_divisor_sum);
        }


        if divisor_map.contains_key(&idx_divisor_sum) {
            other_divisor_sum = *divisor_map.get(&idx_divisor_sum).unwrap() as u128;
        } else {
            other_divisor_sum = get_factors(idx_divisor_sum, false).iter().sum::<u128>() + 1;
            divisor_map.insert(idx_divisor_sum, other_divisor_sum);
        }

        if idx != idx_divisor_sum  && other_divisor_sum == idx {
            value += idx;
        }
    }

    assert_eq!(value, 31626);
    return (Value::U16(value as u16), start.elapsed());
}