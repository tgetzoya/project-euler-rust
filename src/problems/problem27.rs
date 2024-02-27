use std::ops::Add;
use std::time::{Duration, Instant};

use rug::Integer;
use rug::integer::IsPrime;
use rug::ops::Pow;
use crate::enums::value::Value;

pub fn problem27() -> (Value, Duration) {
    let start = Instant::now();

    let mut value: i32 = 0;
    let mut length;
    let mut max_length = 0;

    for a in -1000..1000 {
        for b in -1000..=1000 {
            length = 0;

            for n in 0..1000 {
                if quadratic(a,b,n).is_probably_prime(30) == IsPrime::No {
                    break;
                }

                length += 1;
            }

            if length > max_length {
                max_length = length;
                value = a * b;
            }
        }
    }

    assert_eq!(value, -59231);
    return (Value::I32(value), start.elapsed());
}

fn quadratic(a: i32, b: i32, n: i32) -> Integer {
    Integer::from(n).pow(2).add(a * n).add(b)
}
