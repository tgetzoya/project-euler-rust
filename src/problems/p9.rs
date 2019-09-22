/*
 * A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
 *      a2 + b2 = c2
 * For example, 32 + 42 = 9 + 16 = 25 = 52.
 *
 * There exists exactly one Pythagorean triplet for which a + b + c = 1000.
 * Find the product abc.
 *
 * Answer: 31875000
 */

pub fn run() -> u64 {
    let mut a: u32;

    for c in (1..500).rev() {
        for b in (1..c).rev() {
            a = 1000 - c - b;

            if (a*a) + (b*b) == (c*c) {
                return (a * b* c) as u64;
            }
        }
    }

    return 0;
}
