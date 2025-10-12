#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first: &[i32], second: &[i32]) -> Comparison {
    if first == second {
        Comparison::Equal
    } else if first.is_empty() || second.windows(first.len()).any(|window| first == window) {
        Comparison::Sublist
    } else if second.is_empty() || first.windows(second.len()).any(|window| second == window) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}
