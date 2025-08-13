pub fn factors(n: u64) -> Vec<u64> {
    if n < 2 {
        return Vec::new();
    }

    let mut prime_factors: Vec<u64> = Vec::new();
    let mut n = n;
    while n % 2 == 0 {
        prime_factors.push(2);
        n /= 2;
    }

    if !prime_factors.is_empty() {
        prime_factors.extend(factors(n));
        return prime_factors;
    }

    let sq_rt = (n as f64).sqrt().ceil() as u64;
    (3..=sq_rt)
        .step_by(2)
        .find(|i| n % i == 0)
        .map_or_else(|| vec![n], |i| [vec![i], factors(n / i)].concat())
}
