pub fn score(word: &str) -> u64 {
    word.chars().fold(0, |score, c|{
        match c.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'l' | 'n' | 'o' | 'r' | 's' | 't' | 'u' => score + 1,
            'd' | 'g' => score + 2,
            'b' | 'c' | 'm' | 'p' => score + 3,
            'f' | 'h' | 'v' | 'w' | 'y' => score + 4,
            'k' => score + 5,
            'j' | 'x' => score + 8,
            'q' | 'z' => score + 10,
            _ => score,
        }
    })
}
