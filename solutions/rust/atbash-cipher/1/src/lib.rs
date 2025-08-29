/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter_map(encode_char)
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars().filter_map(encode_char).collect::<String>()
}

fn encode_char(c: char) -> Option<char> {
    if c.is_ascii_alphabetic() {
        Some(((25 - (c.to_ascii_lowercase() as u8 - b'a').rem_euclid(26)) + b'a') as char)
    } else if c.is_ascii_digit() {
        Some(c)
    } else {
        None
    }
}
