mod problems;

use std::time::{Instant};

fn main() {
    let mut start: Instant;
    let mut val: u32;

    for idx in 5..=6 {
        start = Instant::now();
        val = problems::run(idx);
        println!("Problem {:03}: {}. Duration: {:?}",idx, val,start.elapsed());
    }
}
