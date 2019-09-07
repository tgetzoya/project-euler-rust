mod p1;
mod p2;
mod p3;
mod util;

pub fn run(problem_id: u16) {
    match problem_id {
        1 => p1::run(),
        2 => p2::run(),
        3 => p3::run(),
        _ => println!("Unknown Problem Id: {}", problem_id)
    }
}