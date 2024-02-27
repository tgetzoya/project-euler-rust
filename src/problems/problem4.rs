use std::time::{Duration, Instant};

use crate::enums::value::Value;
use crate::utils;

pub fn problem4() -> (Value, Duration) {
    let start = Instant::now();

    let mut value = u32::MIN;
    let start_value = 999*999;
    let end_value = 100*100;

    'main_loop: for idx in (end_value..start_value).rev().filter(|&x| utils::numbers::is_palindrome(x)) {
        let factors = utils::factors::get_factors(idx, false)
            .into_iter()
            .filter(|&x| x > 99 && x < 1000)
            .collect::<Vec<u128>>();

        if factors.len() < 2 {
            continue;
        }

        for jdx in factors.iter() {
            for kdx in factors.iter() {
                if jdx * kdx == idx {
                    value = idx as u32;
                    break 'main_loop;
                }
            }
        }
    }

    assert_eq!(value, 906609);
    return (Value::U32(value), start.elapsed());
}