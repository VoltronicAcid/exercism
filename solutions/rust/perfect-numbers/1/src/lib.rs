use std::cmp::Ordering::{Equal, Greater, Less};

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num < 1 {
        return None;
    }

    match aliquot_sum(num).cmp(&num) {
        Less => Some(Classification::Deficient),
        Equal => Some(Classification::Perfect),
        Greater => Some(Classification::Abundant),
    }
}

fn aliquot_sum(num: u64) -> u64 {
    (1..num).filter(|n| num % *n == 0).sum::<u64>()
}
