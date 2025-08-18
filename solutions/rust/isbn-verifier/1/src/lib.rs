/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let is_all_valid_chars = isbn
        .chars()
        .all(|ch| ch.is_numeric() || ch == 'X' || ch == 'x' || ch == '-');
    let digits = isbn.replace("-", "");

    if digits.len() != 10 || !is_all_valid_chars {
        return false;
    }

    if digits.chars().find(|&ch| ch == 'x' || ch == 'X').is_some() {
        if let Some(pos) = digits.chars().position(|ch| ch == 'x' || ch == 'X') {
            if pos != 9 {
                return false;
            }
        }
    }

    let checksum = digits.chars().enumerate().fold(0, |total, (idx, ch)| {
        let mult: u32 = (10 - idx).try_into().unwrap();
        match ch {
            ch if char::to_digit(ch, 10).is_some() => {
                let digit = char::to_digit(ch, 10).unwrap();

                digit * mult + total
            }
            'X' | 'x' => 10 * mult + total,
            _ => total,
        }
    });

    checksum % 11 == 0
}
