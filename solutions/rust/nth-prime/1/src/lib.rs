pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::new();

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

    let n = n as usize;
    for (idx, prime) in prime_iterator.enumerate() {
        if idx == n {
            return prime;
        }
    }

    1
}
