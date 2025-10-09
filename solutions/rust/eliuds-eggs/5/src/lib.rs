pub fn egg_count(display_value: u32) -> usize {
    format!("{display_value:b}")
        .chars()
        .filter_map(|ch| if ch == '1' { Some(1) } else { None })
        .count()
}
