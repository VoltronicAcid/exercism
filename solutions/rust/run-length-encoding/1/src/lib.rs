pub fn encode(source: &str) -> String {
    let mut encoded = String::from("");
    if source.is_empty() {
        return encoded;
    }

    let mut curr: char = source.as_bytes()[0] as char;
    let mut count = 0;

    source.chars().for_each(|ch| {
        if ch == curr {
            count += 1;
        } else {
            if count > 1 {
                encoded = format!("{}{}{}", encoded, count, curr);
            } else {
                encoded = format!("{}{}", encoded, curr);
            }
            curr = ch;
            count = 1;
        }
    });
    if count > 1 {
        encoded = format!("{}{}{}", encoded, count, curr);
    } else {
        encoded = format!("{}{}", encoded, curr);
    }

    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::from("");
    if source.is_empty() {
        return decoded;
    }

    let mut num = 0;

    source.chars().for_each(|ch| {
        if ch.is_numeric() {
            num *= 10;
            num += ch.to_digit(10).unwrap();
        } else {
            if num > 1 {
                decoded = format!("{}{}", decoded, ch.to_string().repeat(num as usize));
                num = 0;
            } else {
                decoded = format!("{}{}", decoded, ch);
            }
        }
    });

    decoded
}
