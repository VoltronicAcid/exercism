pub struct Matrix {
    rows: Vec<Vec<u32>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let rows: Vec<Vec<u32>> = input
            .lines()
            .map(|l| {
                l.split(" ")
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect();

        Matrix { rows }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        self.rows.get(row_no - 1).cloned()
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        let width = self.rows.first().unwrap().len();
        if col_no <= width {
            Some(
                self.rows
                    .iter()
                    .map(|row| *row.get(col_no - 1).unwrap())
                    .collect::<Vec<u32>>(),
            )
        } else {
            None
        }
    }
}
