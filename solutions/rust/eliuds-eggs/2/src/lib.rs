pub fn egg_count(display_value: u32) -> usize {
    (0..32)
        .map(|x: u32| {
            if 2_u32.pow(x) & display_value > 0 {
                1
            } else {
                0
            }
        })
        .sum()
}
