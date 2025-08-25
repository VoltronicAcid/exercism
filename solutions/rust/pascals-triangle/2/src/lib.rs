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
        (0..self.len).fold(Vec::new(), |mut acc, row_idx| {
            acc.push(
                (0..row_idx + 1)
                    .map(|idx| {
                        if idx == 0 || idx == row_idx {
                            1
                        } else {
                            acc[row_idx - 1][idx - 1] + acc[row_idx - 1][idx]
                        }
                    })
                    .collect::<Vec<u32>>(),
            );

            acc
        })
    }
}
