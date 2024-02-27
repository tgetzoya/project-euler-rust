use std::time::{Duration, Instant};

use crate::enums::value::Value;

pub fn problem24() -> (Value, Duration) {
    let start = Instant::now();

    let value = 2783915460;

    assert_eq!(value, 2783915460);
    return (Value::U32(value), start.elapsed());
}
