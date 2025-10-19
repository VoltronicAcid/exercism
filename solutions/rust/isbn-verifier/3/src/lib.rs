const VALID_CHARS: [char; 11] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'X'];
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.len() != 10 && isbn.len() != 13 { return false; }

    let valid_chars = isbn
        .chars()
        .filter(|&c| VALID_CHARS.contains(&c))
        .collect::<Vec<char>>();
    if valid_chars.len() != 10 { return false; }

    let mut checksum = 0;
    for (idx, c) in valid_chars.iter().enumerate() {
        match idx {
            0..9 => {
                if c.is_ascii_digit() {
                    checksum += c.to_digit(10).unwrap() * (10 - idx as u32);
                } else {
                    return false;
                }
                
            }
            9 => {
                if c.is_ascii_digit() {
                    checksum += c.to_digit(10).unwrap() * (10 - idx as u32);
                } else if *c == 'X' {
                    checksum += 10;
                } else {
                    return false;
                }
            }
            _ => return false,
        }
    }

    checksum % 11 == 0
}
