pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    if factors.contains(&1) {
        return (limit * (limit - 1)) / 2;
    }

    (1..limit)
        .filter(|num| factors.iter().any(|f| *f > 1 && num % f == 0))
        .sum::<u32>()
}
