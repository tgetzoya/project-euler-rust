use std::time::{Duration, Instant};

pub fn problem6() -> (u128, Duration) {
    let start = Instant::now();

    let value: u128 = (square_of_sum(100) - sum_of_squares(100)) as u128;

    assert_eq!(value, 25164150);
    return (value, start.elapsed());
}

fn sum_of_squares(num: u32) -> u32 {
    return (1..(num + 1)).map(|x| x.pow(2)).sum();
}

fn square_of_sum(num: u32) -> u32 {
    return (1..(num + 1)).sum::<u32>().pow(2);
}