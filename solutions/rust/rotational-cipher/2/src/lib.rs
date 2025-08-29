pub fn rotate(input: &str, key: u8) -> String {
    input
        .chars()
        .map(|c: char| match c {
            'a'..='z' => {
                let shift = (c as i8 - b'a' as i8 + key as i8) as u8;
                (shift.rem_euclid(26) + b'a') as char
            }
            'A'..='Z' => {
                let shift = (c as i8 - b'A' as i8 + key as i8) as u8;
                (shift.rem_euclid(26) + b'A') as char
            }
            _ => c,
        })
        .collect::<String>()
}
