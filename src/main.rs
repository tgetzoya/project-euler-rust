use std::time::{Instant};

mod problems;
mod utils;

fn main() {
    let start = Instant::now();

    println!("Problem 1: {}", problems::problem1::problem1());
    println!("Problem 2: {}", problems::problem2::problem2());
    println!("Problem 3: {}", problems::problem3::problem3());
    println!("Problem 4: {}", problems::problem4::problem4());
    println!("Problem 5: {}", problems::problem5::problem5());
    println!("Problem 6: {}", problems::problem6::problem6());
    println!("Problem 7: {}", problems::problem7::problem7());
    println!("Problem 8: {}", problems::problem8::problem8());
    println!("Problem 9: {}", problems::problem9::problem9());

    println!("Total run time: {:?}", start.elapsed());
}
