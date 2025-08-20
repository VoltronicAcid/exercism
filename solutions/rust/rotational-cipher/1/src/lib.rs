pub fn rotate(input: &str, key: u8) -> String {
    let lower = "abcdefghijklmnopqrstuvwxyz";
    let upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    input
        .chars()
        .map(|ch: char| {
            if let Some(pos) = lower.find(ch) {
                return lower.as_bytes()[(pos + key as usize) % 26] as char;
            } else if let Some(pos) = upper.find(ch) {
                return upper.as_bytes()[(pos + key as usize) % 26] as char;
            }

            ch
        })
        .collect::<String>()
}
