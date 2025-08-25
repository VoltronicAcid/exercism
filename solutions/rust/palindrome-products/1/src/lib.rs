use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    range: std::ops::Range<u64>,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        let sq_rt = (self.value as f64).sqrt() as u64;
        let mut factors: HashSet<(u64, u64)> = HashSet::new();

        for num in self.range.start..=sq_rt {
            if self.value % num == 0 && self.range.contains(&(self.value / num)) {
                factors.insert((num, self.value / num));
            }
        }

        factors
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }

    if let Some(min_palindrome) = get_min_palindrome(min, max) {
        if let Some(max_palindrome) = get_max_palindrome(min, max) {
            return Some((
                Palindrome {
                    value: min_palindrome,
                    range: std::ops::Range {
                        start: min,
                        end: max + 1,
                    },
                },
                Palindrome {
                    value: max_palindrome,
                    range: std::ops::Range {
                        start: min,
                        end: max + 1,
                    },
                },
            ));
        } else {
            return None;
        }
    } else {
        return None;
    }
}

fn get_min_palindrome(min: u64, max: u64) -> Option<u64> {
    let mut min_palindrome = max * max + 1;

    for n1 in min..=max {
        if n1 > min_palindrome {
            break;
        }

        for n2 in min..=max {
            if n1 * n2 > min_palindrome {
                break;
            }
            if is_palindrome(n1 * n2) {
                min_palindrome = n1 * n2;
            }
        }
    }

    if min_palindrome == max * max + 1 {
        None
    } else {
        Some(min_palindrome)
    }
}

fn get_max_palindrome(min: u64, max: u64) -> Option<u64> {
    let mut max_palindrome: u64 = 0;

    for n1 in (min..=max).rev() {
        for n2 in (min..=n1).rev() {
            if n1 * n2 < max_palindrome {
                break;
            } else if is_palindrome(n1 * n2) {
                max_palindrome = n1 * n2;
            }
        }
    }

    if max_palindrome == 0 {
        None
    } else {
        Some(max_palindrome)
    }
}

fn is_palindrome(num: u64) -> bool {
    let mut n = num;
    let mut rev_num = 0;

    while n > 0 {
        rev_num *= 10;
        rev_num += n % 10;
        n /= 10;
    }

    num == rev_num
}
