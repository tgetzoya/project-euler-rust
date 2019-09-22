/*
 * By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
 * What is the 10 001st prime number?
 *
 * Answer: 104743
 */

use crate::problems::util::is_prime;

pub fn run() -> u64 {
    let mut prime_count: u64 = 0;
    let mut idx: u64 = 2;

    while prime_count != 10_001 {
        if is_prime(idx) {
            prime_count += 1;
        }

        idx += 1;
    }

    return idx - 1;
}
