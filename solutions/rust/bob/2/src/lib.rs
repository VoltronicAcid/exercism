pub fn reply(message: &str) -> &str {
    let responses = [
        "Whatever.",
        "Sure.",
        "Whoa, chill out!",
        "Calm down, I know what I'm doing!",
        "Fine. Be that way!",
    ];

    let mut idx: usize = 0;
    idx += if is_question(message) { 1 } else { 0 };
    idx += if is_yelling(message) { 2 } else { 0 };
    idx = if is_silence(message) { 4 } else { idx };

    responses[idx]
}

fn is_silence(msg: &str) -> bool {
    msg.trim().is_empty()
}

fn is_question(msg: &str) -> bool {
    msg.trim().ends_with("?")
}

fn is_yelling(msg: &str) -> bool {
    let letters: Vec<char> = msg.chars().filter(|c| c.is_alphabetic()).collect();

    !letters.is_empty() && letters.iter().all(|c| c.is_ascii_uppercase())
}
