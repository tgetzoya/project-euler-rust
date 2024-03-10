use std::collections::HashSet;
use std::time::{Duration, Instant};

use rug::Integer;
use rug::ops::Pow;

use crate::enums::value::Value;

pub fn problem29() -> (Value, Duration) {
    let start = Instant::now();

    let mut list: HashSet<Integer> = HashSet::new();

    for a in 2u32..=100 {
        for b in 2u32..=100 {
            list.insert(Integer::from(a).pow(b));
        }
    }

    assert_eq!(list.len(), 9183);
    return (Value::U16(list.len() as u16), start.elapsed());
}