pub fn get_diamond(c: char) -> Vec<String> {
    ('A'..c).chain(('A'..=c).rev()).map(|letter| {
        let char_code = letter as u8;
        let outer = " ".repeat((c as u8 - char_code).into());
        let inner = " ".repeat(((char_code - b'A') * 2).saturating_sub(1).into());

        match inner.len() {
            0 => format!("{outer}{letter}{outer}"),
            _ => format!("{outer}{letter}{inner}{letter}{outer}"),
        }
    }).collect()
}
