use std::time::{Duration, Instant};
use crate::enums::value::Value;

use crate::utils::factors::get_factors;

pub fn problem23() -> (Value, Duration) {
    let start = Instant::now();

    let mut value = 0;
    let abundant_numbers = get_all_abundant_numbers();

    /* This is significantly faster than making a HashSet of summed values. */
    let mut values: Vec<bool> = vec![true; 28124];
    for  x in abundant_numbers.iter() {
        for y in  abundant_numbers.iter(){
            let pos = (x+y) as usize;

            if pos > 28123 {
                break;
            }

            values[pos] = false;
        }
    }

    values
        .iter() // Iterate
        .enumerate() // Include index
        .filter(|(_idx, &x)| x) // Get only elements still set to true
        .for_each(|(idx, _)| value += idx); // Add the index to value

    assert_eq!(value, 4179871);
    return (Value::U16(value as u16), start.elapsed());
}

fn get_all_abundant_numbers () -> Vec<u32> {
    let mut abundant_numbers: Vec<u32> = Vec::new();

    for idx in 1u32..28124 {
        let sum = get_factors(idx as u128, false).iter().sum::<u128>() + 1;

        if sum > idx as u128 {
            abundant_numbers.push(idx);
        }
    }

    abundant_numbers
}