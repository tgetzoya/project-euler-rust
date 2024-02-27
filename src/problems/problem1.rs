use std::time::{Duration, Instant};

use crate::enums::value::Value;

pub fn problem1() -> (Value, Duration) {
    let start = Instant::now();
    let mut sum: u32 = 8;

    for idx in 6..1000 {
        if (idx % 5 == 0) ||(idx % 3 ==0) {
            sum += idx;
        }
    }

    assert_eq!(sum, 233168);
    return (Value::U32(sum), start.elapsed());
}