pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        None
    } else {
        let mut n = n;
        let mut count: u64 = 0;

        while n != 1 {
            count += 1;
            n = if n % 2 == 0 { n / 2 } else { n * 3 + 1 };
        }

        Some(count)
    }
}
