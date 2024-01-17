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