pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    match s1.len().cmp(&s2.len()) {
        std::cmp::Ordering::Equal => {
            std::iter::zip(s1.chars(), s2.chars())
                .try_fold(0_usize, |diffs, (c1, c2)| match c1 == c2 {
                    true => Some(diffs),
                    false => Some(diffs + 1),
                })
        }
        _ => None,
    }
}
