use crate::utils;

pub fn problem4() -> u32 {
    let mut value = u128::MIN;

    let start = 999*999;
    let end = 100*100;

    'main_loop: for idx in (end..start).rev().filter(|&x| utils::numbers::is_palindrome(x)) {
        let factors = utils::factors::get_factors(idx as u128)
            .into_iter()
            .filter(|&x| x > 99 && x < 1000)
            .collect::<Vec<u128>>();

        if factors.len() < 2 {
            continue;
        }

        for jdx in factors.iter() {
            for kdx in factors.iter() {
                if jdx * kdx == idx as u128 {
                    value = idx;
                    break 'main_loop;
                }
            }
        }
    }

    assert_eq!(value,906609);
    return value as u32;
}