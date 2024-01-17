pub fn is_prime<T: Into<u128>>(number: T) -> bool {
    let num = number.into();

    if num <= 1 {
        return false;
    }

    for idx in 2..(num as f64).sqrt() as u128 {
        if num % idx == 0 {
            return false;
        }
    }

    return true;
}