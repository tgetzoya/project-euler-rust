/*
 * The sum of the squares of the first ten natural numbers is,
 *                      1^2 + 2^2 + ... + 10^2 = 385
 *
 * The square of the sum of the first ten natural numbers is,
 *                      (1 + 2 + ... + 10)^2 = 55^2 = 3025
 *
 * Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
 *
 * Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
 *
 * Answer: 25164150
 */

pub fn run() -> u32 {
    let mut sum_of_sqaures: u32 = 0;
    let mut square_of_sums: u32 = 0;

    for idx in 1..=100 {
        sum_of_sqaures += idx * idx;
        square_of_sums += idx;
    }

    square_of_sums = square_of_sums * square_of_sums;

    return square_of_sums - sum_of_sqaures;
}
