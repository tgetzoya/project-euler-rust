/*
 * Utility functions used across more than one problem
 */

//use std::collections::BTreeSet;
//
//pub fn get_factors(number: u64, include_self: bool) -> BTreeSet<u64> {
//    let mut factors: BTreeSet<u64> = BTreeSet::new();
//    let half: u64 = number / 2;
//
//    if include_self {
//        factors.insert(1);
//        factors.insert(number);
//    }
//
//    for idx in 2..half {
//        if number % idx == 0 {
//            factors.insert(idx);
//            factors.insert(number / idx);
//        }
//    }
//
//    return factors;
//}

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