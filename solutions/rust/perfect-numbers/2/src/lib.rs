use crate::Classification::*;
use std::cmp::Ordering::*;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    let root = (num as f64).sqrt() as u64;
    if root < 2 {
        return Some(Deficient);
    }

    let sum = (2..=root)
        .filter_map(|factor| {
            num.is_multiple_of(factor)
                .then_some(HashSet::<u64>::from_iter([factor, num / factor]))
        })
        .flatten()
        .sum::<u64>();

    match sum.cmp(&(num - 1)) {
        Less => Some(Deficient),
        Equal => Some(Perfect),
        Greater => Some(Abundant),
    }
}
