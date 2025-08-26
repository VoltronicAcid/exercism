pub fn encrypt(input: &str) -> String {
    let normalized = normalize(input.to_string());
    if normalized.is_empty() {
        return String::new();
    }
    let (word_count, word_len) = get_count_and_len(&normalized);

    normalized
        .iter()
        .enumerate()
        .fold(Vec::<String>::new(), |mut acc, (i, c)| {
            let idx = i % word_count;
            if let Some(word) = acc.get(idx) {
                acc[idx] = format!("{word}{c}");
            } else {
                acc.push(c.to_string());
            }

            acc
        })
        .iter_mut()
        .map(|word| {
            while word.len() < word_len {
                word.push(' ');
            }

            word.clone()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn normalize(mut input: String) -> Vec<char> {
    input.make_ascii_lowercase();
    input
        .chars()
        .filter(|&c| c.is_ascii_alphanumeric())
        .collect::<Vec<char>>()
}

fn get_count_and_len(input: &[char]) -> (usize, usize) {
    let count: usize = ((input.len() as f64).sqrt()).ceil() as usize;
    let len: usize = if count * (count - 1) >= input.len() {
        count - 1
    } else {
        count
    };

    (count, len)
}
