pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden
        .iter()
        .enumerate()
        .map(|(y, &row)| {
            row.chars()
                .enumerate()
                .map(move |(x, ch)| match ch {
                    ' ' => match count_neighbors(y, x, garden) {
                        count @ 1..9 => char::from_digit(count, 10).unwrap(),
                        _ => ch,
                    },
                    _ => ch,
                })
                .collect()
        })
        .collect()
}

fn count_neighbors(y: usize, x: usize, garden: &[&str]) -> u32 {
    (y.saturating_sub(1)..=(y + 1).min(garden.len() - 1))
        .map(|row_idx| {
            (x.saturating_sub(1)..=(x + 1).min(garden[row_idx].len() - 1))
                .filter(|&col_idx| garden[row_idx].chars().nth(col_idx) == Some('*'))
                .count() as u32
        })
        .sum()
}
