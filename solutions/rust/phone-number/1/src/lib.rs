pub fn number(user_number: &str) -> Option<String> {
    let digits = user_number
        .chars()
        .filter(|&c| c.is_digit(10))
        .collect::<String>();

    match digits.len() {
        10 => return number(&("1".to_owned() + &digits)),
        11 => (number_has_valid_codes(&digits)).then_some(digits[1..].into()),
        _ => None,
    }
}

fn number_has_valid_codes(number: &str) -> bool {
    number.starts_with("1")
        && 0 == number
            .char_indices()
            .filter(|(idx, c)| (*idx == 1 || *idx == 4) && (*c == '1' || *c == '0'))
            .count()
}
