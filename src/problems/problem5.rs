use crate::utils;

pub fn problem5() -> u32 {
    let value = (11..20).reduce(utils::numbers::lcm).unwrap();

    assert_eq!(value,232792560);
    return value as u32;
}