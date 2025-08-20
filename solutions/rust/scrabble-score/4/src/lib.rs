/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.chars().fold(0, letter_values)
}

fn letter_values(sum: u64, character: char) -> u64 {
    match character.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'l' | 'n' | 'o' | 'r' | 's' | 't' | 'u' => sum + 1,
        'd' | 'g' => sum + 2,
        'b' | 'c' | 'm' | 'p' => sum + 3,
        'f' | 'h' | 'v' | 'w' | 'y' => sum + 4,
        'k' => sum + 5,
        'j' | 'x' => sum + 8,
        'q' | 'z' => sum + 10,
        _ => sum,
    }
}
