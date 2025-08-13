pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::new();

    let mut prime_iterator = std::iter::from_fn(|| {
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
    prime_iterator.nth(n).unwrap()
}
