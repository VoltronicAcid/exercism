pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let exp = num_str.len() as u32;

    num == num_str
        .chars()
        .filter_map(|ch| ch.to_digit(10).map(|n| n.pow(exp)))
        .sum()
}
