const VALID_CHARS: [char; 11] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'X'];
pub fn is_valid_isbn(isbn: &str) -> bool {
    let verify_length = |s: &str| {
        (s.len() == 10 || s.len() == 13)
            && s.chars().filter(|c| VALID_CHARS.contains(c)).count() == 10
    };

    let verify_digit_positions = |s: &str| {
        s.chars()
            .filter(|c| VALID_CHARS.contains(c))
            .take(9)
            .all(|c| c.is_ascii_digit())
    };

    let has_valid_checksum_char = |s: &str| {
        s.chars()
            .last()
            .is_some_and(|c| c.is_ascii_digit() || c == 'X')
    };

    let checksum = isbn
        .chars()
        .filter(|&c| VALID_CHARS.contains(&c))
        .enumerate()
        .map(|(idx, c)| {
            if c.is_ascii_digit() {
                (10 - idx as u32) * c.to_digit(10).unwrap()
            } else {
                10_u32
            }
        })
        .sum::<u32>();
    let valid_checksum = checksum % 11 == 0;

    let check_functions: Vec<fn(&str) -> bool> = vec![
        verify_length,
        verify_digit_positions,
        has_valid_checksum_char,
    ];

    check_functions.iter().all(|&f| f(isbn)) && valid_checksum
}
