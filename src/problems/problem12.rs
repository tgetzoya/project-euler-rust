use std::time::{Duration, Instant};
use crate::utils;

pub fn problem12() -> (u128, Duration) {
    let start = Instant::now();

    let mut value = 76576500;

    for idx in 2u32.. {
        let num = calculate_triangle_number(idx);
        let factors = utils::factors::get_factor_count(num as u128, true);

        if factors > 500 {
            value = num;
            break;
        }
    }


    assert_eq!(value, 76576500);
    return (value as u128, start.elapsed());
}

fn calculate_triangle_number(num: u32) -> u32 {
    let mut sum: u32 = num;

    for idx in (1..num).rev() {
        sum += idx;
    }

    sum
}