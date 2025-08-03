use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set: HashSet<u32> = HashSet::new();

    for num in factors {
        let n = *num;
        
        if n == 0 {
            continue;
        }
        
        if n == 1 {
            return (limit * (limit - 1)) / 2;
        }
        
        set.extend((n..limit).step_by(n as usize));
    }

    if set.len() > 0 { set.iter().sum() } else { 0 }
}
