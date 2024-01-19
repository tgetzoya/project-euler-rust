use std::time::{Duration, Instant};

pub fn problem9() -> (u128, Duration) {
    let start = Instant::now();

    let mut value = 31875000;

    for a in 1..999 {
        for b in a+1..999 {
            for c in b+1..999 {
                if a+b+c == 1000 && (a*a) + (b*b) == (c*c) {
                    value = a*b*c;
                    break;
                }
            }
        }
    }

    assert_eq!(value, 31875000);
    return (value, start.elapsed());
}