use std::time::{Duration, Instant};

use crate::enums::value::Value;
use crate::enums::directions::Directions;
use crate::utils::matrix::Matrix;



pub fn problem28() -> (Value, Duration) {
    let start = Instant::now();

    let value: u32 = calculate_diagonals(setup_matrix(1001));

    assert_eq!(value, 669171001);
    return (Value::U32(value), start.elapsed());
}

fn setup_matrix(size: usize) -> Matrix<u32> {
    let mut matrix: Matrix<u32> = Matrix::new(size, size, 0);

    let mut direction: Directions = Directions::EAST;
    let mut point = (((size / 2) as f32).ceil() as usize, ((size / 2) as f32).ceil() as usize);
    let mut idx = 2u32;

    matrix.set_point(point.0, point.1, 1u32);
    point.1 += 1;

    while point != (0, size) {
        matrix.set_point(point.0, point.1, idx);
        idx += 1;
        (point, direction) = next_point(point, direction, &matrix);
    }

    matrix
}

fn next_point(point: (usize, usize), direction: Directions, matrix: &Matrix<u32>) -> ((usize, usize), Directions) {
    let next_point;
    let next_direction;

    match direction {
        Directions::EAST => {
            if matrix.get_point(point.0 + 1, point.1).eq( &0u32) {
                next_point = (point.0 + 1, point.1);
                next_direction = Directions::SOUTH;
            } else {
                next_point = (point.0, point.1 + 1);
                next_direction = direction;
            }
        }
        Directions::NORTH => {
            if matrix.get_point(point.0, point.1 + 1).eq(&0u32) {
                next_point = (point.0, point.1 + 1);
                next_direction = Directions::EAST;
            } else {
                next_point = (point.0 - 1, point.1);
                next_direction = direction;
            }
        }
        Directions::SOUTH => {
            if matrix.get_point(point.0, point.1 - 1).eq(&0u32) {
                next_point = (point.0, point.1 - 1);
                next_direction = Directions::WEST;
            } else {
                next_point = (point.0 + 1, point.1);
                next_direction = direction;
            }
        }
        Directions::WEST => {
            if matrix.get_point(point.0 - 1, point.1).eq(&0u32) {
                next_point = (point.0 - 1, point.1);
                next_direction = Directions::NORTH;
            } else {
                next_point = (point.0, point.1 - 1);
                next_direction = direction;
            }
        }
    }

    return (next_point, next_direction);
}

fn calculate_diagonals(matrix: Matrix<u32>) -> u32 {
    let mut forward = (0,0);
    let mut backward = (0,matrix.get_dimensions().1 - 1);
    let mut sum = 0u32;

    while forward != matrix.get_dimensions() {
        sum += matrix.get_point(forward.0, forward.1);

        if forward != backward {
            sum += matrix.get_point(backward.0, backward.1);
        }

        forward = (forward.0 + 1, forward.1 + 1);

        if backward.1 > 0 {
            backward = (backward.0 + 1, backward.1 - 1);
        }
    }

    sum
}