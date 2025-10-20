pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .chars()
        .try_fold([false; 26], |mut counts, c| {
            match c.to_ascii_lowercase() {
                'a'..='z' => {
                    let lower = c as u8 | b' ';
                    let idx = (lower - b'a') as usize;
                    counts[idx] = true;

                    Some(counts)
                }
                _ => Some(counts),
            }
        })
        .is_some_and(|counts| counts.iter().all(|&b| b))
}
