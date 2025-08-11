pub fn build_proverb(list: &[&str]) -> String {
    let Some(first) = list.get(0) else {
        return String::from("");
    };
    let last = format!("And all for the want of a {}.", first);

    let mut lines: Vec<String> = list
        .windows(2)
        .map(|win| format!("For want of a {} the {} was lost.", win[0], win[1]))
        .collect();
    lines.push(last);

    lines.join("\n")
}
