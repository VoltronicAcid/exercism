pub fn reply(msg: &str) -> &str {
    let responses = [
        "Whatever.",
        "Sure.",
        "Whoa, chill out!",
        "Calm down, I know what I'm doing!",
        "Fine. Be that way!",
    ];

    if is_silence(msg) {
        responses[4]
    } else {
        responses[is_question(msg) + is_yelling(msg)]
    }
}

fn is_silence(msg: &str) -> bool {
    msg.trim().is_empty()
}

fn is_question(msg: &str) -> usize {
    match msg.trim().ends_with("?") {
        true => 1,
        false => 0,
    }
}

fn is_yelling(msg: &str) -> usize {
    let letters: Vec<char> = msg.chars().filter(|c| c.is_alphabetic()).collect();

    match !letters.is_empty() && letters.iter().all(char::is_ascii_uppercase) {
        true => 2,
        false => 0,
    }
}
