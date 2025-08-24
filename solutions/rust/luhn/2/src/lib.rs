/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.chars().any(|ch| !ch.is_numeric() && ch != ' ') {
        return false;
    }

    let luhn_map: [u32; 10] = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9];
    let (count, sum) = code
        .chars()
        .filter_map(|ch| char::to_digit(ch, 10))
        .rev()
        .fold((0, 0), |acc: (u32, u32), num| {
            if acc.0 % 2 == 1 {
                (acc.0 + 1, acc.1 + luhn_map[num as usize])
            } else {
                (acc.0 + 1, acc.1 + num)
            }
        });

    count > 1 && sum % 10 == 0
}
