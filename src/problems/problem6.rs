pub fn problem6() -> u32 {
    let value = square_of_sum(100) - sum_of_squares(100);

    assert_eq!(value, 25164150);
    return value;
}

fn sum_of_squares(num: u32) -> u32 {
    return (1..(num + 1)).map(|x| x.pow(2)).sum();
}

fn square_of_sum(num: u32) -> u32 {
    return (1..(num + 1)).sum::<u32>().pow(2);
}