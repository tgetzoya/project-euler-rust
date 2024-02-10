use std::fs;
use std::time::{Duration, Instant};

pub fn problem22() -> (u128, Duration) {
    let start = Instant::now();
    let mut value: u32 = 0;

    let mut words = fs::read_to_string("./files/0022_names.txt")
        .expect("Should have been able to read the file")
        .split(",")
        .map(|s| s.replace("\"", ""))
        .collect::<Vec<String>>();

    /* Needs to be after the collection and before the iterator. */
    words.sort();

    for (idx, val) in words.iter().enumerate() {
        value += val.chars().map(|c| c as u32 - 64).sum::<u32>() * (idx+1) as u32;
    }


    assert_eq!(value, 871198282);
    return (value as u128, start.elapsed());
}