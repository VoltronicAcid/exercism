pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    (s1.len() == s2.len()).then(|| {
        std::iter::zip(s1.chars(), s2.chars()).fold(
            0,
            |diffs, (c1, c2)| if c1 == c2 { diffs } else { diffs + 1 },
        )
    })
}
