pub fn rotate(input: &str, key: u8) -> String {
    input
        .chars()
        .map(|c| {
            match c {
                'A'..='Z' => {
                    let shift = (c as u8 - b'A' + key).rem_euclid(26);
                    char::from(shift + b'A')
                },
                'a'..='z' => {
                    let shift = (c as u8 - b'a' + key).rem_euclid(26);
                    char::from(shift + b'a')
                },
                _ => c,
            }
        })
        .collect::<String>()
}
