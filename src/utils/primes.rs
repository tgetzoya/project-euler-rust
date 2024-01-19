pub fn is_prime<T: Into<u128>>(number: T) -> bool {
    let num = number.into();

    if num <= 1 {
        return false;
    }

    if num == 2 {
        return true;
    }

    if num % 2 == 0 {
        return false;
    }

    let mut factor = 3;
    while factor*factor <= num {
        if num % factor == 0 {
            return false;
        }

        factor += 2;
    }

    return true;
}