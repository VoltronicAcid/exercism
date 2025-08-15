pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return Vec::new();
    } else if garden[0].is_empty() {
        return vec![String::from(""); garden.len()];
    }

    let mut counts: Vec<Vec<u8>> = vec![vec![0u8; garden[0].len()]; garden.len()];

    garden.iter().enumerate().for_each(|(row, s)| {
        for (col, ch) in s.char_indices() {
            if ch == '*' {
                counts[row][col] = u8::MAX;

                if row > 0 {
                    if col > 0 {
                        counts[row - 1][col - 1] = counts[row - 1][col - 1].saturating_add(1);
                    }
                    counts[row - 1][col] = counts[row - 1][col].saturating_add(1);

                    if col + 1 < s.len() {
                        counts[row - 1][col + 1] = counts[row - 1][col + 1].saturating_add(1);
                    }
                }

                if col > 0 {
                    counts[row][col - 1] = counts[row][col - 1].saturating_add(1);
                }

                if col + 1 < s.len() {
                    counts[row][col + 1] = counts[row][col + 1].saturating_add(1);
                }

                if row + 1 < garden.len() {
                    if col > 0 {
                        counts[row + 1][col - 1] = counts[row + 1][col - 1].saturating_add(1);
                    }
                    counts[row + 1][col] = counts[row + 1][col].saturating_add(1);

                    if col + 1 < s.len() {
                        counts[row + 1][col + 1] = counts[row + 1][col + 1].saturating_add(1);
                    }
                }
            }
        }
    });

    counts
        .iter()
        .map(|v| {
            v.iter()
                .map(|count| match *count {
                    0 => String::from(" "),
                    u8::MAX => String::from("*"),
                    _ => count.to_string(),
                })
                .collect()
        })
        .collect()
}
