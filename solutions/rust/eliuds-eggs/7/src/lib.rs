pub fn egg_count(display_value: u32) -> usize {
    format!("{display_value:b}")
        .chars()
        .filter(|ch| *ch == '1')
        .count()
}
