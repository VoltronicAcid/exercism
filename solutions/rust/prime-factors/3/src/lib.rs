pub fn factors(input: u64) -> Vec<u64> {
    let mut prime_factors: Vec<u64> = Vec::new();
    if input < 2 {
        return prime_factors;
    }
    let mut num = input;
    let mut primes: Vec<u64> = vec![];

    let prime_iterator = std::iter::from_fn(move || {
        if primes.is_empty() {
            primes.push(2);
            Some(2)
        } else if primes.len() == 1 {
            primes.push(3);
            Some(3)
        } else {
            let mut test_num = 2 + primes.last().unwrap();
            while primes.iter().any(|prime| test_num % prime == 0) {
                test_num += 2;
            }
            primes.push(test_num);
            Some(test_num)
        }
    });

    let sq_rt = (input as f64).sqrt();
    for prime in prime_iterator {
        if prime as f64 > sq_rt {
            prime_factors.push(num);
            return prime_factors;
        }
        while num % prime == 0 {
            prime_factors.push(prime);
            num /= prime;
        }
        if num == 1 {
            break;
        }
    }

    prime_factors
}
