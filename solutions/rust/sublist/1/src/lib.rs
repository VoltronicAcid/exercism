#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first: &[i32], second: &[i32]) -> Comparison {
    let f_len = first.len();
    let s_len = second.len();

    if f_len == s_len {
        return if first == second {
            Comparison::Equal
        } else {
            Comparison::Unequal
        };
    } else if f_len < s_len {
        for idx in 0..=s_len - f_len {
            if *first == second[idx..idx + f_len] {
                return Comparison::Sublist;
            }
        }
        return Comparison::Unequal;
    } else {
        for idx in 0..=f_len - s_len {
            if *second == first[idx..idx + s_len] {
                return Comparison::Superlist;
            }
        }
        return Comparison::Unequal;
    }
}
