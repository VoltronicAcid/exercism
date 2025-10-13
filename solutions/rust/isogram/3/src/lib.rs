pub fn check(candidate: &str) -> bool {
    let mut letters = std::collections::HashSet::new();

    candidate
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .all(|c| letters.insert(c))
}
