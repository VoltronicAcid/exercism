pub fn translate(input: &str) -> String {
    input
        .split_ascii_whitespace()
        .map(translate_word)
        .collect::<Vec<String>>()
        .join(" ")
}

fn translate_word(input: &str) -> String {
    let suffix = "ay";

    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
    let prefixes = ["xr", "yt"];

    if input.starts_with(&vowels[..5]) || prefixes.iter().any(|&pre| input.starts_with(pre)) {
        return format!("{input}{suffix}");
    } else if input.starts_with(&vowels[5..]) {
        return format!("{}yay", &input[1..]);
    } else if let Some(pos) = input.find("qu") {
        if pos <= 1 {
            return format!("{}{}{suffix}", &input[(pos + 2)..], &input[0..(pos + 2)]);
        }
    }

    let consonants = input
        .chars()
        .take_while(|ch| !vowels.contains(ch))
        .collect::<String>();

    format!(
        "{}{consonants}{suffix}",
        input.chars().skip(consonants.len()).collect::<String>()
    )
}
