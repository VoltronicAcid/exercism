pub fn reply(message: &str) -> &str {
    let responses = [
        "Whatever.",
        "Sure.",
        "Whoa, chill out!",
        "Calm down, I know what I'm doing!",
    ];

    let msg = message.trim();
    if msg.is_empty() {
        "Fine. Be that way!"
    } else {
        responses[is_question(msg) + is_yelling(msg)]
    }
}

fn is_question(msg: &str) -> usize {
    match msg.ends_with("?") {
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
