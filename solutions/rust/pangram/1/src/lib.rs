/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut counts = [0; 26];
    sentence.chars().for_each(|ch| {
        if ch.is_ascii() && ch.is_alphabetic() {
            let idx = ch.to_ascii_lowercase() as usize - 'a' as usize;
            counts[idx] = 1;
        }
    });

    counts.iter().all(|count| *count > 0)
}
