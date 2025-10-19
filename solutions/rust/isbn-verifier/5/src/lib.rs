pub fn is_valid_isbn(isbn: &str) -> bool {
    isbn.chars()
        .filter(|&c| c != '-')
        .enumerate()
        .try_fold((0_u32, 0_u32), |(checksum, length), (idx, c)| {
            match (idx, c) {
                (0..=9, '0'..='9') => Ok((
                    checksum + c.to_digit(10).unwrap() * (10 - idx as u32),
                    length + 1,
                )),
                (9, 'X') => Ok((checksum + 10, length + 1)),
                _ => Err(()),
            }
        })
        .is_ok_and(|(checksum, length)| checksum % 11 == 0 && length == 10)
}
