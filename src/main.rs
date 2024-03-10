use std::time::{Duration, Instant};

mod problems;
mod utils;
mod enums;

fn main() {
    let start = Instant::now();

    let mut result: (enums::value::Value, Duration);

    result = problems::problem1::problem1();
    println!("Problem 001: {}, {:?}", result.0, result.1);

    result = problems::problem2::problem2();
    println!("Problem 002: {}, {:?}", result.0, result.1);

    result = problems::problem3::problem3();
    println!("Problem 003: {}, {:?}", result.0, result.1);

    result = problems::problem4::problem4();
    println!("Problem 004: {}, {:?}", result.0, result.1);

    result = problems::problem5::problem5();
    println!("Problem 005: {}, {:?}", result.0, result.1);

    result = problems::problem6::problem6();
    println!("Problem 006: {}, {:?}", result.0, result.1);

    result = problems::problem7::problem7();
    println!("Problem 007: {}, {:?}", result.0, result.1);

    result = problems::problem8::problem8();
    println!("Problem 008: {}, {:?}", result.0, result.1);

    result = problems::problem9::problem9();
    println!("Problem 009: {}, {:?}", result.0, result.1);

    result = problems::problem10::problem10();
    println!("Problem 010: {}, {:?}", result.0, result.1);

    result = problems::problem11::problem11();
    println!("Problem 011: {}, {:?}", result.0, result.1);

    result = problems::problem12::problem12();
    println!("Problem 012: {}, {:?}", result.0, result.1);

    result = problems::problem13::problem13();
    println!("Problem 013: {}, {:?}", result.0, result.1);

    result = problems::problem14::problem14();
    println!("Problem 014: {}, {:?}", result.0, result.1);

    result = problems::problem15::problem15();
    println!("Problem 015: {}, {:?}", result.0, result.1);

    result = problems::problem16::problem16();
    println!("Problem 016: {}, {:?}", result.0, result.1);

    result = problems::problem17::problem17();
    println!("Problem 017: {}, {:?}", result.0, result.1);

    result = problems::problem18::problem18();
    println!("Problem 018: {}, {:?}", result.0, result.1);

    result = problems::problem19::problem19();
    println!("Problem 019: {}, {:?}", result.0, result.1);

    result = problems::problem20::problem20();
    println!("Problem 020: {}, {:?}", result.0, result.1);

    result = problems::problem21::problem21();
    println!("Problem 021: {}, {:?}", result.0, result.1);

    result = problems::problem22::problem22();
    println!("Problem 022: {}, {:?}", result.0, result.1);

    result = problems::problem23::problem23();
    println!("Problem 023: {}, {:?}", result.0, result.1);

    result = problems::problem24::problem24();
    println!("Problem 024: {}, {:?}", result.0, result.1);

    result = problems::problem25::problem25();
    println!("Problem 025: {}, {:?}", result.0, result.1);

    result = problems::problem26::problem26();
    println!("Problem 026: {}, {:?}", result.0, result.1);

    result = problems::problem27::problem27();
    println!("Problem 027: {}, {:?}", result.0, result.1);

    result = problems::problem28::problem28();
    println!("Problem 028: {}, {:?}", result.0, result.1);

    result = problems::problem29::problem29();
    println!("Problem 029: {}, {:?}", result.0, result.1);

    result = problems::problem30::problem30();
    println!("Problem 030: {}, {:?}", result.0, result.1);

    println!("Total run time: {:?}", start.elapsed());
}
