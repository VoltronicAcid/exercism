use std::collections::HashMap;

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let letter_values = HashMap::from([
        ('a', 1),
        ('b', 3),
        ('c', 3),
        ('d', 2),
        ('e', 1),
        ('f', 4),
        ('g', 2),
        ('h', 4),
        ('i', 1),
        ('j', 8),
        ('k', 5),
        ('l', 1),
        ('m', 3),
        ('n', 1),
        ('o', 1),
        ('p', 3),
        ('q', 10),
        ('r', 1),
        ('s', 1),
        ('t', 1),
        ('u', 1),
        ('v', 4),
        ('w', 4),
        ('x', 8),
        ('y', 4),
        ('z', 10),
    ]);

    word.chars()
        .map(|ch| {
            if let Some(val) = letter_values.get(&ch.to_ascii_lowercase()) {
                if ch.is_ascii() && ch.is_alphabetic() {
                    return *val;
                }
            }

            return 0;
        })
        .sum()
}
