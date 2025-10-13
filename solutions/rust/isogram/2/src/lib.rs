pub fn check(candidate: &str) -> bool {
    let mut counts = [0; 26];

    candidate.chars().for_each(|c| {
        c.is_alphabetic().then(|| {
            let idx = ((c as u8 & 31) - 1) as usize;

            counts[idx] += 1
        });
    });

    counts.iter().all(|count| *count < 2)
}
