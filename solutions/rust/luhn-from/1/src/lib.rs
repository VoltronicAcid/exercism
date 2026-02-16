use std::fmt::Display;

pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let map: [u32; 10] = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9];

        self.code
            .chars()
            .all(|ch| ch.is_numeric() || ch.is_whitespace())
            .then(|| {
                self.code
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

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> From<T> for Luhn
where
    T: Display,
{
    fn from(input: T) -> Self {
        Luhn {
            code: input.to_string(),
        }
    }
}
