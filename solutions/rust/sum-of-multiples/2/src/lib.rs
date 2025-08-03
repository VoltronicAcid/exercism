use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set: HashSet<u32> = HashSet::new();

    for num in factors {
        let n: u32 = *num;

        if n == 0 {
            continue;
        }

        if n == 1 {
            return (limit * (limit - 1)) / 2;
        }

        set.extend((n..limit).step_by(n as usize));
    }

    if set.is_empty() { 0 } else { set.iter().sum() }
}
