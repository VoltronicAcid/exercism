pub struct Matrix {
    rows: Vec<Vec<u32>>,
    cols: Vec<Vec<u32>>,
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
        let width = rows.get(0).unwrap().len();
        let cols: Vec<Vec<u32>> = (0..width)
            .map(|col_idx| rows.iter().map(|r| r[col_idx]).collect::<Vec<u32>>())
            .collect();

        Matrix { rows, cols }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        if let Some(rslt) = self.rows.get(row_no - 1) {
            Some(rslt.clone())
        } else {
            None
        }
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        if let Some(rslt) = self.cols.get(col_no - 1) {
            Some(rslt.clone())
        } else {
            None
        }
    }
}
