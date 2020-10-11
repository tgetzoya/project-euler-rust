/*
 * The prime factors of 13195 are 5, 7, 13 and 29.
 *
 * What is the largest prime factor of the number 600851475143 ?
 *
 * Answer: 6857
 */

use crate::problems::util::is_prime;
use crate::problems::util::factors;
use std::iter::FromIterator;

pub fn run() -> u64 {
    let mut largest: u64 = 0;

    let mut bob = Vec::from_iter(factors(600851475143, false));
    bob.sort();
    bob.reverse();

    for factor in bob {
        if is_prime(factor as u64) {
            largest =  factor as u64;
            break;
        }
    }

    return largest as u64;
}