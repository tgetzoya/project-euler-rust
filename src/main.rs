use std::time::{Instant};

mod problems;
mod utils;

fn main() {
    let start = Instant::now();

    let mut result;

    result = problems::problem1::problem1();
    println!("Problem 001: {:15}, {:?}", result.0, result.1);

    result = problems::problem2::problem2();
    println!("Problem 002: {:15}, {:?}", result.0, result.1);

    result = problems::problem3::problem3();
    println!("Problem 003: {:15}, {:?}", result.0, result.1);

    result = problems::problem4::problem4();
    println!("Problem 004: {:15}, {:?}", result.0, result.1);

    result = problems::problem5::problem5();
    println!("Problem 005: {:15}, {:?}", result.0, result.1);

    result = problems::problem6::problem6();
    println!("Problem 006: {:15}, {:?}", result.0, result.1);

    result = problems::problem7::problem7();
    println!("Problem 007: {:15}, {:?}", result.0, result.1);

    result = problems::problem8::problem8();
    println!("Problem 008: {:15}, {:?}", result.0, result.1);

    result = problems::problem9::problem9();
    println!("Problem 009: {:15}, {:?}", result.0, result.1);

    result = problems::problem10::problem10();
    println!("Problem 010: {:15}, {:?}", result.0, result.1);

    result = problems::problem11::problem11();
    println!("Problem 011: {:15}, {:?}", result.0, result.1);

    result = problems::problem12::problem12();
    println!("Problem 012: {:15}, {:?}", result.0, result.1);

    result = problems::problem13::problem13();
    println!("Problem 013: {:15}, {:?}", result.0, result.1);

    result = problems::problem14::problem14();
    println!("Problem 014: {:15}, {:?}", result.0, result.1);

    result = problems::problem15::problem15();
    println!("Problem 015: {:15}, {:?}", result.0, result.1);

    result = problems::problem16::problem16();
    println!("Problem 016: {:15}, {:?}", result.0, result.1);

    result = problems::problem17::problem17();
    println!("Problem 017: {:15}, {:?}", result.0, result.1);

    result = problems::problem18::problem18();
    println!("Problem 018: {:15}, {:?}", result.0, result.1);

    result = problems::problem19::problem19();
    println!("Problem 019: {:15}, {:?}", result.0, result.1);

    result = problems::problem20::problem20();
    println!("Problem 020: {:15}, {:?}", result.0, result.1);

    result = problems::problem21::problem21();
    println!("Problem 021: {:15}, {:?}", result.0, result.1);

    result = problems::problem22::problem22();
    println!("Problem 022: {:15}, {:?}", result.0, result.1);

    result = problems::problem23::problem23();
    println!("Problem 023: {:15}, {:?}", result.0, result.1);

    result = problems::problem24::problem24();
    println!("Problem 024: {:15}, {:?}", result.0, result.1);

    result = problems::problem25::problem25();
    println!("Problem 025: {:15}, {:?}", result.0, result.1);

    result = problems::problem26::problem26();
    println!("Problem 025: {:15}, {:?}", result.0, result.1);

    println!("Total run time: {:?}", start.elapsed());
}
