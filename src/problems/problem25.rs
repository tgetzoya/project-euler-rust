use std::time::{Duration, Instant};

use rug::Integer;
use rug::ops::Pow;

use crate::enums::value::Value;

pub fn problem25() -> (Value, Duration) {
    let start = Instant::now();
    let mut value = 2;

    let mut x = Integer::from(1);
    let mut y = Integer::from(1);
    let one_thousand_digits = Integer::from(10).pow(999);

    while y < one_thousand_digits {
        (x,y) = (y.clone(), x + y);
        value += 1;
    }

    assert_eq!(value, 4782);
    return (Value::U32(value), start.elapsed());
}