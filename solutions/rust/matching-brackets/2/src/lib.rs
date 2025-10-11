pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for curr in string.chars().filter(|ch| "(){}[]".contains(*ch)) {
        if matches!(curr, '(' | '[' | '{') {
            stack.push(curr);
            continue;
        }

        if let Some(prev) = stack.pop() {
            match (prev, curr) {
                ('(', ')') | ('[', ']') | ('{', '}') => continue,
                _ => return false,
            }
        } else {
            return false;
        }
    }

    stack.is_empty()
}
