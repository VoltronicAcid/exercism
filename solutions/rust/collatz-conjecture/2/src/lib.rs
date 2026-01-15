pub fn collatz(num: u64) -> Option<u64> {
    match num {
        0 => None,
        1 => Some(0),
        n if num % 2 == 0 => Some(collatz(n / 2).unwrap() + 1),
        n if num % 2 == 1 => Some(collatz(n * 3 + 1).unwrap() + 1),
        _ => unreachable!(),
    }
}
