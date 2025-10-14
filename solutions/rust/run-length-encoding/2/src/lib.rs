pub fn encode(msg: &str) -> String {
    if let Some(first_char) = msg.chars().next() {
        let count = msg.chars().take_while(|ch| *ch == first_char).collect::<String>().len();
        match count {
            1 =>  format!("{first_char}{}", encode(&msg[count..])),
            _ =>  format!("{count}{first_char}{}", encode(&msg[count..]))
        }
    } else {
        msg.to_string()
    }
}

pub fn decode(msg: &str) -> String {
    msg
        .split_inclusive(|c: char| !c.is_ascii_digit())
        .map(|s| {
            let (count, letter) = s.split_at(s.len() - 1);
            let count = count.parse::<usize>().unwrap_or(1);

            letter.to_string().repeat(count)
        })
        .collect()
}