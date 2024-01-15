pub fn get_factors(number: u128) -> Vec<u128> {
    let mut value = number;
    let mut factors = Vec::new();

    for i in (1u128..).take_while(|x| x * x < number) {
        if value % i == 0 {
            value /= i;
            factors.push(i);
        }
    }

    factors.sort_by(|a, b| a.cmp(b));

    return factors;
}