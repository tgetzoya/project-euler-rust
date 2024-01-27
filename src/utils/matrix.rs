#[derive(Debug)]
pub struct Matrix {
    col_len: usize,
    row_len: usize,
    rows: Vec<Vec<i64>>,
}

impl Matrix {
    pub fn new(row_len: usize, col_len: usize) -> Self {
        Self {col_len, row_len, rows: vec![vec![0; col_len]; row_len] }
    }

    pub fn new_with_default_value(row_len: usize, col_len: usize, value: i64) -> Self {
        Self {col_len, row_len, rows: vec![vec![value; col_len]; row_len] }
    }

    fn check_position(&self, x: usize, y: usize) {
        if x > self.row_len {
            panic!("Row {} is outside the row bounds of the matrix", x);
        }

        if y > self.col_len {
            panic!("Column {} is outside the bounds of row {} in the matrix", y, x);
        }
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.row_len, self.col_len)
    }

    pub fn get_point(&self, x: usize, y: usize) -> i64 {
        self.check_position(x, y);
        return self.rows.get(x).unwrap()[y];
    }

    pub fn set_point(&mut self, x: usize, y: usize, value: i64) {
        self.check_position(x, y);
        self.rows.get_mut(x).unwrap()[y] = value;
    }

    pub fn set_row(&mut self, x: usize, values: &[i64]) {
        self.check_position(x, values.len());

        let row = self.rows.get_mut(x).unwrap();
        for (idx, val) in values.iter().enumerate() {
            row[idx] = *val;
        }
    }
}