pub fn encrypt(input: &str) -> String {
    let normalized: Vec<char> = input
        .chars()
        .filter_map(|c| c.is_ascii_alphanumeric().then_some(c.to_ascii_lowercase()))
        .collect();

    match normalized.len() {
        0 => String::new(),
        1 => normalized.iter().collect(),
        _ => {
            let columns = (normalized.len() as f64).sqrt().ceil() as usize;
            let chunks = normalized.chunks(columns).collect::<Vec<_>>();

            (0..columns)
                .map(|col| {
                    (0..chunks.len())
                        .map(|row| chunks[row].get(col).unwrap_or(&' '))
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
                .join(" ")
        }
    }
}
