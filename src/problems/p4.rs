/*
 * A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
 *
 * Find the largest palindrome made from the product of two 3-digit numbers.
 *
 * Answer: 906609
 */

use std::iter::FromIterator;

use crate::problems::util::{factors, is_palindrome};

pub fn run() -> u64 {
    /* 100 ^ 2 to 999 ^ 2  */
    for idx in (10000..998001).rev() {
        if is_palindrome(idx) {
            let mut factors = Vec::from_iter(factors(idx, false));

            if factors.len() < 2 {
                continue;
            }

            factors.sort();

            for jdx in 0..factors.len() {
                if factors[jdx] > 99 && factors[jdx] < 1000 {
                    if factors[factors.len() - (1 + jdx)] > 99 && factors[factors.len() - (1 + jdx)] < 1000 {
                        return idx as u64;
                    }
                }
            }
        }
    }

    return 0;
}