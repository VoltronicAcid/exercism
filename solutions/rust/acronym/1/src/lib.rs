pub fn abbreviate(phrase: &str) -> String {
    let separators = [' ', '-', '_'];

    phrase
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .enumerate()
        .filter_map(|(idx, window)| {
            let prev = window[0];
            let curr = window[1];

            if idx == 0 {
                Some(prev.to_uppercase().collect::<String>())
            } else if curr.is_alphabetic() && separators.contains(&prev) {
                Some(curr.to_uppercase().collect::<String>())
            } else if curr.is_uppercase() && prev.is_lowercase() {
                Some(curr.to_uppercase().collect::<String>())
            } else {
                None
            }
        })
        .collect()
}
