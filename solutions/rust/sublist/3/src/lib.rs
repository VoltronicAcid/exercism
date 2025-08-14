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
        Ordering::Equal => {
            if first == second {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
        Ordering::Less => {
            if first.is_empty() || second.windows(first.len()).any(|slc| first == slc) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
        Ordering::Greater => {
            if second.is_empty() || first.windows(second.len()).any(|slc| second == slc) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
    }
}
