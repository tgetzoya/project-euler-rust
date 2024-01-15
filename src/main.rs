use std::time::{Instant};

mod problems;
mod utils;

fn main() {
    let start = Instant::now();

    println!("{}", problems::problem1::problem1());
    println!("{}", problems::problem2::problem2());
    println!("{}", problems::problem3::problem3());

    println!("Total run time: {:?}", start.elapsed());
}
