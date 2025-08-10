pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() {
        Vec::new()
    } else {
        (0..=(digits.len() - len))
            .map(|idx| digits[idx..(idx + len)].to_string())
            .collect()
    }
}
