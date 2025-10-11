pub fn factors(num: u64) -> Vec<u64> {
    if num < 2 {
        return vec![];
    }

    let root = (num as f64).sqrt().ceil() as u64;
    std::iter::once(2_u64)
        .chain((3..=root).step_by(2))
        .find(|&factor| num % factor == 0)
        .map_or_else(
            || vec![num],
            |factor| [vec![factor], factors(num / factor)].concat(),
        )
}
