pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut potential_primes = vec![true; 1 + upper_bound as usize];
    potential_primes[0] = false;
    potential_primes[1] = false;

    (2..potential_primes.len()).for_each(|num| {
        if potential_primes[num] {
            for idx in (num + num..potential_primes.len()).step_by(num) {
                potential_primes[idx] = false;
            }
        }
    });

    potential_primes
        .into_iter()
        .enumerate()
        .filter_map(
            |(num, is_prime)| {
                if is_prime {
                    Some(num as u64)
                } else {
                    None
                }
            },
        )
        .collect()
}
