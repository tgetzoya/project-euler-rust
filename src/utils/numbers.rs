use rug::{Integer};

pub fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    return a;
}

pub fn is_palindrome(number: u128) -> bool {
    let reversed = number.to_string().chars().rev().collect::<String>().parse::<u128>().unwrap();

    return number == reversed;
}

pub fn lcm(a: u128, b: u128) -> u128 {
    if a == 0 || b == 0 {
        return 0;
    }

    (a * b) / gcd(a, b)
}

pub fn factorial(num: u128) -> Integer {
    let mut value: Integer = Integer::from(1);

    for idx in 2..=num {
        value = value * idx;
    }

    return value;
}

pub fn to_digits(num: u32) -> Vec<u8> {
    let mut val: u32 = num.clone();
    let mut digits: Vec<u8> = Vec::new();

    while val >= 10 {
        digits.push((val % 10) as u8);
        val /= 10;
    }

    digits.push(val as u8);

    digits
}