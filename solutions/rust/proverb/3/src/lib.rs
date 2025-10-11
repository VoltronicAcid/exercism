pub fn build_proverb(list: &[&str]) -> String {
    if let Some(first) = list.first() {
        list.windows(2)
            .map(|win| format!("For want of a {} the {} was lost.", win[0], win[1]))
            .chain(std::iter::once(format!(
                "And all for the want of a {first}."
            )))
            .collect::<Vec<String>>()
            .join("\n")
    } else {
        "".into()
    }
}
