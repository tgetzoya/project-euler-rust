use crate::utils;

pub fn problem10() -> u64 {
    let mut value = 2;


    for val in (3..2_000_000).step_by(2).filter(|&x| utils::primes::is_prime(x as u128)) {
        value += val;
    }

    assert_eq!(value, 142913828922);
    return value;
}