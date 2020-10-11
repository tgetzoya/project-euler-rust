/*
 *
 * The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
 * Find the sum of all the primes below two million.
 *
 * Answer: 142913828922
 */

use crate::problems::util::is_prime;

pub fn run() -> u64 {
    let mut sum = 17;
    let mut idx = 9;

    while idx < 2_000_000 {
        if is_prime(idx) {
            sum += idx;
        }

        idx += 2;
    }

    return sum;
}
