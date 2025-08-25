#![allow(unused)]
pub struct PascalsTriangle {
    len: usize,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let len: usize = row_count as usize;

        PascalsTriangle { len }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut values: Vec<Vec<u32>> = Vec::new();

        for idx in 0..self.len {
            let mut curr = vec![1; idx + 1];
            for jdx in 1..idx {
                curr[jdx] = values[idx - 1][jdx - 1] + values[idx - 1][jdx];
            }

            values.push(curr);
        }

        values
    }
}
