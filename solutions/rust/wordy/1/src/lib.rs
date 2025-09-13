pub fn answer(command: &str) -> Option<i32> {
    if !command.starts_with("What is") || !command.ends_with("?") {
        return None;
    }

    let tokens = tokenize(remove_words(command));

    if tokens.len() == 1 {
        return tokens[0].parse::<i32>().ok();
    } else if tokens.len() % 2 == 0 {
        return None;
    }

    if let Ok(result) = tokens.first()?.parse::<i32>() {
        return tokens[1..].chunks(2).try_fold(result, |acc, chunk| {
            let num_str = match chunk[1].parse::<i32>() {
                Ok(_) => chunk[1].to_string(),
                Err(_) => chunk[1]
                    .chars()
                    .take_while(|c| c.is_numeric())
                    .collect::<String>(),
            };

            let Ok(num) = num_str.as_str().parse::<i32>() else {
                return None;
            };

            match chunk[0].as_str() {
                "plus" => Some(acc + num),
                "minus" => Some(acc - num),
                "multiplied" => Some(acc * num),
                "divided" => Some(acc / num),
                "raised" => Some(acc.pow(num as u32)),
                _ => None,
            }
        });
    }

    None
}

fn remove_words(command: &str) -> String {
    command[..command.len() - 1]
        .replace("by", "")
        .replace("power", "")
        .replace("to the", "")
}

fn tokenize(words: String) -> Vec<String> {
    words
        .split_whitespace()
        .skip(2)
        .filter(|&s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}
