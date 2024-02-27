use std::time::{Duration, Instant};
use crate::enums::value::Value;

pub fn problem18() -> (Value, Duration) {
    let start = Instant::now();

    let mut triangle:Vec<Vec<u32>> = create_triangle();

    for idx in (1..15).rev() {
        for jdx in 0..(triangle[idx].len() -1) {
            if triangle[idx][jdx] > triangle[idx][jdx+1] {
                triangle[idx-1][jdx] += triangle[idx][jdx];
            } else {
                triangle[idx-1][jdx] += triangle[idx][jdx+1];
            }
        }
    }

    assert_eq!(triangle[0][0], 1074);
    return (Value::U16(triangle[0][0] as u16), start.elapsed());
}

fn create_triangle() -> Vec<Vec<u32>> {
    let mut rows: Vec<Vec<u32>> = Vec::new();

    rows.push(vec![75]);
    rows.push(vec![95, 64]);
    rows.push(vec![17, 47, 82]);
    rows.push(vec![18, 35, 87, 10]);
    rows.push(vec![20, 04, 82, 47, 65]);
    rows.push(vec![19, 01, 23, 75, 03, 34]);
    rows.push(vec![88, 02, 77, 73, 07, 63, 67]);
    rows.push(vec![99, 65, 04, 28, 06, 16, 70, 92]);
    rows.push(vec![41, 41, 26, 56, 83, 40, 80, 70, 33]);
    rows.push(vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29]);
    rows.push(vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14]);
    rows.push(vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57]);
    rows.push(vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48]);
    rows.push(vec![63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31]);
    rows.push(vec![04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23]);

    return rows;
}