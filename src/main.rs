use std::time::{Instant};

mod problems;
mod utils;

fn main() {
    let start = Instant::now();

    let mut result = problems::problem1::problem1();
    println!("Problem 1: {:15}, {:?}", result.0, result.1);

    result = problems::problem2::problem2();
    println!("Problem 2: {:15}, {:?}", result.0, result.1);

    result = problems::problem3::problem3();
    println!("Problem 3: {:15}, {:?}", result.0, result.1);

    result = problems::problem4::problem4();
    println!("Problem 4: {:15}, {:?}", result.0, result.1);

    result = problems::problem5::problem5();
    println!("Problem 5: {:15}, {:?}", result.0, result.1);

    result = problems::problem6::problem6();
    println!("Problem 6: {:15}, {:?}", result.0, result.1);

    result = problems::problem7::problem7();
    println!("Problem 7: {:15}, {:?}", result.0, result.1);

    result = problems::problem8::problem8();
    println!("Problem 8: {:15}, {:?}", result.0, result.1);

    result = problems::problem9::problem9();
    println!("Problem 9: {:15}, {:?}", result.0, result.1);

    result = problems::problem10::problem10();
    println!("Problem 10: {:15}, {:?}", result.0, result.1);

    println!("Total run time: {:?}", start.elapsed());
}
