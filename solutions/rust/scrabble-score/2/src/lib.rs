/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.chars()
        .map(|ch| match ch.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'l' | 'n' | 'o' | 'r' | 's' | 't' | 'u' => 1,
            'd' | 'g' => 2,
            'b' | 'c' | 'm' | 'p' => 3,
            'f' | 'h' | 'v' | 'w' | 'y' => 4,
            'k' => 5,
            'j' | 'x' => 8,
            'q' | 'z' => 10,
            _ => 0,
        })
        .sum()
}
