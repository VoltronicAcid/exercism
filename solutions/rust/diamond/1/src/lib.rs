pub fn get_diamond(c: char) -> Vec<String> {
    if c == 'A' {
        return vec![String::from("A")];
    }

    let offset = (c as u8 - 'A' as u8) as usize;
    let mut lines: Vec<String> = Vec::new();

    for (idx, ch) in ('A'..=c).enumerate() {
        let outer = String::new() + &String::from(" ".repeat(offset - idx));
        let inner = String::new() + &String::from(" ".repeat((idx * 2).saturating_add_signed(-1)));

        let ins_idx = lines.len() / 2;
        if idx == 0 {
            lines.push(format!("{outer}A{outer}"));
            lines.push(format!("{outer}A{outer}"));
        } else if ch == c {
            lines.insert(ins_idx, format!("{ch}{inner}{ch}"));
        } else {
            lines.insert(ins_idx, format!("{outer}{ch}{inner}{ch}{outer}"));
            lines.insert(ins_idx, format!("{outer}{ch}{inner}{ch}{outer}"));
        }
    }

    lines
}
