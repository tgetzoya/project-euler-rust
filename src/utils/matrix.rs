#[derive(Debug)]
pub struct Matrix {
    rows: Vec<Vec<i32>>,
}

impl Matrix {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self { rows: vec![vec![0; columns]; rows] }
    }

    pub fn get_point(&self, x: usize, y: usize) -> i32 {
        let optional = self.rows.get(x);

        if optional.is_none() {
            panic!("Row {} is outside the row bounds of the matrix", x);
        }

        let row = optional.unwrap();

        if y > row.len() {
            panic!("Column {} is not a part of row {} in the matrix", y, x);
        }

        return row[y];
    }

    #[allow(dead_code)]
    pub fn set_point(&mut self, x: usize, y: usize, value: i32) {
        let optional = self.rows.get_mut(x);

        if optional.is_none() {
            panic!("Row {} is outside the row bounds of the matrix", x);
        }

        let row = optional.unwrap();

        if y > row.len() {
            panic!("Column {} is not a part of row {} in the matrix", y, x);
        }

        row[y] = value;
    }

    pub fn set_row(&mut self, x: usize, values: &[i32]) {
        let optional = self.rows.get_mut(x);

        if optional.is_none() {
            panic!("Row {} is outside the row bounds of the matrix", x);
        }

        let row = optional.unwrap();

        if values.len() > row.len() {
            panic!("Row {} has a length of {}, but {} items are in the array of values", x, row.len(), values.len());
        }

        for (idx, val) in values.iter().enumerate() {
            row[idx] = *val;
        }
    }
}