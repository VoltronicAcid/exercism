pub struct Matcher<T> {
    rule: fn(T) -> bool,
    sub: String,
}

impl<T> Matcher<T> {
    pub fn new(rule: fn(T) -> bool, sub: &str) -> Matcher<T> {
        Matcher {
            rule,
            sub: sub.to_string(),
        }
    }
}

#[derive(Default)]
pub struct Fizzy<T>
where
    T: Default + Copy + ToString,
{
    rules: Vec<Matcher<T>>,
}

impl<T> Fizzy<T>
where
    T: Default + Copy + ToString,
{
    pub fn new() -> Self {
        Fizzy::default()
    }

    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.rules.push(matcher);

        self
    }

    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
    {
        iter.map(move |item| {
            self.rules
                .iter()
                .filter(|&matcher| (matcher.rule)(item))
                .map(|matcher| matcher.sub.clone())
                .fold(None, |acc, sub| match acc {
                    None => Some(sub),
                    Some(prev) => Some(prev + &sub),
                })
                .unwrap_or(item.to_string())
        })
    }
}

pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Default + Copy + ToString + std::ops::Rem<Output = T> + PartialEq + From<u8>,
{
    Fizzy::new()
        .add_matcher(Matcher::new(
            |item: T| item.rem(T::from(3)).eq(&T::default()),
            "fizz",
        ))
        .add_matcher(Matcher::new(
            |item: T| item.rem(T::from(5)).eq(&T::default()),
            "buzz",
        ))
}
