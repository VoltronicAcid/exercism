pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let map: [u32; 10] = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9];

        self.to_string()
            .chars()
            .all(|ch| ch.is_numeric() || ch.is_whitespace())
            .then(|| {
                self.to_string()
                    .chars()
                    .filter_map(|ch| ch.to_digit(10))
                    .rev()
                    .fold((0_u32, 0_u32), |(len, sum), digit| match len % 2 == 1 {
                        true => (len + 1, sum + map[digit as usize]),
                        false => (len + 1, sum + digit),
                    })
            })
            .is_some_and(|(len, sum)| len > 1 && sum % 10 == 0)
    }
}
