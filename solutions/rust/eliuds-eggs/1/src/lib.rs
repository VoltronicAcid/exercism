pub fn egg_count(display_value: u32) -> usize {
    let base: u32 = 2;
    let mut count: usize = 0;
    for exp in 0..32 {
        if base.pow(exp) & display_value > 0 {
            count += 1;
        }
    }

    count
}
