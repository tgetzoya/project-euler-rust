/*
 * The prime factors of 13195 are 5, 7, 13 and 29.
 *
 * What is the largest prime factor of the number 600851475143 ?
 *
 * Answer: 6857
 */

use crate::problems::util::is_prime;

pub fn run() -> u32 {
    let num: u64 = 600851475143;
    let half: u64 = 300425737571;
    let mut largest: u32 = 0;

    for idx in 2..half {
        if num % idx == 0 {
            if is_prime(num / idx) {
                largest = (num / idx) as u32;
                break;
            }
        }
    }

    return largest;
}