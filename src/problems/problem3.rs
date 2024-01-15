use crate::utils;
use crate::utils::primes::is_prime;

pub fn problem3() -> u32 {
    let largest_prime = utils::factors::get_factors(600851475143)
        .into_iter()
        .filter(|&x| is_prime(x))
        .last()
        .unwrap();

    assert_eq!(largest_prime, 6857);

    return largest_prime as u32;
}