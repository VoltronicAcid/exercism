pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut numbers = (0..=upper_bound)
        .map(|num| num > 1)
        .collect::<Vec<bool>>();

    (2..upper_bound as usize).for_each(|num| {
        if numbers[num] {
            (num + num..=upper_bound as usize)
                .step_by(num)
                .for_each(|idx| numbers[idx] = false);
        }
    });

    numbers
        .iter()
        .enumerate()
        .filter_map(|(idx, is_prime)| is_prime.then_some(idx as u64))
        .collect()
}
