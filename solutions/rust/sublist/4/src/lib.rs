use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first: &[i32], second: &[i32]) -> Comparison {
    match first.len().cmp(&second.len()) {
        Ordering::Equal => compare(first, second, Comparison::Equal),
        Ordering::Less => compare(first, second, Comparison::Sublist),
        Ordering::Greater => compare(second, first, Comparison::Superlist),
    }
}

fn compare(shrt: &[i32], long: &[i32], success: Comparison) -> Comparison {
    if shrt.is_empty() || shrt == long || long.windows(shrt.len()).any(|sublist| shrt == sublist) {
        success
    } else {
        Comparison::Unequal
    }
}
