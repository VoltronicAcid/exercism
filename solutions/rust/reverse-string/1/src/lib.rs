pub fn reverse(input: &str) -> String {
    let mut result: String = String::new();

    for char in input.chars().rev() {
        result += &char.to_string();
    }

    result
}
