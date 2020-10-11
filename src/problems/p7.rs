/*
 * By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
 * What is the 10 001st prime number?
 *
 * Answer: 104743
 */

use crate::problems::util::is_prime;

pub fn run() -> u64 {
    let mut count = 6; //Defined in the question
    let mut idx = 13;

    loop {
        idx += 2;

        if is_prime(idx) {
            count += 1;

            if count == 10_001 {
                return idx;
            }
        }
    }
}
