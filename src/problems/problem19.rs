use std::time::{Duration, Instant};
use chrono::Months;
use chrono::prelude::*;

pub fn problem19() -> (u128, Duration) {
    let start = Instant::now();

    let mut value: u8 = 0;

    let mut date = NaiveDate::from_ymd_opt(1901,1,1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2000, 12, 31).unwrap();

    while date < end_date {
        if date.weekday() == Weekday::Sun {
            value += 1;
        }
        date = date.checked_add_months(Months::new(1)).unwrap();
    }


    assert_eq!(value, 171);
    return (value as u128, start.elapsed());
}