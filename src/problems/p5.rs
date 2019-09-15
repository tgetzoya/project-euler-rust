/*
 * 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
 *
 * What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
 *
 * Answer: 232792560
 */

pub fn run() -> u32 {
    /* Assuming at least twice as many digits as 2520 (4 digits to 8, guessing starting at 9) */
    let mut smallest: u32 = 100_000_000;
    let mut is_divisible: bool;

    while smallest < std::u32::MAX {
        is_divisible = true;

        for idx in 2..20 {
            if smallest % idx != 0 {
                is_divisible = false;
                break;
            }
        }

        if is_divisible {
            return smallest;
        }

        /* It'll have to be a multiple of 20, right? */
        smallest += 20;
    }

    return 0;
}
