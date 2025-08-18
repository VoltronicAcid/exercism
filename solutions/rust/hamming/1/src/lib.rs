pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    let dist = std::iter::zip(s1.chars(), s2.chars())
        .fold(0, |dist, (c1, c2)| dist + (if c1 != c2 { 1 } else { 0 }));

    Some(dist)
}
