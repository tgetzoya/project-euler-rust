use std::time::{Duration, Instant};

use crate::enums::value::Value;

pub fn problem9() -> (Value, Duration) {
    let start = Instant::now();

    let mut value = 31875000;

    for a in 1..999 {
        for b in a+1..999 {
            for c in b+1..999 {
                if a+b+c == 1000 && (a*a) + (b*b) == (c*c) {
                    value = a*b*c;
                    break;
                }
            }
        }
    }

    assert_eq!(value, 31875000);
    return (Value::U32(value), start.elapsed());
}