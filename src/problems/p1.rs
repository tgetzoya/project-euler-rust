/*
 * If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
 * The sum of these multiples is 23.
 *
 * Find the sum of all the multiples of 3 or 5 below 1000.
 *
 * Correct Answer: 233168
 */

pub fn run() {
    let mut sum: u32 = 0;

    for n in 1..1000 {
        if (n % 5 == 0) || (n % 3 == 0) {
            sum += n;
        }
    }

    println!("Problem 001: {}", sum);
}