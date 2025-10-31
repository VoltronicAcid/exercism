pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(num_rows: usize) -> Self {
        let mut rows: Vec<Vec<u32>> = vec![];

        for row in 0..num_rows {
            rows.push(vec![]);
            for col in 0..row + 1 {
                let val: u32 = match col {
                    i if i == 0 || i == row => 1,
                    _ => rows[row - 1][col - 1] + rows[row - 1][col],
                };
                rows[row].push(val);
            }
        }
        PascalsTriangle { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
