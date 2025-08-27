use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .split(&[' ', '\n', '\t', ','])
        .filter_map(|word| {
            let key = word
                .trim_matches(|c: char| !c.is_ascii_alphanumeric())
                .to_ascii_lowercase();
            match key.is_empty() {
                true => None,
                false => Some(key),
            }
        })
        .fold(HashMap::<String, u32>::new(), |mut acc, word| {
            acc.entry(word).and_modify(|count| *count += 1).or_insert(1);

            acc
        })
}
