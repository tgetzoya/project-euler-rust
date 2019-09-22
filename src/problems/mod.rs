mod p1;
mod p2;
mod p3;
mod p4;
mod p5;
mod p6;
mod p7;
mod p8;
mod p9;
mod util;

pub fn run(problem_id: u16) -> u64 {
    let mut val: u64 = 0;

    match problem_id {
        1 => val = p1::run(),
        2 => val = p2::run(),
        3 => val = p3::run(),
        4 => val = p4::run(),
        5 => val = p5::run(),
        6 => val = p6::run(),
        7 => val = p7::run(),
        8 => val = p8::run(),
        9 => val = p9::run(),
        _ => println!("Unknown Problem Id: {}", problem_id)
    }

    return val;
}
