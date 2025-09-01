pub fn egg_count(display_value: u32) -> usize {
    (0..32).fold(0, |count: usize, exp: u32| {
        match 2_u32.pow(exp) & display_value {
            0 => count,
            _ => count + 1,
        }
    })
}
