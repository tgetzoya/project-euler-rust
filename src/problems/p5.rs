/*
 * 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
 *
 * What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
 *
 * Answer: 232792560
 */

pub fn run() -> u64 {
    /* Start with a sufficient big number */
    let mut idx = 20 * 19 * 18 * 17 * 15;

    'main: loop {
        /* All multiples of 20 end with 0 and first significant digit is an even number. */
        idx += 1;

        let num = 20 * idx;

        for idx in (2..20).rev() {
            if num % idx > 0 {
                continue 'main;
            }
        }

        return num;
    }
}
