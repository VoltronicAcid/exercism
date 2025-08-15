use std::cmp::{max, min};

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let height = garden.len();
    (0..height)
        .map(|row| {
            let width = garden[row].len();
            (0..width)
                .map(|col| match garden[row].as_bytes()[col] {
                    b'*' => '*',
                    _ => {
                        match (max(0, row.saturating_add_signed(-1))..min(height, row + 2))
                            .map(|r_idx| {
                                (max(0, col.saturating_add_signed(-1))..min(width, col + 2)).fold(
                                    0,
                                    |count, c_idx| {
                                        count
                                            + if garden[r_idx].as_bytes()[c_idx] == b'*' {
                                                1
                                            } else {
                                                0
                                            }
                                    },
                                )
                            })
                            .sum()
                        {
                            0 => ' ',
                            num => char::from_digit(num, 10).unwrap(),
                        }
                    }
                })
                .collect()
        })
        .collect()
}
