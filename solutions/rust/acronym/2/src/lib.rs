pub fn abbreviate(phrase: &str) -> String {
    let separators = [' ', '-', '_'];
    let is_first_char = |p: char, c: char| -> bool { c.is_alphabetic() && separators.contains(&p) };
    let is_camel_case = |p: char, c: char| -> bool { p.is_lowercase() && c.is_uppercase() };
    let phrase_chars: Vec<char> = phrase.chars().collect();

    phrase_chars
        .windows(2)
        .enumerate()
        .filter_map(|(idx, window)| {
            let prev = window[0];
            let curr = window[1];
            let upper_prev = prev.to_string().to_uppercase();
            let upper_curr = curr.to_string().to_uppercase();

            match idx {
                0 => Some(upper_prev),
                _ => match is_first_char(prev, curr) || is_camel_case(prev, curr) {
                    true => Some(upper_curr),
                    false => None,
                },
            }
        })
        .collect()
}
