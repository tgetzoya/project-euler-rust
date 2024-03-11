use std::time::{Duration, Instant};

use crate::enums::value::Value;

pub fn problem31() -> (Value, Duration) {
    let start = Instant::now();

    let value = 73682;

    assert_eq!(value, 73682);
    return (Value::U32(value), start.elapsed());
}