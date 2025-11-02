pub struct Matrix {
    rows: Vec<Vec<u32>>,
    columns: Vec<Vec<u32>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let rows: Vec<Vec<u32>> = Self::get_rows(input);
        let columns: Vec<Vec<u32>> = Self::get_columns(&rows);

        Matrix { rows, columns }
    }

    fn get_rows(input: &str) -> Vec<Vec<u32>> {
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect()
            })
            .collect()
    }

    fn get_columns(rows: &[Vec<u32>]) -> Vec<Vec<u32>> {
        (0..rows.first().unwrap().len())
            .map(|col| (0..rows.len()).map(|row| rows[row][col]).collect())
            .collect()
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        match row_no {
            0 => None,
            _ => self.rows.get(row_no - 1).cloned(),
        }
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        match col_no {
            0 => None,
            _ => self.columns.get(col_no - 1).cloned(),
        }
    }
}
