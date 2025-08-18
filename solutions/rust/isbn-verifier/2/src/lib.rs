/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let digits: Vec<u32> = isbn
        .replace("-", "")
        .chars()
        .enumerate()
        .filter_map(|(idx, ch)| {
            if ch.is_numeric() {
                let digit: u32 = char::to_digit(ch, 10).unwrap();

                Some(digit * (10 - idx as u32))
            } else if idx == 9 && (ch == 'x' || ch == 'X') {
                Some(10)
            } else {
                None
            }
        })
        .collect();

    (isbn.len() == 13 || isbn.len() == 10)
        && digits.len() == 10
        && digits.iter().sum::<u32>() % 11 == 0
}
