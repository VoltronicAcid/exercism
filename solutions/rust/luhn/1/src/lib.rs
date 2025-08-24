/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.chars().any(|ch| !ch.is_numeric() && ch != ' ') {
        return false;
    }

    let digits = code
        .chars()
        .filter(|&ch| ch.is_numeric())
        .map(|ch| char::to_digit(ch, 10).unwrap())
        .collect::<Vec<u32>>();

    if digits.len() < 2 {
        return false;
    }

    let luhn_map = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9];
    let mut luhn_sum = 0;
    for (idx, &digit) in digits.iter().enumerate() {
        if idx % 2 == digits.len() % 2 {
            luhn_sum += luhn_map[digit as usize];
        } else {
            luhn_sum += digit;
        }
    }

    luhn_sum % 10 == 0
}
