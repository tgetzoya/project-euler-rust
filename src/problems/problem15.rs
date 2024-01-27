use std::time::{Duration, Instant};
use crate::utils::matrix::Matrix;

pub fn problem15() -> (u128, Duration) {
    let start = Instant::now();

    let  matrix = &mut Matrix::new_with_default_value(20, 20, -1);
    let value =  next(matrix, 0, 0);

    assert_eq!(value, 137846528820);
    return (value as u128, start.elapsed());
}

fn next(matrix: &mut Matrix, x: usize, y: usize) -> i64 {
    let bounds = matrix.get_dimensions();

    let mut down = 1i64;
    let mut right = 1i64;

    if y+1 < bounds.1 {
        down = matrix.get_point(x, y+1);

        if down == -1 {
            down = next(matrix, x, y+1);
        }
    }

    if x+1 < bounds.0 {
        right = matrix.get_point(x + 1, y);

        if right == -1 {
            right = next(matrix, x+1, y);
        }
    }

    matrix.set_point(x, y, down + right);

    down + right
}