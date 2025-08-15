pub fn annotate(garden: &[&str]) -> Vec<String> {
    let height = garden.len();
    (0..height)
        .map(|row| {
            let width = garden[row].len();
            (0..width)
                .map(|col| match garden[row].as_bytes()[col] {
                    b'*' => '*',
                    b' ' => count_neighbors(garden, row, col),
                    _ => garden[row].chars().nth(col).unwrap(),
                })
                .collect()
        })
        .collect()
}

fn count_neighbors(garden: &[&str], row: usize, col: usize) -> char {
    let offsets: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let count = offsets
        .into_iter()
        .filter_map(|(dy, dx)| {
            let (row, col) = (row.wrapping_add_signed(dy), col.wrapping_add_signed(dx));
            if row < garden.len() && col < garden[0].len() && garden[row].as_bytes()[col] == b'*' {
                Some(1)
            } else {
                None
            }
        })
        .sum();

    match count {
        0 => ' ',
        num => char::from_digit(num, 10).unwrap(),
    }
}
