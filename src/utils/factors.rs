pub fn get_factors(number: u128) -> Vec<u128> {
    let mut factors = Vec::new();

    let step = if number % 2 == 0 {1} else {2};

    for i in (1u128..).take_while(|x| x * x < number).step_by(step) {
        if number % i == 0 {
            factors.push(i);
            factors.push(number / i);
        }
    }

    factors.sort_by(|a, b| a.cmp(b));

    return factors;
}