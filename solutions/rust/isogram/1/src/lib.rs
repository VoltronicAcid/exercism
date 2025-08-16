pub fn check(candidate: &str) -> bool {
    let mut counts: [usize; 128] = [0; 128];
    candidate.chars().for_each(|ch| {
        if ch.is_alphabetic() {
            let char_code = if ch.is_lowercase() {
                ch as usize
            } else {
                32 + ch as usize
            };
            counts[char_code] += 1;
        }
    });

    counts.iter().all(|count| *count < 2)
}
