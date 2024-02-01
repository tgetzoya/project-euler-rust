use std::time::{Duration, Instant};
use rug::Integer;

use crate::utils::numbers;

pub fn problem20() -> (u128, Duration) {
    let start = Instant::now();

    let ten = Integer::from(10);

    let mut value: u32 = 0;
    let mut big_ass_number = numbers::factorial(100);

    while big_ass_number > 0 {
        let (quotient, rem) = big_ass_number.div_rem(ten.clone());
        value += rem.to_u32().unwrap();
        big_ass_number = quotient;
    }

    /* The below also works, but it takes longer.
     * value = big_ass_number.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>();
     */

    assert_eq!(value, 648);
    return (value as u128, start.elapsed());
}