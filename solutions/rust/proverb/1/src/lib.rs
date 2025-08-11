pub fn build_proverb(list: &[&str]) -> String {
    let mut lines: Vec<String> = Vec::new();
    let Some(first) = list.get(0) else {
        return String::from("");
    };
    let last = format!("And all for the want of a {}.", first);

    for window in list.windows(2) {
        match *window {
            [prev, curr] => {
                lines.push(format!("For want of a {} the {} was lost.", prev, curr,));
            }
            _ => break,
        }
    }

    lines.push(last);
    lines.join("\n")
}
