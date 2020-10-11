/*
 * Utility functions used across more than one problem
 */

use std::collections::HashSet;

pub fn is_prime(number: u64) -> bool {
    if number == 0 || number == 1 {
        return false;
    }

    if number == 2 || number == 3 {
        return true;
    }

    if number % 2 == 0 {
        return false;
    }

    let sqrt: u64 = (number as f64).sqrt() as u64;

    let mut idx: u64 = 3;

    while idx <= sqrt {
        if number % idx == 0 {
            return false;
        }
        idx += 2;
    }

    return true;
}

pub fn number_to_digits(num: u128) -> Vec<u8> {
    let mut digits: Vec<u8> = Vec::new();
    let mut val = num;

    while val > 10 {
        digits.push((val % 10) as u8);

        val = val / 10;
    }

    digits.push(val as u8);

    return digits;
}

pub fn factors(num: u128, include_self: bool) -> HashSet<u128> {
    let mut factors: HashSet<u128> = HashSet::new();

    if include_self {
        factors.insert(1);
        factors.insert(num);
    }

    let mut idx = 2;

    while idx * idx <= num {
        if !factors.contains(&idx) && num % idx == 0 {
            factors.insert(idx);
            factors.insert(num / idx);
        }

        idx += 1;
    }

    return factors;
}

pub fn is_palindrome(num: u128) -> bool {
    let digits = number_to_digits ( num as u128);

    for idx in 0..(digits.len() / 2) {
        if digits[idx] != digits[digits.len() - (1 + idx)] {
            return false;
        }
    }

    return true;
}