pub fn reply(message: &str) -> &str {
    let question = is_question(message);
    let yelling = is_yelling(message);
    let silence = is_silence(message);

    if question && yelling {
        "Calm down, I know what I'm doing!"
    } else if question {
        "Sure."
    } else if yelling {
        "Whoa, chill out!"
    } else if silence {
        "Fine. Be that way!"
    } else {
        "Whatever."
    }
}

fn is_question(msg: &str) -> bool {
    let trimmed = msg.trim();
    let mut idx = trimmed.len();
    if idx == 0 {
        false
    } else {
        idx -= 1;
        trimmed.get(idx..).unwrap() == "?"
    }
}

fn is_yelling(msg: &str) -> bool {
    let letters: Vec<char> = msg.chars().filter(|c| c.is_alphabetic()).collect();

    letters.len() > 0 && letters.iter().all(|c| c.is_ascii_uppercase())
}

fn is_silence(msg: &str) -> bool {
    msg.chars().all(|c| c.is_whitespace())
}
