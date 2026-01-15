pub fn annotate(garden: &[&str]) -> Vec<String> {
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

    let height = garden.len();
    (0..height)
        .map(|row| {
            let width = garden[row].len();
            (0..width)
                .map(|col| match garden[row].as_bytes()[col] {
                    b'*' => String::from("*"),
                    _ => match offsets
                        .into_iter()
                        .map(|(dy, dx)| (row.wrapping_add_signed(dy), col.wrapping_add_signed(dx)))
                        .filter(|&(r, c)| {
                            r < height && c < width && garden[r].as_bytes()[c] == b'*'
                        })
                        .count()
                    {
                        0 => String::from(" "),
                        num => num.to_string(),
                    },
                })
                .collect()
        })
        .collect()
}
