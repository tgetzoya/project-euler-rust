use std::ops::{Add, Rem};
use std::time::{Duration, Instant};
use crate::enums::value::Value;

const ONES: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
const TENS: [&str; 10] = [
    "zero",
    "ten",
    "twenty",
    "thirty",
    "forty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety",
];
const ORDERS: [&str; 6] = [
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

pub fn problem17() -> (Value, Duration) {
    let start = Instant::now();
    let mut value= 0;

    for idx in 1..1001 {
        value = value + number_to_word(idx, 0).len();
    }

    assert_eq!(value, 21124);
    return (Value::U16(value as u16), start.elapsed());
}

fn number_to_word(num: u64, order: i8) -> String {
    let mut result: String;

    match num {
        0..=19 => result = ONES[num as usize].to_owned(),
        20..=99 => {
            result = String::from(TENS[(num / 10) as usize]);

            if num % 10 > 0 {
                result = result.add(ONES[(num % 10) as usize]);
            }
        }
        100..=999 => {
            result = String::from(ONES[(num / 100) as usize])
                .add("hundred");

            let remainder = num % 100;

            if remainder > 0 {
                if order <= 0 {
                    result = result.add("and");
                }

                result = result.add(&*number_to_word(remainder, 0));
            }
        }
        _ => {
            result = number_to_word(num / 1000, order + 1)
                .add(ORDERS[order as usize]);

            let remainder = num.rem(1000);

            if remainder > 0 {
                result = result.add(&*number_to_word(remainder, order));
            }
        }
    }

    return result;
}