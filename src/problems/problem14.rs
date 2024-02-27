use std::time::{Duration, Instant};
use crate::enums::value::Value;

pub fn problem14() -> (Value, Duration) {
    let start = Instant::now();

    let mut value =  (0u32, 0u32);

    for idx in 1..1_000_000 {
        let mut step: u64 = idx;
        let mut count: u32 = 0;

        while step != 1 {
            count += 1;

            if step % 2 == 0 {
                step /= 2;
            } else {
                step = (3 * step) + 1;
            }
        }


        if value.0 < count {
            value = (count, idx as u32);
        }
    }

    assert_eq!(value.1, 837799);
    return (Value::U32(value.1), start.elapsed());
}