pub fn problem2() -> u32 {
    let mut x = 1;
    let mut y = 1;
    let mut sum = 0;

    while y < 4_000_000 {
        if y % 2 == 0 {
            sum += y;
        }

        y += x;
        x = y - x;
    }

    assert_eq!(sum, 4613732);

    return sum;
}