pub fn nth(n: u32) -> u32 {
    let two = 2..3;
    let odds = (3..).step_by(2);

    two.chain(odds)
        .filter(|p| !(2..=(*p as f64).sqrt() as u32).any(|d| p % d == 0))
        .nth(n as usize)
        .unwrap()
}
