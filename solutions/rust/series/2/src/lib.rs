pub fn series(digits: &str, len: usize) -> Vec<String> {
    digits
        .as_bytes()
        .windows(len)
        .filter_map(|win| String::from_utf8(Vec::from(win)).ok())
        .collect()
}
