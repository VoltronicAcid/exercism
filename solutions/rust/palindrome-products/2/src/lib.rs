use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    number: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn new(number: u64) -> Option<Palindrome> {
        let mut val = number;
        let mut rev = 0_u64;

        while val > 0 {
            let rev_shifted = rev.checked_mul(10)?;
            (rev, val) = (rev_shifted + val % 10, val / 10);
        }

        if rev == number {
            Some(Palindrome{
                number, factors: HashSet::new()
            })
        } else {
            None
        }
    }
    pub fn value(&self) -> u64 {
        self.number
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }
}

pub fn palindrome_products(start: u64, end: u64) -> Option<(Palindrome, Palindrome)> {
    use std::cmp::{min, max};
    use std::cmp::Ordering::*;

    let mut min_pal = Palindrome::new(18446744066044764481_u64)?;
    for factor1 in start..=end {
        for factor2 in factor1..=end {
            let number = factor1 * factor2;
            match number.cmp(&min_pal.value()) {
                Less => {
                    if let Some(pal) = Palindrome::new(number) {
                        pal.clone_into(&mut min_pal);
                        min_pal.factors.insert((min(factor1, factor2), max(factor1, factor2)));
                    }
                },
                Equal => { min_pal.factors.insert((min(factor1, factor2), max(factor1, factor2))); }
                Greater => break,
            }
        }
    }

    let mut max_pal = Palindrome::new(0)?;
    for factor1 in (start..=end).rev() {
        for factor2 in (start..=factor1).rev() {
            let number = factor1 * factor2;
            match number.cmp(&max_pal.value()) {
                Less => break,
                Equal => { max_pal.factors.insert((min(factor1, factor2), max(factor1, factor2))); }
                Greater => {
                    if let Some(pal) = Palindrome::new(number) {
                        pal.clone_into(&mut max_pal);
                        max_pal.factors.insert((min(factor1, factor2), max(factor1, factor2)));
                    }
                },
            }
        }
    }

    match (min_pal.value(), max_pal.value()) {
        (18446744066044764481_u64, 0) => None,
        _ => Some((min_pal, max_pal)),
    }
}
