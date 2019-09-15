/*
 * A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
 *
 * Find the largest palindrome made from the product of two 3-digit numbers.
 *
 * Answer: 906609
 */

use crate::problems::util::{number_to_digits};

pub fn run() -> u32 {
    let mut x: u32 = 999;
    let mut y: u32;
    let mut z: u32;
    let mut is_palindrome: bool;
    let mut digits: Vec<u8>;
    let mut largest: u32 = 0;

    while x > 99 {
        y = 999;
        while y > 99 {
            z = x * y;
            digits = number_to_digits ( z as u128);
            is_palindrome = true;

            for idx in 0..(digits.len() / 2) {

                if digits[idx] != digits[digits.len() - (1 + idx)] {
                    is_palindrome = false;
                    break;
                }
            }

            if is_palindrome && z > largest {
                largest =  z;
            }

            y -= 1;
        }

        x -= 1;
    }

    return largest;
}
