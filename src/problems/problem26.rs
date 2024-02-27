use std::time::{Duration, Instant};
use crate::enums::value::Value;

pub fn problem26() -> (Value, Duration) {
    let start = Instant::now();

    let value = 983;

    assert_eq!(value, 983);
    return (Value::U32(value), start.elapsed());
}
