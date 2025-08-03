pub fn egg_count(display_value: u32) -> usize {
    (0..32).fold(0, |count: usize, x: u32| {
        count
            + if 2_u32.pow(x) & display_value > 0 {
                1
            } else {
                0
            }
    })
}
