pub fn get_factors(number: u128, include_one_and_value: bool) -> Vec<u128> {
    let mut factors = Vec::new();

    if include_one_and_value {
        factors.push(1);
    }

    if number > 1 {
        if include_one_and_value {
            factors.push(number);
        }

        for i in (2u128..).take_while(|x| x * x <= number) {
            if number % i == 0 {
                factors.push(i);

                if number / i != i {
                    factors.push(number / i);
                }
            }
        }

        factors.sort_by(|a, b| a.cmp(b));
    }

    return factors;
}

pub fn get_factor_count(number: u128, include_one_and_value: bool) -> u32 {
    if number == 1 {
        return 1;
    }

    let mut count: u32 = if include_one_and_value {2} else {u32::MIN};

    for i in (2u128..).take_while(|x| x * x <= number) {
        if number % i == 0 {
            count += 1;

            if number / i != i {
                count += 1;
            }
        }
    }

    return count;
}